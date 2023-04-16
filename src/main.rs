use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::{arg, command, value_parser, ArgAction, Command};
use nimlib::{moves, NimAction, NimGame, NimRule, Split, Stack, TakeSize};

const RULES: [NimRule; 1] = [NimRule {
    take: TakeSize::Any,
    split: Split::Never,
}];

const STACKS: [Stack; 4] = [Stack(1), Stack(3), Stack(5), Stack(7)];

fn make_move_woo(game: &mut NimGame) {
    let mov = moves::calculate_legal_moves(&STACKS, &RULES, (0, 0))
        .into_iter()
        .filter(|mov| match mov {
            NimAction::Take(take) => true,
            _ => false,
        })
        .take(1)
        .collect::<Vec<NimAction>>()
        .pop()
        .unwrap();

    unsafe {
        moves::apply_move_unchecked(game, &mov);
    }
}

fn main() {
    env_logger::init();

    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fatal: {e}");
            1
        }
    });
}

fn run() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "set custom rules in RON format (https://github.com/ron-rs/ron). Default is:
                         (take: TakeSize::Any,  split: Split::Never)
                for more information see the nimlib documentation on the NimRule struct."
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
       .arg(
            arg!(
                -s --sum <PILE> "return nim sum of piles, eg. nimnimnim -s \"1 3 5 7\" returns 0"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .get_matches();

    // config
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }


    // sum 
    if let Some(piles) = matches.get_one::<String>("sum") {
        let mut piles_vec: Vec<u64> = Vec::new();

        for p in piles.split_whitespace() {
            piles_vec.push(p.parse::<u64>()?);
        }

        let stacks: Vec<Stack> = piles_vec.into_iter().map(|n| Stack(n)).collect();

        let game = NimGame::new(RULES.to_vec(), stacks);

        println!("{}", game.calculate_nimber());
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    Ok(())
}
