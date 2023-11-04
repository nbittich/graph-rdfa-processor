use std::error::Error;

use scraper::{node::Element, ElementRef, Selector};

#[derive(Debug, Clone, Copy)]
pub struct RdfaElement<'a, 'b> {
    pub element_ref: &'b ElementRef<'a>,
    pub element: &'a Element,
    pub base: Option<&'a str>,
    pub vocab: Option<&'a str>,
    pub prefix: Option<&'a str>,
    pub lang: Option<&'a str>,
    pub about: Option<&'a str>,
    pub property: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub rev: Option<&'a str>,
    pub src: Option<&'a str>,
    pub href: Option<&'a str>,
    pub type_of: Option<&'a str>,
    pub inlist: Option<&'a str>,
    pub content: Option<&'a str>,
    pub datatype: Option<&'a str>,
    pub datetime: Option<&'a str>,
    pub resource: Option<&'a str>,
}

#[allow(unused)]
impl<'a, 'b> RdfaElement<'a, 'b> {
    pub fn new(element_ref: &'b ElementRef<'a>) -> Result<Self, Box<dyn Error>> {
        let element = element_ref.value();
        let vocab = element.attr("vocab").map(|v| v.trim());
        let base = element_ref
            .select(&Selector::parse("base")?)
            .next()
            .and_then(|e| e.attr("href"))
            .map(|b| {
                let pos_fragment = b.chars().position(|p| p == '#').unwrap_or(b.len());
                &b[0..pos_fragment]
            });
        let prefix = element.attr("prefix");
        let resource = element.attr("resource");
        let lang = element.attr("lang").or_else(|| element.attr("xml:lang"));
        let property = element.attr("property");
        let rel = element.attr("rel");
        let rev = element.attr("rev");
        let type_of = element.attr("typeof");
        let src = element.attr("src");
        let href = element.attr("href");
        let datatype = element.attr("datatype");
        let inlist = element.attr("inlist");
        let content = element.attr("content");
        let about = element.attr("about");
        let datetime = element.attr("datetime");

        Ok(Self {
            element_ref,
            element,
            base,
            vocab,
            prefix,
            lang,
            about,
            property,
            rel,
            rev,
            src,
            href,
            type_of,
            inlist,
            content,
            datatype,
            datetime,
            resource,
        })
    }

    pub fn has_no_rel_and_no_property(&self) -> bool {
        self.rel.is_none() && self.property.is_none()
    }
    pub fn src_or_href(&self) -> Option<&'a str> {
        self.src.or(self.href)
    }
    pub fn is_inlist(&self) -> bool {
        self.inlist.is_some()
    }
    pub fn has_no_content_and_no_datatype(&self) -> bool {
        self.content.is_none() && self.datatype.is_none()
    }

    pub fn has_content_or_datatype(&self) -> bool {
        self.content.is_some() || self.datatype.is_some()
    }
    pub fn has_property(&self) -> bool {
        self.property.is_some()
    }

    pub fn has_about(&self) -> bool {
        self.about.is_some()
    }
    pub fn get_time(&self) -> Option<&'a str> {
        if self.element.name() == "time" || self.datetime.is_some() {
            self.datetime
                .or_else(|| self.element_ref.text().take(1).last())
        } else {
            None
        }
    }
    pub fn texts(&self) -> Vec<&'a str> {
        self.element_ref
            .text()
            .filter(|t| !t.trim().is_empty())
            .collect()
    }

    pub(crate) fn has_rel_or_rev(&self) -> bool {
        self.rel.is_some() || self.rev.is_some()
    }
}
