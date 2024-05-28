use std::collections::HashMap;

use kubectl_wrapper_rs::KubernetesResourceType;

pub mod values;

pub struct SearchResult {
    pub resource_name: String,
    pub resource_type: KubernetesResourceType,
    pub values: HashMap<String, String>
}