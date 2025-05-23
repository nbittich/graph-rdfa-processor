use std::{borrow::Cow, collections::HashMap};

use crate::{Node, structs::DataTypeFromPattern};

#[cfg(test)]
static FAKE_UUID_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(test)]
pub(crate) fn reset_fake_uuid_gen() {
    FAKE_UUID_GEN.store(0, std::sync::atomic::Ordering::SeqCst);
}
#[cfg(not(test))]
pub fn get_uuid() -> String {
    uuid::Uuid::now_v7().to_string().replace("-", "")
}
#[cfg(test)]
pub fn get_uuid() -> String {
    FAKE_UUID_GEN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!(
        "{}",
        FAKE_UUID_GEN.load(std::sync::atomic::Ordering::SeqCst)
    )
}
// pub static DEFAULT_WELL_KNOWN_PREFIX: &str = "http://data.lblod.info/.well-known/genid#";
pub static RDFA_COPY_PREDICATE: &str = "http://www.w3.org/ns/rdfa#copy";
pub static RDFA_PATTERN_TYPE: &str = "http://www.w3.org/ns/rdfa#Pattern";
pub static RDFA_USES_VOCABULARY: &str = "http://www.w3.org/ns/rdfa#usesVocabulary";
pub static RDF_XML_LITERAL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral";
pub static RDF_HTML_LITERAL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML";
pub static RDF_PLAIN_LITERAL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral";
pub static RDF_XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
pub static NS_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
pub static RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
pub static RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
pub static RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";

pub static RESERVED_KEYWORDS: [&str; 3] = ["license", "describedby", "role"];

pub static DATETIME_TYPES: [&DataTypeFromPattern; 6] = [
    &DataTypeFromPattern {
        pattern: "-?P(?:[0-9]+Y)?(?:[0-9]+M)?(?:[0-9]+D)?(?:T(?:[0-9]+H)?(?:[0-9]+M)?(?:[0-9]+(?:.[0-9]+)?S)?)?",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#duration"),
    },
    &DataTypeFromPattern {
        pattern: r"-?(?:[1-9][0-9][0-9][0-9]|0[1-9][0-9][0-9]|00[1-9][0-9]|000[1-9])-[0-9][0-9]-[0-9][0-9]T(?:[0-1][0-9]|2[0-4]):[0-5][0-9]:[0-5][0-9](?:\.[0-9]+)?(?:Z|[+\-][0-9][0-9]:[0-9][0-9])?",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#dateTime"),
    },
    &DataTypeFromPattern {
        pattern: "-?(?:[1-9][0-9][0-9][0-9]|0[1-9][0-9][0-9]|00[1-9][0-9]|000[1-9])-[0-9][0-9]-[0-9][0-9](?:Z|[+-][0-9][0-9]:[0-9][0-9])?",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#date"),
    },
    &DataTypeFromPattern {
        pattern: "(?:[0-1][0-9]|2[0-4]):[0-5][0-9]:[0-5][0-9](?:.[0-9]+)?(?:Z|[+-][0-9][0-9]:[0-9][0-9])?",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#time"),
    },
    &DataTypeFromPattern {
        pattern: "-?(?:[1-9][0-9][0-9][0-9]|0[1-9][0-9][0-9]|00[1-9][0-9]|000[1-9])-[0-9][0-9]",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#gYearMonth"),
    },
    &DataTypeFromPattern {
        pattern: "-?[1-9][0-9][0-9][0-9]|0[1-9][0-9][0-9]|00[1-9][0-9]|000[1-9]",
        datatype: crate::iri!("http://www.w3.org/2001/XMLSchema#gYear"),
    },
];

lazy_static::lazy_static! {
    pub static ref NODE_RDF_XML_LITERAL: Node<'static> = Node::Iri(Cow::Borrowed(RDF_XML_LITERAL));
    pub static ref NODE_RDF_PLAIN_LITERAL: Node<'static> = Node::Iri(Cow::Borrowed(RDF_PLAIN_LITERAL));
    pub static ref NODE_RDF_HTML_LITERAL: Node<'static> = Node::Iri(Cow::Borrowed(RDF_HTML_LITERAL));
    pub static ref NODE_RDF_FIRST: Node<'static> = Node::Iri(Cow::Borrowed(RDF_FIRST));
    pub static ref NODE_RDF_REST: Node<'static> = Node::Iri(Cow::Borrowed(RDF_REST));
    pub static ref NODE_RDF_NIL: Node<'static> = Node::Iri(Cow::Borrowed(RDF_NIL));
    pub static ref NODE_RDFA_USES_VOCABULARY: Node<'static> = Node::Iri(Cow::Borrowed(RDFA_USES_VOCABULARY));
    pub static ref NODE_RDF_XSD_STRING: Node<'static> = Node::Iri(Cow::Borrowed(RDF_XSD_STRING));
    pub static ref NODE_RDFA_PATTERN_TYPE: Node<'static> = Node::Iri(Cow::Borrowed(RDFA_PATTERN_TYPE));
    pub static ref NODE_RDFA_COPY_PREDICATE: Node<'static> = Node::Iri(Cow::Borrowed(RDFA_COPY_PREDICATE));
    pub static ref NODE_NS_TYPE: Node<'static>=Node::Iri(Cow::Borrowed(NS_TYPE));
    pub static  ref COMMON_PREFIXES: HashMap<&'static str, &'static str> =
        HashMap::from([
            ("", "http://www.w3.org/1999/xhtml/vocab#"),
            ("gradl", "http://www.w3.org/2003/g/data-view#"),
            ("ma","http://www.w3.org/ns/ma-ont#"),
            ("owl","http://www.w3.org/2002/07/owl#"),
            ("rdf","http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfa","http://www.w3.org/ns/rdfa#"),
            ("rdfs","http://www.w3.org/2000/01/rdf-schema#"),
            ("rif","http://www.w3.org/2007/rif#"),
            ("skos","http://www.w3.org/2004/02/skos/core#"),
            ("skosxl","http://www.w3.org/2008/05/skos-xl#"),
            ("wdr","http://www.w3.org/2007/05/powder#"),
            ("void","http://rdfs.org/ns/void#"),
            ("wdrs","http://www.w3.org/2007/05/powder-s#"),
            ("xhv","http://www.w3.org/1999/xhtml/vocab#"),
            ("xml","http://www.w3.org/XML/1998/namespace"),
            ("xsd","http://www.w3.org/2001/XMLSchema#"),
            ("prov","http://www.w3.org/ns/prov#"),
            ("rr","http://www.w3.org/ns/r2rml#"),
            ("sd","http://www.w3.org/ns/sparql-service-description#"),
            ("org","http://www.w3.org/ns/org#"),
            ("gldp","http://www.w3.org/ns/people#"),
            ("cnt","http://www.w3.org/2008/content#"),
            ("dcat","http://www.w3.org/ns/dcat#"),
            ("earl","http://www.w3.org/ns/earl#"),
            ("ht","http://www.w3.org/2006/http#"),
            ("ptr","http://www.w3.org/2009/pointers#"),
            ("cc","http://creativecommons.org/ns#"),
            ("ctag","http://commontag.org/ns#"),
            ("dc","http://purl.org/dc/terms/"),
            ("dcterms","http://purl.org/dc/terms/"),
            ("foaf","http://xmlns.com/foaf/0.1/"),
            ("gr","http://purl.org/goodrelations/v1#"),
            ("ical","http://www.w3.org/2002/12/cal/icaltzd#"),
            ("og","http://ogp.me/ns#"),
            ("qb", "http://purl.org/linked-data/cube#"),
            ("csvw", "http://www.w3.org/ns/csvw#"),
            ("rev","http://purl.org/stuff/rev#"),
            ("grddl", "http://www.w3.org/2003/g/data-view#"),
            ("sioc","http://rdfs.org/sioc/ns#"),
            ("v","http://rdf.data-vocabulary.org/#"),
            ("vcard","http://www.w3.org/2006/vcard/ns#"),
            ("schema","http://schema.org/"),
            ("describedby","http://www.w3.org/2007/05/powder-s#describedby"),
            ("license","http://www.w3.org/1999/xhtml/vocab#license"),
            ("role","http://www.w3.org/1999/xhtml/vocab#role"),


        ]);
}

pub static IS_SPECIAL_NODE_FN: fn(&Option<Box<Node<'_>>>) -> bool =
    |datatype: &Option<Box<Node<'_>>>| {
        datatype
            .as_ref()
            .filter(|dt| {
                dt.as_ref() == &*NODE_RDF_HTML_LITERAL
                    || dt.as_ref() == &*NODE_RDF_XML_LITERAL
                    || dt.as_ref() == &*NODE_RDF_PLAIN_LITERAL
            })
            .is_some()
    };
