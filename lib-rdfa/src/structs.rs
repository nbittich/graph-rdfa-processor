use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    sync::Arc,
};

use regex::Regex;
use uuid::Uuid;

use crate::constants::{
    BNODE_ID_GENERATOR, DATETIME_TYPES, DEFAULT_WELL_KNOWN_PREFIX, NODE_RDF_XSD_STRING,
};
#[macro_export]
macro_rules! iri {
    ($name:literal) => {
        Node::Iri(Cow::Borrowed($name))
    };
}

#[derive(Debug)]
pub struct RdfaGraph<'a> {
    pub well_known_prefix: Option<&'a str>,
    pub statements: HashSet<Statement<'a>>,
}

#[derive(Debug, Default, Clone)]
pub struct Context<'a> {
    pub base: &'a str,
    pub well_known_prefix: Option<&'a str>,
    pub vocab: Option<&'a str>,
    pub lang: Option<&'a str>,
    pub in_rel: Option<Vec<Node<'a>>>,
    pub in_rev: Option<Vec<Node<'a>>>,
    pub in_list: Option<Vec<Node<'a>>>,
    pub current_node: Option<Node<'a>>,
    pub prefixes: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub struct DataTypeFromPattern<'a> {
    pub pattern: &'a str,
    pub datatype: Node<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal<'a> {
    pub datatype: Option<Box<Node<'a>>>,
    pub value: Cow<'a, str>,
    pub lang: Option<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Hash)]
pub enum Node<'a> {
    Iri(Cow<'a, str>),
    TermIri(Cow<'a, str>),
    Literal(Literal<'a>),
    Ref(Arc<Node<'a>>),
    Blank(u64),
    RefBlank((&'a str, Uuid)),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statement<'a> {
    pub subject: Node<'a>,
    pub predicate: Node<'a>,
    pub object: Node<'a>,
}

impl Statement<'_> {
    fn as_ntriple_string(&self, well_known_prefix: Option<&str>) -> String {
        let Statement {
            subject,
            predicate,
            object,
        } = self;
        format!(
            r#"{} {} {}."#,
            subject.as_ntriple_string(well_known_prefix),
            predicate.as_ntriple_string(well_known_prefix),
            object.as_ntriple_string(well_known_prefix)
        )
    }
}

impl Node<'_> {
    pub fn is_empty(&self) -> bool {
        match self {
            Node::Iri(iri) => iri.is_empty(),
            Node::TermIri(iri) => iri.is_empty(),
            Node::Literal(l) => {
                l.value.is_empty()
                    && l.datatype.as_ref().filter(|li| !li.is_empty()).is_none()
                    && l.lang.filter(|lan| lan.is_empty()).is_none()
            }
            Node::Ref(r) => r.is_empty(),
            Node::Blank(_) => false,
            Node::RefBlank((s, _)) => s.is_empty(),
        }
    }
    fn as_ntriple_string(&self, well_known_prefix: Option<&str>) -> String {
        match self {
            Node::Iri(iri) | Node::TermIri(iri) => format!("<{}>", iri),
            Node::Ref(iri) => iri.as_ntriple_string(well_known_prefix),
            Node::Literal(Literal {
                datatype,
                lang,
                value,
            }) => {
                let mut s = if value
                    .as_ref()
                    .chars()
                    .any(|c| c.is_ascii_control() || c.is_control())
                {
                    format!(r#""""{value}""""#)
                } else {
                    format!(r#""{value}""#)
                };

                if let Some(datatype) = datatype
                    .as_ref()
                    .filter(|dt| dt.as_ref() != &*NODE_RDF_XSD_STRING)
                {
                    s.push_str(&format!(
                        r#"^^{}"#,
                        datatype.as_ntriple_string(well_known_prefix)
                    ));
                } else if let Some(lang) = lang {
                    s.push_str(&format!(r#"@{lang}"#));
                }
                s
            }
            Node::Blank(id) => {
                // todo maybe this should use the base?
                format!(
                    "<{}{}>",
                    well_known_prefix.unwrap_or(DEFAULT_WELL_KNOWN_PREFIX),
                    id
                )
            }
            Node::RefBlank((id, uuid)) => {
                if let Ok(v) = id.parse::<u64>() {
                    if v <= BNODE_ID_GENERATOR.load(std::sync::atomic::Ordering::SeqCst) {
                        format!(
                            "<{}{}>",
                            well_known_prefix.unwrap_or(DEFAULT_WELL_KNOWN_PREFIX),
                            uuid
                        )
                    } else {
                        format!(
                            "<{}{}>",
                            well_known_prefix.unwrap_or(DEFAULT_WELL_KNOWN_PREFIX),
                            id
                        )
                    }
                } else if id.is_empty() {
                    format!(
                        "<{}{}>",
                        well_known_prefix.unwrap_or(DEFAULT_WELL_KNOWN_PREFIX),
                        uuid
                    )
                } else {
                    format!(
                        "<{}{}>",
                        well_known_prefix.unwrap_or(DEFAULT_WELL_KNOWN_PREFIX),
                        id
                    )
                }
            }
        }
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Iri(l0), Self::Iri(r0)) => l0 == r0,
            (Self::Iri(l0), Self::TermIri(r0)) => l0 == r0,
            (Self::TermIri(l0), Self::TermIri(r0)) => l0 == r0,
            (Self::TermIri(l0), Self::Iri(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            (Self::Ref(l0), rhs) => l0.as_ref() == rhs,
            (lhs, Self::Ref(r0)) => lhs == r0.as_ref(),
            (Self::Blank(l0), Self::Blank(r0)) => l0 == r0,
            (Self::RefBlank(l0), Self::RefBlank(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Display for RdfaGraph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .statements
                .iter()
                .map(|s| s.as_ntriple_string(self.well_known_prefix))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
#[allow(unused)]
impl<'a> DataTypeFromPattern<'a> {
    pub fn test(&self, value: &'a str) -> Option<Node<'a>> {
        let re = Regex::new(self.pattern).ok()?;
        if re.find(value).filter(|r| r.len() == value.len()).is_some() {
            Some(self.datatype.clone())
        } else {
            None
        }
    }
    pub fn date_time_from_pattern(value: &'a str) -> Option<Node<'a>> {
        for dtp in DATETIME_TYPES {
            if let v @ Some(_) = dtp.test(value) {
                return v;
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::DataTypeFromPattern;

    use crate::Cow;
    use crate::Node;

    #[test]
    fn test_date() {
        let res = DataTypeFromPattern::date_time_from_pattern("2022-09-10");
        assert_eq!(Some(iri!("http://www.w3.org/2001/XMLSchema#date")), res);

        let res = DataTypeFromPattern::date_time_from_pattern("00:00:00");
        assert_eq!(Some(iri!("http://www.w3.org/2001/XMLSchema#time")), res);
        let res = DataTypeFromPattern::date_time_from_pattern("2012-03-18T00:00:00Z");
        assert_eq!(Some(iri!("http://www.w3.org/2001/XMLSchema#dateTime")), res);

        let res = DataTypeFromPattern::date_time_from_pattern("2022");
        assert_eq!(Some(iri!("http://www.w3.org/2001/XMLSchema#gYear")), res);

        let res = DataTypeFromPattern::date_time_from_pattern("2022-09");
        assert_eq!(
            Some(iri!("http://www.w3.org/2001/XMLSchema#gYearMonth")),
            res
        );

        let res = DataTypeFromPattern::date_time_from_pattern("PT2H30M45.5S");
        assert_eq!(Some(iri!("http://www.w3.org/2001/XMLSchema#duration")), res);
    }
}
