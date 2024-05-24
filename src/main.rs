use std::env;
use std::process::exit;

use clap::ArgMatches;

use crate::cli::{init_cli_app, init_working_dir, LOG_LEVEL_ARGUMENT, LOG_LEVEL_DEFAULT_VALUE, SEARCH_MASK_ARG};
use crate::k8s::KubectlToolImpl;
use crate::logging::get_logging_config;
use crate::output::print_search_results;
use crate::usecase::values::search_values_in_configmaps;

mod cli;
mod logging;
mod k8s;
mod usecase;
mod output;

const BUILD: &str = "UNKNOWN";

fn main() {
    let matches = init_cli_app();
    init_logging(&matches);
    init_working_dir(&matches);

    match matches.subcommand() {
        Some(("values", matches)) => {
            let search_mask = matches.get_one::<String>(SEARCH_MASK_ARG).unwrap();

            println!("find configmap values with mask '{search_mask}'..");

            check_required_env_vars(&vec!["KUBECONFIG"]);

            let kubectl_tool = KubectlToolImpl::new();

            match search_values_in_configmaps(&kubectl_tool, &search_mask) {
                Ok(search_results) => print_search_results(&search_results, &search_mask),
                Err(e) => eprintln!("error: {}", e)
            }
        },
        _ => println!("use -h to get help")
    }
}

fn init_logging(matches: &ArgMatches) {
    let log_level = match matches.get_one::<String>(LOG_LEVEL_ARGUMENT) {
        Some(value) => {value}
        None => LOG_LEVEL_DEFAULT_VALUE
    };

    let logging_config = get_logging_config(log_level);
    log4rs::init_config(logging_config).expect("logging init error");
}

fn check_required_env_vars(required_vars: &Vec<&str>) {
    for var_name in required_vars {
        if env::var_os(var_name).is_none() {
            eprintln!("error: environment variable '{var_name}' is not defined. exit");
            exit(1)
        }
    }
}