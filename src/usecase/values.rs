use crate::k8s::KubectlTool;
use crate::usecase::SearchResult;

pub fn search_values_in_configmaps(kubectl_tool: &dyn KubectlTool, mask: &str) -> anyhow::Result<Vec<SearchResult>> {
    unimplemented!()
}