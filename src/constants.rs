use std::collections::HashMap;
pub static DEFAULT_WELL_KNOWN_PREFIX: &str = "http://data.lblod.info/.well-known/genid#";
pub static RDFA_COPY_PREDICATE: &str = "http://www.w3.org/ns/rdfa#copy";
pub static RDFA_PATTERN_TYPE: &str = "http://www.w3.org/ns/rdfa#Pattern";
pub static NS_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
lazy_static::lazy_static! {
   pub static  ref PREFIXES: HashMap<&'static str, &'static str> =
        HashMap::from([
            ("", "http://www.w3.org/1999/xhtml/vocab#"),
            // w3c
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
            // non-rec w3c
            ("sd","http://www.w3.org/ns/sparql-service-description#"),
            ("org","http://www.w3.org/ns/org#"),
            ("gldp","http://www.w3.org/ns/people#"),
            ("cnt","http://www.w3.org/2008/content#"),
            ("dcat","http://www.w3.org/ns/dcat#"),
            ("earl","http://www.w3.org/ns/earl#"),
            ("ht","http://www.w3.org/2006/http#"),
            ("ptr","http://www.w3.org/2009/pointers#"),
            // widely used
            ("cc","http://creativecommons.org/ns#"),
            ("ctag","http://commontag.org/ns#"),
            ("dc","http://purl.org/dc/terms/"),
            ("dcterms","http://purl.org/dc/terms/"),
            ("foaf","http://xmlns.com/foaf/0.1/"),
            ("gr","http://purl.org/goodrelations/v1#"),
            ("ical","http://www.w3.org/2002/12/cal/icaltzd#"),
            ("og","http://ogp.me/ns#"),
            ("rev","http://purl.org/stuff/rev#"),
            ("sioc","http://rdfs.org/sioc/ns#"),
            ("v","http://rdf.data-vocabulary.org/#"),
            ("vcard","http://www.w3.org/2006/vcard/ns#"),
            ("schema","http://schema.org/"),
            // terms
            ("describedby","http://www.w3.org/2007/05/powder-s#describedby"),
            ("license","http://www.w3.org/1999/xhtml/vocab#license"),
            ("role","http://www.w3.org/1999/xhtml/vocab#role"),


        ]);
}
