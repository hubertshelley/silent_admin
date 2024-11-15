use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeleteParam {
    pub ids: Vec<String>,
}

impl DeleteParam {
    pub fn ids(&self) -> Vec<String> {
        self.ids.clone()
    }
}
