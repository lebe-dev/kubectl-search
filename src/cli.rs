use std::env;
use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use log::debug;

pub const WORKDIR: &str = ".";
pub const WORK_DIR_ARGUMENT: &str = "work-dir";
pub const WORK_DIR_SHORT_ARGUMENT: char = 'd';

pub const LOG_LEVEL_ARGUMENT: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "off";

pub const VALUES_COMMAND: &str = "values";

pub const NAMESPACE_ARG: &str = "namespace";
pub const SEARCH_MASK_ARG: &str = "mask";

pub fn init_cli_app() -> ArgMatches {
    Command::new("kubectl-search")
        .version("0.1.0")
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
                .arg(get_k8s_namespace_arg())
                .arg(get_search_mask_arg())

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