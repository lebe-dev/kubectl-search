use std::env;
use std::process::exit;

use clap::ArgMatches;
use kubectl_wrapper_rs::executor::DefaultKubectlExecutor;
use kubectl_wrapper_rs::KubectlWrapperImpl;

use crate::cli::{IGNORE_BASE64_ERRORS_FLAG, IGNORE_UTF8_ERRORS_FLAG, init_cli_app, init_working_dir, LOG_LEVEL_ARGUMENT, LOG_LEVEL_DEFAULT_VALUE, NAMESPACE_ARG, SEARCH_MASK_ARG, SECRETS_FLAG};
use crate::logging::get_logging_config;
use crate::output::print_search_results;
use crate::usecase::values::{search_values, ValuesSearchOptions};

mod cli;
mod logging;
mod usecase;
mod output;

const BUILD: &str = "UNKNOWN";

fn main() {
    let matches = init_cli_app();
    init_logging(&matches);
    init_working_dir(&matches);

    match matches.subcommand() {
        Some(("values", matches)) => {
            let namespace = matches.get_one::<String>(NAMESPACE_ARG).unwrap();
            let search_mask = matches.get_one::<String>(SEARCH_MASK_ARG).unwrap();

            println!("find values in '{namespace}' namespace with mask '{search_mask}'..");

            let search_in_secrets = matches.get_flag(SECRETS_FLAG);
            let ignore_base64_errors = matches.get_flag(IGNORE_BASE64_ERRORS_FLAG);
            let ignore_utf8_errors = matches.get_flag(IGNORE_UTF8_ERRORS_FLAG);

            println!("- search in secret values: {search_in_secrets}");
            println!("- ignore base64 errors: {ignore_base64_errors}");
            println!("- ignore utf8 errors: {ignore_utf8_errors}");

            check_required_env_vars(&vec!["KUBECONFIG"]);

            let executor = DefaultKubectlExecutor::new();
            let kubectl_tool = KubectlWrapperImpl::new(&executor);

            let search_options = ValuesSearchOptions {
                search_in_secrets,
                ignore_base64_errors,
                ignore_utf8_errors,
            };

            match search_values(&kubectl_tool, &kubectl_tool, &kubectl_tool,
                                &namespace, &search_mask, &search_options) {
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