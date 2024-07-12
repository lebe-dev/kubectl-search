use std::collections::HashMap;

use anyhow::anyhow;
use kubectl_wrapper_rs::{KubectlWrapper, KubernetesResourceType};
use kubectl_wrapper_rs::configmap::KubectlConfigMapWrapper;
use kubectl_wrapper_rs::secret::KubectlSecretWrapper;
use log::{debug, error, info, warn};
use vault_cli_wrapper::VaultCliWrapper;

use crate::usecase::SearchResult;

pub struct ValuesSearchOptions {
    pub search_in_secrets: bool,
    pub search_in_vault_secrets: bool,
    pub ignore_base64_errors: bool,
    pub ignore_utf8_errors: bool
}

pub fn search_values(
    kubectl_tool: &dyn KubectlWrapper,
    kubectl_configmap_tool: &dyn KubectlConfigMapWrapper,
    kubectl_secret_tool: &dyn KubectlSecretWrapper,
    vault_tool: &dyn VaultCliWrapper,
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
        info!("search in secrets..");
        match kubectl_tool.get_resource_names(namespace, KubernetesResourceType::Secret) {
            Ok(names) => {
                debug!("secret names received: {:?}", names);

                let names: Vec<String> = names.into_iter().filter(|n| {
                    let name = n.trim();
                    !name.is_empty()
                }).collect::<Vec<String>>();

                for configmap_name in names {
                    match kubectl_secret_tool.get_secret_key_values(&namespace, &configmap_name,
                                                                    search_options.ignore_base64_errors,
                                                                    search_options.ignore_utf8_errors) {
                        Ok(secret_map) => {
                            let mut filtered_map: HashMap<String,String> = HashMap::new();

                            for (k, v) in secret_map {
                                let mut value = v.to_string();

                                if search_options.search_in_vault_secrets &&
                                   value.starts_with("vault:") {
                                    info!("vault path detected '{value}'");

                                    let parts = value.split("#").collect::<Vec<&str>>();

                                    if parts.len() > 1 {
                                        let var_name = parts.last().unwrap();

                                        let vault_path = value.replace(&format!("#{var_name}"), "");

                                        // vault:kv/data/some/path#VAR
                                        match vault_tool.read_secrets_data(&vault_path) {
                                            Ok(secrets) => {
                                                if secrets.contains_key(*var_name) {
                                                    value = secrets.get(*var_name).unwrap().to_string();
                                                    info!("vault secret value successfully extracted from path '{v}'");

                                                } else {
                                                    warn!("unexpected error: extracted vault secrets from path don't contain '{var_name}' secret, skip");
                                                }
                                            }
                                            Err(_) => error!("unable to resolve vault path to value, skip")
                                        }

                                    } else {
                                        warn!("unsupported vault path format: '{value}'")
                                    }
                                }

                                let lowercased_values = value.to_lowercase();

                                if lowercased_values.contains(&mask) {
                                    info!("- match '{k}': '************'");
                                    let _ = &filtered_map.insert(k, value);
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