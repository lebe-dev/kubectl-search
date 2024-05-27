use std::collections::HashMap;
use std::error::Error;

use anyhow::anyhow;
use kubectl_wrapper_rs::{KubectlWrapper, KubernetesResourceType};
use kubectl_wrapper_rs::configmap::KubectlConfigMapWrapper;
use kubectl_wrapper_rs::error::KubectlWrapperError;
use log::{debug, info};

use crate::k8s::KubectlTool;
use crate::usecase::SearchResult;

pub fn search_values_in_configmaps(
    kubectl_tool: &dyn KubectlWrapper,
    kubectl_configmap_tool: &dyn KubectlConfigMapWrapper,
    namespace: &str, mask: &str) -> anyhow::Result<Vec<SearchResult>> {
    info!("search values in config-maps with mask '{mask}', namespace '{namespace}'..");

    match kubectl_tool.get_resource_names(namespace, KubernetesResourceType::ConfigMap) {
        Ok(names) => {
            debug!("configmaps received: {:?}", names);

            let mut results: Vec<SearchResult> = vec![];

            let names: Vec<String> = names.into_iter().filter(|n| {
                let name = n.trim();
                !name.is_empty()
            }).collect::<Vec<String>>();

            for configmap_name in names {
                match kubectl_configmap_tool.get_configmap_key_values(&namespace, &configmap_name) {
                    Ok(config_map_values) => {
                        let mut filtered_map: HashMap<String,String> = HashMap::new();

                        for (k, v) in config_map_values {
                            let lowercased_values = v.to_lowercase();

                            if lowercased_values.contains(&mask) {
                                info!("- match '{k}': '{v}'");
                                let _ = &filtered_map.insert(k, v);
                            }
                        }

                        let result = SearchResult {
                            resource_name: configmap_name.to_string(),
                            values: filtered_map.clone(),
                        };

                        results.push(result);
                    }
                    Err(_) => {
                        return Err(anyhow!(format!("unable to get key-values from configmap '{configmap_name}'")))
                    }
                }
            }

            Ok(results)
        }
        Err(_) => {
            Err(anyhow!(format!("unable to get configmap names in namespace '{namespace}'")))
        }
    }
}