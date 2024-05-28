use std::collections::HashMap;

use anyhow::anyhow;
use kubectl_wrapper_rs::{KubectlWrapper, KubernetesResourceType};
use kubectl_wrapper_rs::configmap::KubectlConfigMapWrapper;
use kubectl_wrapper_rs::error::KubectlWrapperError;
use kubectl_wrapper_rs::secret::KubectlSecretWrapper;
use log::{debug, info};

use crate::usecase::SearchResult;

pub struct ValuesSearchOptions {
    pub search_in_secrets: bool,
    pub ignore_base64_errors: bool,
    pub ignore_utf8_errors: bool,
}

pub fn search_values(
    kubectl_tool: &dyn KubectlWrapper,
    kubectl_configmap_tool: &dyn KubectlConfigMapWrapper,
    kubectl_secret_tool: &dyn KubectlSecretWrapper,
    namespace: &str, mask: &str,
    search_options: &ValuesSearchOptions) -> anyhow::Result<Vec<SearchResult>> {
    info!("search values with mask '{mask}', namespace '{namespace}'..");

    let mut results: Vec<SearchResult> = vec![];

    match kubectl_tool.get_resource_names(namespace, KubernetesResourceType::ConfigMap) {
        Ok(names) => {
            debug!("configmaps received: {:?}", names);

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
                            resource_type: KubernetesResourceType::ConfigMap,
                            values: filtered_map.clone(),
                        };

                        results.push(result);
                    }
                    Err(_) => {
                        return Err(anyhow!(format!("unable to get key-values from configmap '{configmap_name}'")))
                    }
                }
            }
        }
        Err(_) => {
            return Err(anyhow!(format!("unable to get configmap names in namespace '{namespace}'")))
        }
    }

    if search_options.search_in_secrets {
        match kubectl_tool.get_resource_names(namespace, KubernetesResourceType::Secret) {
            Ok(names) => {
                debug!("secrets received: {:?}", names);

                let names: Vec<String> = names.into_iter().filter(|n| {
                    let name = n.trim();
                    !name.is_empty()
                }).collect::<Vec<String>>();

                for configmap_name in names {
                    match kubectl_secret_tool.get_secret_key_values(&namespace, &configmap_name,
                                                                    search_options.ignore_base64_errors,
                                                                    search_options.ignore_utf8_errors) {
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
                                resource_type: KubernetesResourceType::Secret,
                                values: filtered_map.clone(),
                            };

                            results.push(result);
                        }
                        Err(_) => {
                            return Err(anyhow!(format!("unable to get key-values from secret '{configmap_name}'")))
                        }
                    }
                }
            }
            Err(_) => {
                return Err(anyhow!(format!("unable to get secrets in namespace '{namespace}'")))
            }
        }
    }

    Ok(results)
}