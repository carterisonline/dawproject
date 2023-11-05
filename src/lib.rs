pub use crate::project::*;
use serde::{Deserialize, Serialize};

#[allow(hidden_glob_reexports)]
mod project;

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetaData {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    original_artist: Option<String>,
    composer: Option<String>,
    songwriter: Option<String>,
    producer: Option<String>,
    arranger: Option<String>,
    year: Option<String>,
    genre: Option<String>,
    copyright: Option<String>,
    website: Option<String>,
    comment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_project_xml() {
        quick_xml::de::from_str::<Project>(&std::fs::read_to_string("./project.xml").unwrap())
            .unwrap();
    }
}
