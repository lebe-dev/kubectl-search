use kubectl_wrapper_rs::KubernetesResourceType;

use crate::usecase::SearchResult;

pub fn print_search_results(search_results: &Vec<SearchResult>, search_mask: &str) {
    println!("------------------");
    println!("SEARCH RESULTS:");
    println!("------------------");

    if search_results.is_empty() {
        println!("no values found by mask '{search_mask}'");

    } else {
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
                    match search_result.resource_type {
                        KubernetesResourceType::ConfigMap => {
                            println!("  - '{k}': '{v}'")
                        }
                        KubernetesResourceType::Secret => {
                            println!("  - '{k}': '***********'")
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}