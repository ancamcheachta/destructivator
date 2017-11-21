/// Formats `<members/>` tags
macro_rules! format_members {
    ( $($arg:tt)* ) => (format!("<members>{}</members>", $($arg)*));
}

/// Formats `<types/>` tags and their descendents
macro_rules! format_types {
    ( $($arg:tt)* ) => (format!(r"<types>
        {}
        <name>{}</name>
    </types>", $($arg)*));
}

/// Formats `<package/>` tags and its descendents
macro_rules! format_package {
    ( $($arg:tt)* ) => (format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
    {}
    <version>{}</version>
</Package>
"#, $($arg)*));
}

const DEFAULT_API_VERSION: &'static str = "39.0";

#[derive(Debug)]
pub struct Members(String);

#[derive(Clone, Debug)]
pub struct Package {
    pub types: Vec<Types>,
    version: String,
}

#[derive(Clone, Debug)]
pub struct Types {
    pub members: Vec<Members>,
    pub name: String,
}

pub trait ToXML {
    fn to_xml(&self) -> String;
}

impl Members {
    pub fn from(member: String) -> Self {
        Members(member)
    }
}

impl Package {
    pub fn new(version: Option<&str>) -> Self {
        Package {
            types: Vec::new(),
            version: match version {
                Some(v) => v.to_string(),
                None => DEFAULT_API_VERSION.to_string(),
            },
        }
    }
}

impl Types {
    pub fn new(name: &str) -> Self {
        Types {
            name: name.to_string(),
            members: Vec::new(),
        }
    }
}

impl Clone for Members {
    fn clone(&self) -> Self {
        let Members(ref m) = *self;
        Members(m.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        let Members(ref s) = *source;
        let Members(ref mut m) = *self;
        m.clone_from(s);
    }
}

impl ToXML for Members {
    fn to_xml(&self) -> String {
        let Members(ref m) = *self;
        format_members!(&m)
    }
}

impl ToXML for Package {
    fn to_xml(&self) -> String {
        format_package!(&self.types.to_xml(), &self.version)
    }
}

impl ToXML for Types {
    fn to_xml(&self) -> String {
        format_types!(&self.members.to_xml(), &self.name)
    }
}

impl ToXML for Vec<Members> {
    fn to_xml(&self) -> String {
        let members: Vec<String> = self.iter()
            .map(|members| members.to_xml())
            .collect();
        
        members.join("\n")
    }
}

impl ToXML for Vec<Types> {
    fn to_xml(&self) -> String {
        let types: Vec<String> = self.iter()
            .map(|types| types.to_xml())
            .collect();
        
        types.join("\n")
    }
}