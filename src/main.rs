use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::{arg, command, value_parser, ArgAction, Command};
use nimlib::{moves, NimAction, NimGame, NimRule, Split, Stack, TakeSize};

use log::{debug, error, info, log_enabled, Level};

use serde::{Deserialize, Serialize};

use env_logger::Env;

fn main() {

    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("RKO: {e}");
            1
        }
    });
}

fn run() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Set custom rules in RON format. If no file is specified, a default config will be generated."
            )
            .required(false)
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(
                -s --sum <PILES> "return nim sum of piles according to config, eg. nimnimnim -s \"1 3 5 7\" returns *0 with default config."
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -v --verbose ... "Once enables warning log levels, twice info log levels."
            )
            .required(false)
        )
        .get_matches();

    let mut rule: NimRule = NimRule {
        take: TakeSize::Any,
        split: Split::Never,
    };

    // determine verbosity level
    match matches.get_one::<u8>("verbose").unwrap() {
        0 => env_logger::Builder::from_env(Env::default().default_filter_or("error")).init(),
        1 => env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init(),
        _ => env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(),
    }

    // config
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        // parse rules from config
        rule = ron::from_str(&match std::fs::read_to_string(config_path) {
            Ok(ron_string) => {
                info!("using config file provided");
                ron_string
            }
            Err(e) => match e {
                NotFound => {
                    info!("config file not found. using default config..");
                    emit_default_config(&rule, config_path)?
                }
                _ => {
                    info!("another error..");
                    return Err(anyhow::Error::from(e));
                }
            },
        })?;
    }
    // sum
    if let Some(piles) = matches.get_one::<String>("sum") {
        info!("summing pile according to rules..");
        let mut piles_vec: Vec<u64> = Vec::new();
        for p in piles.split_whitespace() {
            piles_vec.push(p.parse::<u64>()?);
        }
        let stacks: Vec<Stack> = piles_vec.into_iter().map(|n| Stack(n)).collect();
        let game = NimGame::new([rule].to_vec(), stacks);
        println!("{}", game.calculate_nimber());
    }

    Ok(())
}

// if config file is not found
fn emit_default_config(rule: &NimRule, path: &PathBuf) -> Result<String> {
    let config_to_emit = ron::to_string(rule)?;
    let comment_1 = "// configuration is in the RON format, more info can be found at https://github.com/ron-rs/ron.";
    let comment_2 = "// the default rules are as follows. They mean you can take as many matchsticks from one pile, and piles are never";
    let comment_3 = "// split into two or more piles. Any valid NimRule struct according to the nimlib documentation is a valid configuration.";
    let data_to_write = vec![
        config_to_emit,
        comment_1.to_string(),
        comment_2.to_string(),
        comment_3.to_string(),
    ];

    std::fs::write(path, data_to_write.join("\n"))?;

    let default_config_string = std::fs::read_to_string(PathBuf::from("./config.ron"))?;
    Ok(default_config_string)
}
