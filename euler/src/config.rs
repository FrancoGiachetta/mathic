use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigToml {
    package: Package,
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Dependency {
    // Matches: <dep> = <version>
    Simple(String),
    // Matches: <dep> = { version = <version> }
    Detailed { version: String },
}

impl ConfigToml {
    pub fn new(project_name: &str) -> Self {
        Self {
            package: Package {
                name: project_name.to_owned(),
                version: String::from("0.1.0"),
            },
            dependencies: HashMap::new(),
        }
    }
}
