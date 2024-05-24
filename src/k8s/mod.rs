use std::collections::HashMap;

use serde::Deserialize;

pub const KUBECTL_EXEC_PATH: &str = "kubectl";

pub trait KubectlTool {
}

pub struct KubectlToolImpl;

impl KubectlToolImpl {
    pub fn new() -> Self {
        Self
    }
}

impl KubectlTool for KubectlToolImpl {

}

#[derive(Deserialize)]
pub struct KubernetesResource {
    pub data: HashMap<String, String>
}
