use std::{borrow::Cow, collections::HashMap, error::Error, sync::Arc};

mod constants;
mod rdfa_elt;
mod structs;
#[cfg(test)]
mod tests;

use constants::{
    BNODE_ID_GENERATOR, COMMON_PREFIXES, NODE_NS_TYPE, NODE_RDFA_PATTERN_TYPE,
    NODE_RDFA_USES_VOCABULARY, RESERVED_KEYWORDS,
};
use itertools::Itertools;
use log::{debug, error};
use rdfa_elt::RdfaElement;
use scraper::{ElementRef, Selector};
use url::Url;
use uuid::Uuid;

use crate::constants::{NODE_RDF_FIRST, NODE_RDF_NIL, NODE_RDF_REST, NODE_RDF_XML_LITERAL};
pub use structs::RdfaGraph;
use structs::{Context, DataTypeFromPattern, Literal, Node, Statement};

impl<'a> RdfaGraph<'a> {
    pub fn parse(
        input: &ElementRef<'a>,
        initial_context: Context<'a>,
    ) -> Result<RdfaGraph<'a>, Box<dyn Error>> {
        let mut triples = vec![];
        traverse_element(input, None, initial_context, &mut triples, &mut vec![])?;

        triples = copy_pattern(triples)?;
        // copy patterns

        Ok(RdfaGraph(triples))
    }
}
pub fn traverse_element<'a, 'b>(
    element_ref: &'b ElementRef<'a>,
    parent: Option<&'b Context<'a>>,
    mut ctx: Context<'a>,
    stmts: &'b mut Vec<Statement<'a>>,
    in_list_stmts: &mut Vec<Statement<'a>>,
) -> Result<Option<Node<'a>>, Box<dyn Error>> {
    let mut elt = RdfaElement::new(element_ref)?;

    ctx.vocab = elt.vocab.or_else(|| parent.as_ref().and_then(|p| p.vocab));

    ctx.base = elt.base.unwrap_or(ctx.base);

    if let Some(vocab) = ctx.vocab.filter(|v| !v.is_empty()) {
        stmts.push(Statement {
            subject: resolve_uri(ctx.base, &ctx, true)?,
            predicate: NODE_RDFA_USES_VOCABULARY.clone(),
            object: resolve_uri(vocab, &ctx, false)?,
        })
    } else {
        ctx.vocab = None;
    }
    ctx.prefixes = elt
        .prefix
        .map(parse_prefixes)
        .or_else(|| parent.map(|p| p.prefixes.clone()))
        .unwrap_or(ctx.prefixes);

    let is_empty_curie = |s: &str| {
        let mut s = s.trim();
        if s.starts_with('[') {
            s = &s[1..];
        }
        if s.ends_with(']') {
            s = &s[0..s.len() - 1];
        }
        s.is_empty()
    };

    let resource = elt.resource.filter(|r| !is_empty_curie(r));

    ctx.lang = elt.lang.or_else(|| parent.and_then(|p| p.lang));

    let about = elt.about.and_then(|a| resolve_uri(a, &ctx, true).ok());

    let mut rels = elt.rel.map(|r| parse_property_or_type_of(r, &ctx, true));
    let mut revs = elt.rev.map(|r| parse_property_or_type_of(r, &ctx, true));

    let mut parent_in_rel = parent.and_then(|c| c.in_rel.clone());
    let mut parent_in_rev = parent.and_then(|c| c.in_rev.clone());
    let mut parent_in_list = parent.and_then(|c| c.in_list.clone());

    let mut src_or_href = elt
        .src_or_href()
        .and_then(|v| resolve_uri(v, &ctx, true).ok());

    let mut type_ofs = elt.type_of.and_then(|t| {
        if t.trim().is_empty() {
            // use vocab
            resolve_uri(ctx.vocab.unwrap_or(ctx.base), &ctx, true)
                .ok()
                .map(|v| vec![v])
        } else {
            Some(parse_property_or_type_of(t, &ctx, true))
        }
    });

    let predicates = elt
        .property
        .map(|p| parse_property_or_type_of(p, &ctx, false));

    let get_parent_subject = |ctx: &Context<'a>| {
        parent
            .and_then(|p| p.current_node.clone())
            .or_else(|| {
                if parent.is_none() {
                    resolve_uri(ctx.base, ctx, true).ok()
                } else {
                    None
                }
            })
            .ok_or("no parent")
    };

    let current_node = if let Some(parent_in_list) = parent_in_list.take() {
        let subject = get_parent_subject(&ctx)?;
        let obj = if let Some(resource) = resource
            .and_then(|r| resolve_uri(r, &ctx, true).ok())
            .map(|n| Node::Ref(Arc::new(n)))
            .or_else(|| src_or_href.clone())
        {
            resource
        } else {
            Node::Ref(Arc::new(extract_literal(&elt, &ctx)?))
        };
        for rel in parent_in_list {
            push_triples_inlist(in_list_stmts, &subject, rel, &obj);
        }
        subject
    } else if elt.is_inlist() {
        let mut in_rel = false;

        let subject = get_parent_subject(&ctx)?;

        if rels.is_some()
            && src_or_href.is_none()
            && predicates.is_none()
            && resource.is_none()
            && about.is_none()
        // empty list
        {
            if element_ref.children().count() != 0 {
                ctx.in_list = rels.take();
            } else {
                push_triples(in_list_stmts, &subject, &rels.take(), &NODE_RDF_NIL);
            }
        }
        if let Some(rels) = rels.take().filter(|r| !r.is_empty()) {
            in_rel = true;
            let obj = if let Some(resource) = resource
                .and_then(|r| resolve_uri(r, &ctx, true).ok())
                .map(|n| Node::Ref(Arc::new(n)))
                .or_else(|| src_or_href.clone())
            {
                resource
            } else {
                Node::Ref(Arc::new(extract_literal(&elt, &ctx)?))
            };
            for rel in rels {
                push_triples_inlist(in_list_stmts, &subject, rel, &obj);
            }
        }
        if let Some(predicates) = predicates {
            let obj = if let (Some(resource), false) = (resource, in_rel) {
                Node::Ref(Arc::new(resolve_uri(resource, &ctx, true)?))
            } else {
                Node::Ref(Arc::new(extract_literal(&elt, &ctx)?))
            };
            for predicate in predicates {
                push_triples_inlist(in_list_stmts, &subject, predicate, &obj);
            }
        }

        subject
    } else if let Some(resource) = resource {
        let object = about
            .as_ref()
            .filter(|_| parent_in_rel.is_some() || parent_in_rev.is_some())
            .map(|a| Node::Ref(Arc::new(a.clone())))
            .unwrap_or(Node::Ref(Arc::new(resolve_uri(resource, &ctx, true)?)));
        let mut curr_node = object;
        let subject = about
            .clone()
            .map(|a| Ok(Node::Ref(Arc::new(a))))
            .unwrap_or_else(|| get_parent_subject(&ctx))?;

        push_triples(stmts, &subject, &predicates, &curr_node);

        if predicates.is_some() && type_ofs.is_none() {
            curr_node = subject;
        } else {
            push_triples(stmts, &subject, &rels.take(), &curr_node);
            push_triples(stmts, &curr_node, &revs.take(), &subject);
        }

        curr_node
    } else if let Some(about) = about {
        // handle about case. set the context.
        // if property is present, children become objects of current.
        let is_empty = elt
            .about
            .filter(|a| !a.trim().is_empty() && is_empty_curie(a))
            .is_some();
        let subject = if !is_empty {
            Node::Ref(Arc::new(about))
        } else {
            resolve_uri(ctx.base, &ctx, true)?
        };

        push_triples(
            stmts,
            &subject,
            &predicates,
            &Node::Ref(Arc::new(extract_literal(&elt, &ctx)?)),
        );

        if let Some(src_or_href) = &src_or_href {
            push_triples(stmts, &subject, &rels, src_or_href);
            push_triples(stmts, src_or_href, &revs, &subject);
        }
        if is_empty {
            make_bnode()
        } else {
            subject
        }
    } else if src_or_href.is_some() && (rels.is_some() || revs.is_some()) {
        let src_or_href = src_or_href.take().ok_or("no src")?;
        let subject = get_parent_subject(&ctx).ok().unwrap_or_else(make_bnode);

        // test 0303, this becomes dumber and dumber
        let mut has_term = false;
        let mut emit_triple = false;
        if elt.has_property() {
            rels = rels.take().map(|rs| {
                rs.into_iter()
                    .filter(|r| {
                        let m = matches!(r, Node::Ref(r) if matches!(r.as_ref(), Node::TermIri(_)));
                        if m {
                            has_term = true;
                        } else {
                            emit_triple = true;
                        }
                        !m
                    })
                    .collect_vec()
            });
            revs = revs.take().map(|rs| {
                rs.into_iter()
                    .filter(|r| {
                        let m = matches!(r, Node::Ref(r) if matches!(r.as_ref(), Node::TermIri(_)));
                        if m {
                            has_term = true;
                        } else {
                            emit_triple = true;
                        }
                        !m
                    })
                    .collect_vec()
            });
        }

        push_triples(stmts, &subject, &rels, &src_or_href);
        push_triples(stmts, &src_or_href, &revs, &subject);

        if has_term {
            if emit_triple {
                elt.src.take();
                elt.href.take();
            }
            push_triples(stmts, &subject, &predicates, &extract_literal(&elt, &ctx)?);
        }
        subject
    } else if elt.has_property()
        && type_ofs.is_some()
        && (parent_in_rel.is_some() || parent_in_rev.is_some())
        && !elt.has_content_or_datatype()
    {
        let subject = make_bnode();
        let node = src_or_href.take().unwrap_or_else(make_bnode);
        for to in type_ofs.take().iter().flatten() {
            push_triples(stmts, &node, &Some(vec![NODE_NS_TYPE.clone()]), to);
        }
        push_triples(stmts, &subject, &predicates, &node);
        subject
    } else if src_or_href.is_some()
        && elt.has_no_content_and_no_datatype()
        && elt.has_property()
        && type_ofs.is_some()
    {
        let src_or_href = src_or_href.take().ok_or("no src")?;

        let subject = get_parent_subject(&ctx).ok().unwrap_or_else(make_bnode);

        push_triples(stmts, &subject, &predicates, &src_or_href);
        src_or_href
    } else if src_or_href.is_some() && elt.has_property() && elt.has_content_or_datatype() {
        let src_or_href = src_or_href.take().ok_or("no src")?;
        push_triples(
            stmts,
            &src_or_href,
            &predicates,
            &extract_literal(&elt, &ctx)?,
        );
        src_or_href
    } else if type_ofs.is_some() && rels.is_some() {
        let base = resolve_uri(ctx.base, &ctx, true)?;
        let b_node = make_bnode();
        for to in type_ofs.take().into_iter().flatten() {
            stmts.push(Statement {
                subject: b_node.clone(),
                predicate: NODE_NS_TYPE.clone(),
                object: to,
            })
        }
        push_triples(stmts, &base, &rels.take(), &b_node);

        b_node
    } else if type_ofs.is_some() {
        let child_with_rdfa_tag = element_ref
            .select(&Selector::parse(
                "[href], [src], [resource], [typeof], [property]",
            )?)
            .count()
            == 0;

        let node = if child_with_rdfa_tag || parent.is_none() {
            resolve_uri(ctx.base, &ctx, true)?
        } else {
            make_bnode()
        };

        let subject = parent
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(make_bnode);

        push_triples(stmts, &subject, &predicates, &node);

        node
    } else {
        let subject = src_or_href
            .clone()
            .filter(|_| parent_in_rel.is_some() || parent_in_rev.is_some())
            .map(Ok)
            .unwrap_or_else(|| get_parent_subject(&ctx))?;
        push_triples(
            stmts,
            &subject,
            &predicates,
            &Node::Ref(Arc::new(extract_literal(&elt, &ctx)?)),
        );

        subject
    };

    if let Some(type_ofs) = type_ofs {
        let sub = src_or_href.unwrap_or_else(|| current_node.clone());
        for type_of in type_ofs {
            stmts.push(Statement {
                subject: sub.clone(),
                predicate: NODE_NS_TYPE.clone(),
                object: type_of,
            })
        }
    }

    if parent_in_rel.is_some() || parent_in_rev.is_some() {
        let parent = get_parent_subject(&ctx)
            .ok()
            .ok_or("in_rel: no parent node")?;
        push_triples(stmts, &parent, &parent_in_rel.take(), &current_node);
        push_triples(stmts, &current_node, &parent_in_rev.take(), &parent);
    }
    ctx.current_node = Some(current_node.clone());
    ctx.in_rel = rels.clone();
    ctx.in_rev = revs.clone();
    for child in get_children(element_ref)? {
        if let Some(c) = ElementRef::wrap(child) {
            // Triples are also 'completed' if any one of @property, @rel or @rev are present.
            let triples_completed = (ctx.in_rel.is_some() || ctx.in_rev.is_some())
                && (c.attr("property").is_some()
                    || c.attr("rel").is_some()
                    || c.attr("rev").is_some())
                && (c.attr("about").is_none() && c.attr("typeof").is_none());

            if triples_completed {
                // Triples are also 'completed' if any one of @property, @rel or @rev are present.
                let b_node = make_bnode();
                push_triples(stmts, &current_node, &ctx.in_rel.take(), &b_node);
                push_triples(stmts, &b_node, &ctx.in_rev.take(), &current_node);

                ctx.current_node = Some(b_node);
            }
            // However, unlike the situation when @about or @typeof are present, all predicates are attached to one bnode
            if c.attr("about").is_some() || c.attr("typeof").is_some() {
                ctx.in_rel = rels.clone();
                ctx.in_rev = revs.clone();
                ctx.current_node = Some(current_node.clone());
            }
            let child_ctx = Context {
                base: ctx.base,
                ..Default::default()
            };

            let node = traverse_element(&c, Some(&ctx), child_ctx, stmts, in_list_stmts)?;
            if node != ctx.current_node {
                stmts.append(in_list_stmts);
            }
        }
    }

    Ok(ctx.current_node.clone())
}
pub fn extract_literal<'a>(
    rdfa_el: &RdfaElement<'a, '_>,
    ctx: &Context<'a>,
) -> Result<Node<'a>, &'static str> {
    let datatype = rdfa_el
        .datatype
        .and_then(|dt| match resolve_uri(dt, ctx, false) {
            Ok(d) => Some(Box::new(d)),
            Err(e) => {
                debug!("could not parse {dt}. error {e}");
                None
            }
        });
    let lang = ctx.lang.filter(|s| datatype.is_none() && !s.is_empty());

    if let Some(value) = rdfa_el.src_or_href().filter(|_| {
        !rdfa_el.has_about() && !rdfa_el.has_property() || rdfa_el.has_no_content_and_no_datatype()
    }) {
        resolve_uri(value, ctx, true)
    } else if let Some(content) = rdfa_el.content {
        Ok(Node::Literal(Literal {
            datatype,
            value: Cow::Borrowed(content),
            lang,
        }))
    } else if datatype
        .as_ref()
        .filter(|dt| dt.as_ref() == &*NODE_RDF_XML_LITERAL)
        .is_some()
    {
        Ok(Node::Literal(Literal {
            value: Cow::Owned(rdfa_el.element_ref.inner_html()),
            datatype,
            lang: None,
        }))
    } else if let Some(content) = rdfa_el.get_time() {
        Ok(Node::Literal(Literal {
            datatype: datatype
                .or_else(|| DataTypeFromPattern::date_time_from_pattern(content).map(Box::new)),
            value: Cow::Borrowed(content),
            lang: None,
        }))
    } else {
        let texts = rdfa_el.texts();
        let text = if texts.is_empty() {
            Cow::Borrowed("")
        } else if texts.len() == 1 {
            let text = {
                if texts[0].lines().filter(|l| !l.trim().is_empty()).count() == 1 {
                    texts[0].trim()
                } else {
                    texts[0]
                }
            };
            Cow::Borrowed(text)
        } else {
            let text = texts.iter().map(|t| t.to_string()).join("");
            Cow::Owned(text)
        };
        Ok(Node::Literal(Literal {
            datatype,
            value: text,
            lang,
        }))
    }
}

pub fn resolve_uri<'a>(
    uri: &'a str,
    ctx: &Context<'a>,
    is_resource: bool,
) -> Result<Node<'a>, &'static str> {
    let iri = Url::parse(uri);

    match iri {
        Ok(iri) if !iri.cannot_be_a_base() || iri.is_special() => Ok(Node::Iri(Cow::Borrowed(uri))),

        // Curie
        Ok(iri) => {
            if uri.starts_with("mail") || uri.starts_with("tel") {
                Ok(Node::Iri(Cow::Borrowed(uri)))
            } else if let Some(value) = ctx.prefixes.get(iri.scheme()) {
                let iri = uri
                    .replacen(':', "", 1)
                    .trim()
                    .replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else if let Some(value) = COMMON_PREFIXES.get(iri.scheme()) {
                let iri = uri
                    .replacen(':', "", 1)
                    .trim()
                    .replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else {
                Ok(Node::Iri(Cow::Owned(uri.to_string())))
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if let Ok((prefix, reference)) = parse_safe_curie(uri) {
                let prefix = prefix.trim();
                if prefix == "_" {
                    let uuid = if cfg!(test) {
                        Uuid::nil()
                    } else {
                        Uuid::new_v4()
                    };
                    return Ok(Node::RefBNode((reference.trim(), uuid)));
                } else if prefix.is_empty() && !reference.is_empty() {
                    return Ok(Node::TermIri(Cow::Owned(
                        [COMMON_PREFIXES[""], reference].join(""),
                    )));
                } else if let Some(prefix) = ctx
                    .prefixes
                    .get(prefix)
                    .or_else(|| COMMON_PREFIXES.get(prefix))
                {
                    let reference = if reference.trim().is_empty() {
                        reference.trim()
                    } else {
                        reference
                    };
                    return Ok(Node::Iri(Cow::Owned([prefix, reference].join(""))));
                }
            }
            if is_resource || uri.starts_with('#') || uri.starts_with('/') {
                let trailing_white_space = if ctx.base.ends_with('/')
                    || ctx.base.ends_with('#')
                    || uri.starts_with('/')
                    || uri.starts_with('#')
                {
                    ""
                } else {
                    "/"
                };
                Ok(Node::TermIri(Cow::Owned(
                    [ctx.base, trailing_white_space, uri].join(""),
                )))
            } else if let Some(vocab) = ctx.vocab {
                Ok(Node::TermIri(Cow::Owned([vocab, uri].join(""))))
            } else if RESERVED_KEYWORDS
                .iter()
                .any(|w| uri.eq_ignore_ascii_case(w))
            {
                Ok(Node::TermIri(Cow::Borrowed(
                    COMMON_PREFIXES[uri.to_lowercase().as_str()],
                )))
            } else {
                debug!("could not determine base/vocab {:?}", ctx);
                // Ok(Node::Iri(Cow::Borrowed(uri)))
                Err("could not determine uri")
            }
        }
        Err(e) => {
            eprintln!("invalid uri {uri}. error: {e}");
            Err("could not resolve uri")
        }
    }
}

fn parse_safe_curie(s: &str) -> Result<(&str, &str), &'static str> {
    let mut s = s.trim();
    if s.starts_with('[') {
        if !s.ends_with(']') {
            return Err("invalid SafeCurie");
        }
        s = &s[1..s.len() - 1];
    }
    s.split_once(':').ok_or("not a curie")
}

fn parse_prefixes(s: &str) -> HashMap<&str, &str> {
    s.split_whitespace()
        .map(|s| s.trim())
        .tuples::<(_, _)>()
        .filter_map(|(s, p)| {
            if let Ok((s, _)) = parse_safe_curie(s) {
                Some((s, p))
            } else {
                error!("fixme! couldn't parse curie for {s}, {p}");
                None
            }
        })
        .collect()
}

fn parse_property_or_type_of<'a>(
    s: &'a str,
    ctx: &Context<'a>,
    allow_b_node: bool,
) -> Vec<Node<'a>> {
    s.split_whitespace()
        .filter_map(|uri| resolve_uri(uri, ctx, false).ok())
        .filter(|node| allow_b_node || !matches!(node, Node::BNode(_) | Node::RefBNode(_)))
        .map(|n| Node::Ref(Arc::new(n)))
        .collect_vec()
}

fn push_triples_inlist<'a>(
    stmts: &mut Vec<Statement<'a>>,
    subject: &Node<'a>,
    predicate: Node<'a>,
    obj: &Node<'a>,
) {
    let b_node = make_bnode();
    stmts.push(Statement {
        subject: b_node.clone(),
        predicate: NODE_RDF_FIRST.clone(),
        object: obj.clone(),
    });

    if let Some(node) =
        find_pos_last_node_in_inlist(stmts, subject, &predicate).and_then(|pos| stmts.get_mut(pos))
    {
        node.object = b_node.clone();
    } else {
        // push the root of the list
        stmts.push(Statement {
            subject: subject.clone(),
            predicate,
            object: b_node.clone(),
        });
    }
    stmts.push(Statement {
        subject: b_node,
        predicate: NODE_RDF_REST.clone(),
        object: NODE_RDF_NIL.clone(),
    });
}
fn find_pos_last_node_in_inlist<'a>(
    stmts: &Vec<Statement<'a>>,
    root_subject: &Node<'a>,
    predicate: &Node<'a>,
) -> Option<usize> {
    fn find_res_nil<'a>(stmts: &Vec<Statement<'a>>, subject: &Node<'a>) -> Option<usize> {
        let node = stmts
            .iter()
            .enumerate()
            .find(|(_, stmt)| &stmt.subject == subject && stmt.predicate == *NODE_RDF_REST);

        if let Some((pos, stmt)) = node {
            if stmt.object == *NODE_RDF_NIL {
                Some(pos)
            } else {
                find_res_nil(stmts, &stmt.object)
            }
        } else {
            None
        }
    }
    let root = stmts
        .iter()
        .find(|stmt| &stmt.subject == root_subject && &stmt.predicate == predicate);
    if let Some(Statement { object, .. }) = root {
        find_res_nil(stmts, object)
    } else {
        None
    }
}

// skip when there are no rdfa attributes, see e.g examples/earl_html5/example0084.html
#[inline]
fn get_children<'a>(
    element_ref: &ElementRef<'a>,
) -> Result<Vec<ego_tree::NodeRef<'a, scraper::Node>>, &'static str> {
    let mut res = vec![];
    for c in element_ref.children() {
        if c.value()
            .as_element()
            .filter(|e| e.attrs().count() == 0)
            .is_some()
        {
            let child_ref = ElementRef::wrap(c).ok_or("not an element ref")?;
            res.append(&mut get_children(&child_ref)?);
        } else {
            res.push(c);
        }
    }

    Ok(res)
}

#[inline]
fn make_bnode<'a>() -> Node<'a> {
    Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
}

#[inline]
pub fn copy_pattern(triples: Vec<Statement<'_>>) -> Result<Vec<Statement<'_>>, Box<dyn Error>> {
    let (pattern_type, pattern): (Vec<Statement>, Vec<Statement>) = triples
        .into_iter()
        .partition(|stmt| stmt.object == *NODE_RDFA_PATTERN_TYPE);

    let (pattern_predicate, pattern): (Vec<Statement>, Vec<Statement>) = pattern
        .into_iter()
        .partition(|stmt| pattern_type.iter().any(|s| s.subject == stmt.subject));

    let (pattern_subject, mut triples): (Vec<Statement>, Vec<Statement>) = pattern
        .into_iter()
        .partition(|stmt| pattern_predicate.iter().any(|s| s.subject == stmt.object));

    // remove only if pattern referenced
    let (mut unreferenced_pattern_predicate, pattern_predicate): (Vec<Statement>, Vec<Statement>) =
        pattern_predicate
            .into_iter()
            .partition(|stmt| pattern_subject.iter().all(|s| s.object != stmt.subject));

    let (mut unreferenced_pattern_type, _): (Vec<Statement>, Vec<Statement>) =
        pattern_type.into_iter().partition(|stmt| {
            unreferenced_pattern_predicate
                .iter()
                .any(|s| s.subject == stmt.subject)
        });
    triples.append(&mut unreferenced_pattern_predicate);
    triples.append(&mut unreferenced_pattern_type);

    for Statement {
        subject, object, ..
    } in pattern_subject
    {
        for Statement {
            predicate,
            object: obj,
            ..
        } in pattern_predicate
            .iter()
            .filter(|stmt| object == stmt.subject)
        {
            triples.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: obj.clone(),
            })
        }
    }

    Ok(triples)
}

#[inline]
fn push_triples<'a>(
    stmts: &mut Vec<Statement<'a>>,
    subject: &Node<'a>,
    predicates: &Option<Vec<Node<'a>>>,
    object: &Node<'a>,
) {
    if let Some(predicate) = predicates {
        for predicate in predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
            });
        }
    }
}
