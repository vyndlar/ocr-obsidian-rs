#[derive(Clone, Debug)]
pub struct ObsidianVault {
    pub name: String,

    // absolute string to represent the filepath of the vault
    pub filepath: String,

    // tags are formated like this: #<tag-text>
    pub tags: Vec<String>,
}

impl ObsidianVault {
    pub fn new(name: String, filepath: String) -> Self {
        ObsidianVault {
            name,
            filepath,
            tags: vec![],
        }
    }
}
