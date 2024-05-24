use crate::usecase::SearchResult;

pub fn print_search_results(search_results: &Vec<SearchResult>, search_mask: &str) {
    println!("------------------");
    println!("SEARCH RESULTS:");
    println!("------------------");

    if search_results.is_empty() {
        println!("no values found by mask '{search_mask}'");

    } else {
        for search_result in search_results {
            println!("- config-map: '{}'", search_result.resource_name);

            for (k, v) in search_result.values {
                println!("  - '{k}': '{v}'")
            }
        }
    }
}