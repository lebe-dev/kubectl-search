use std::collections::HashMap;

pub mod values;

pub struct SearchResult {
    pub resource_name: String,
    pub values: HashMap<String, String>
}