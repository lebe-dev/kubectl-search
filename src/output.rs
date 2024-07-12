use kubectl_wrapper_rs::KubernetesResourceType;

use crate::logging::LOG_LINE_SEPARATOR;
use crate::usecase::SearchResult;

pub fn print_search_results(search_results: &Vec<SearchResult>,
                            search_mask: &str, unmask_secret_values: bool,
                            value_max_length: usize) {
    println!("{}", LOG_LINE_SEPARATOR);
    println!("SEARCH RESULTS:");
    println!("{}", LOG_LINE_SEPARATOR);

    if search_results.is_empty() {
        println!("no values found by mask '{search_mask}'");

    } else {

        let max_length_notice_msg = format!("(value max length {value_max_length})");

        for search_result in search_results {
            if !search_result.values.is_empty() {
                match search_result.resource_type {
                    KubernetesResourceType::ConfigMap => {
                        println!("- config-map: '{}'", search_result.resource_name)
                    }
                    KubernetesResourceType::Secret => {
                        println!("- secret: '{}'", search_result.resource_name)
                    }
                    _ => {}
                }

                for (k, v) in &search_result.values {
                    let mut value: String = v.chars().take(value_max_length).collect();

                    if v.len() > value.len() {
                        value = format!("'{value}...' {max_length_notice_msg}")
                    }

                    match search_result.resource_type {
                        KubernetesResourceType::ConfigMap => {
                            println!("  - '{k}': {value}")
                        }
                        KubernetesResourceType::Secret => {
                            if unmask_secret_values {
                                println!("  - '{k}': {value}")

                            } else {
                                println!("  - '{k}': '***********'")
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}