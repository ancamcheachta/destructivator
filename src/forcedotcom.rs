use git2::DiffDelta;
use std::collections::HashMap;

const DEFAULT_API_VERSION: &'static str = "39.0";

/// Formats `<members/>` tags
macro_rules! format_members {
    ( $($arg:tt)* ) => (format!("        <members>{}</members>", $($arg)*));
}

/// Formats `<types/>` tags and their descendents
macro_rules! format_types {
    ( $($arg:tt)* ) => (format!(r"    <types>
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

pub trait ToPackage {
    /// Converts to a `Package` struct.
    fn to_package(&self, type_mapping_file: &HashMap<String,String>) -> Package;
}

pub trait ToXML {
    /// Converts to a `String` of XML.
    fn to_xml(&self) -> String;
}

impl Members {
    /// Creates `Members` struct from `String`.
    pub fn from(member: String) -> Self {
        Members(member)
    }
}

impl Package {
    /// Creates a new `Package` struct.  Optionally allows a Salesforce API
    /// version to be set using `version`.  If this is `None`, the module
    /// `DEFAULT_API_VERSION` version value is used.
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
    /// Creates a new `Types` struct with the `name` provided.
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

impl<'a> ToPackage for Vec<DiffDelta<'a>> {
    /// Converts a vector of diff deltas to a `Package` struct. In this case, 
    /// `type_mapping` is a `HashMap<String,String>` where key is the component 
    /// folder name in all lowercase, and the value is the component name
    /// itself.
    ///
    /// **Note**: Right now there is no support for components (such as
    /// `StandardObject` and `CustomObject`) which have a many-to-one
    /// relationship to their parent folder (ie. `objects`).
    fn to_package(&self, type_mapping: &HashMap<String,String>) -> Package {
        let mut package = Package::new(None);
        let mut types_hash_map: HashMap<String, Types> = HashMap::new();
        for delta in self.iter() {
            let (mut md_folder, mut component): (String, String) = (Default::default(), Default::default());
            let new_file_path = &delta.new_file().path().unwrap().to_str().unwrap();
            match new_file_path.rsplit("/").nth(1) {
                None => { }, // println!("warning: {} not in a type folder. Consider adding to {}", new_file_path, IGNORE_FILE),
                Some(m) => md_folder.push_str(&m.to_lowercase()),
            };
            if md_folder.len() > 0 {
                match new_file_path.rsplit("/").nth(0) {
                    None => component.push_str(&new_file_path),
                    Some(f) => component.push_str(&f.rsplit(".")
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .iter()
                        .rev()
                        .map(|x| x.to_owned())
                        .collect::<Vec<&str>>()
                        .join(".")
                    ),
                };

                if component.len() > 0 && type_mapping.contains_key(&md_folder) {
                    let md_type: String = type_mapping.get(&md_folder).unwrap().to_owned();
                    let members = Members::from(component);
                    
                    if !types_hash_map.contains_key(&md_type) {
                        types_hash_map.insert(md_type.clone(), Types::new(&md_type));
                    }
                    
                    match types_hash_map.get_mut(&md_type) {
                        Some(ref mut types) => types.members.push(members),
                        None => {},
                    }
                }
            }
        }

        for key in types_hash_map.keys() {
            if let Some(types) = types_hash_map.get(key) {
                package.types.push(types.clone());
            }
        }
        
        package
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

#[cfg(test)]
mod forcedotcom_test {
    use super::*;
    fn package_xml_w_def_api_version<'a>() -> &'a str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
    <types>
        <members>HelloWorld</members>
        <name>ApexClass</name>
    </types>
    <version>39.0</version>
</Package>
"#
    }
    
    fn package_xml_wo_def_api_version<'a>() -> &'a str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
    <types>
        <members>HelloWorld</members>
        <name>ApexPage</name>
    </types>
    <version>30.0</version>
</Package>
"#
    }
    
    #[test]
    fn it_knows_its_package_xml_w_def_api_version() {
        let (members, mut package, mut types) = (
            Members::from("HelloWorld".to_string()),
            Package::new(None),
            Types::new("ApexClass")
        );
        
        types.members.push(members);
        package.types.push(types);
        
        assert_eq!(package.to_xml(), package_xml_w_def_api_version());
    }
    
    #[test]
    fn it_knows_its_package_xml_wo_def_api_version() {
        let (members, mut package, mut types) = (
            Members::from("HelloWorld".to_string()),
            Package::new(Some("30.0")),
            Types::new("ApexPage")
        );
        
        types.members.push(members);
        package.types.push(types);
        
        assert_eq!(package.to_xml(), package_xml_wo_def_api_version());
    }
}
