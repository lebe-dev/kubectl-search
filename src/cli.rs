use std::env;
use std::path::Path;

use clap::{Arg, ArgAction, ArgMatches, Command};
use log::debug;

pub const WORKDIR: &str = ".";
pub const WORK_DIR_ARGUMENT: &str = "work-dir";
pub const WORK_DIR_SHORT_ARGUMENT: char = 'd';

pub const LOG_LEVEL_ARGUMENT: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "off";

pub const VALUES_COMMAND: &str = "values";

pub const NAMESPACE_ARG: &str = "namespace";
pub const SEARCH_MASK_ARG: &str = "mask";

pub const SECRETS_FLAG: &str = "secrets";

pub const IGNORE_BASE64_ERRORS_FLAG: &str = "ignore-base64-errors";
pub const IGNORE_UTF8_ERRORS_FLAG: &str = "ignore-utf8-errors";

pub fn init_cli_app() -> ArgMatches {
    Command::new("kubectl-search")
        .version("0.2.0")
        .author("Eugene Lebedev <eugene.0x90@gmail.com>")
        .about("Search tool for ConfigMaps and Secrets (Kubernetes)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new(WORK_DIR_ARGUMENT)
                .short(WORK_DIR_SHORT_ARGUMENT)
                .help("set working directory")
                .long(WORK_DIR_ARGUMENT)
                .required(false)
        )
        .arg(
            Arg::new(LOG_LEVEL_ARGUMENT)
                .help("set logging-level")
                .long(LOG_LEVEL_ARGUMENT)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
                .required(false)
        )
        .subcommand(
            Command::new(VALUES_COMMAND)
                .about("search values by mask")
                .arg(get_ignore_base64_errors_flag())
                .arg(get_ignore_utf8_errors_flag())
                .arg(get_k8s_namespace_arg())
                .arg(get_search_mask_arg())
                .arg(get_secrets_flag())

        )
        .get_matches()
}

fn get_k8s_namespace_arg() -> Arg {
    Arg::new(NAMESPACE_ARG)
        .help("kubernetes namespace. Example: demo")
        .required(true)
}

fn get_search_mask_arg() -> Arg {
    Arg::new(SEARCH_MASK_ARG)
        .help("search mask")
        .required(true)
}

fn get_secrets_flag() -> Arg {
    Arg::new(SECRETS_FLAG)
        .long(SECRETS_FLAG)
        .help("search in secret values")
        .action(ArgAction::SetTrue)
        .required(false)
}

fn get_ignore_base64_errors_flag() -> Arg {
    Arg::new(IGNORE_BASE64_ERRORS_FLAG)
        .long(IGNORE_BASE64_ERRORS_FLAG)
        .help("ignore base64 decoding errors. Use secret value AS IS if base64 related error occurs")
        .action(ArgAction::SetTrue)
        .required(false)
}

fn get_ignore_utf8_errors_flag() -> Arg {
    Arg::new(IGNORE_UTF8_ERRORS_FLAG)
        .long(IGNORE_UTF8_ERRORS_FLAG)
        .help("ignore utf-8 related errors. Use secret value AS IS if utf-8 related error occurs")
        .action(ArgAction::SetTrue)
        .required(false)
}

pub fn init_working_dir(matches: &ArgMatches) {
    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARGUMENT, WORKDIR);

    debug!("working directory '{}'", &working_directory.display());

    env::set_current_dir(&working_directory).expect("couldn't set working directory");
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, long_argument: &str,
                               default_path: &'a str) -> &'a Path {
    let mut path: &Path = Path::new(default_path);

    match matches.get_one::<String>(long_argument) {
        Some(value) => path = Path::new(value),
        None => {}
    }

    return path;
}