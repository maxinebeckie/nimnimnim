use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

use nimlib::{moves, NimAction, NimGame, NimRule, Split, Stack, TakeSize};

const RULES: [NimRule; 1] = [NimRule {
    take: TakeSize::Any,
    split: Split::Never,
}];

const STACKS: [Stack; 4] = [Stack(1), Stack(3), Stack(5), Stack(7)];

fn create_nim_game() -> NimGame {
    NimGame::new(RULES.to_vec(), STACKS.to_vec())
}

fn print_example_game(game: &NimGame) {
    dbg!(&game);
}

fn calculate_example_nimber() {
    let game = create_nim_game();
    println!("{}", game.calculate_nimber());
}

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
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
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
        .arg(arg!(
            -e --example ... "prints an example game"
        ))
        .arg(arg!(
            --calculate ... "calculates the nim sum of example game"
        ))
        .arg(arg!(
            -t --take ... "take an example number"
        ))
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    match matches.get_one::<u8>("example") {
        Some(_) => print_example_game(&create_nim_game()),
        None => (),
    }

    match matches.get_one::<u8>("calculate") {
        Some(_) => calculate_example_nimber(),
        None => (),
    }

    match matches.get_one::<u8>("take") {
        Some(_) => {
            let mut game = create_nim_game();
            make_move_woo(&mut game);
            print_example_game(&game);
            println!("{}", game.calculate_nimber());
        }
        None => (),
    }

    // check how many flags or args occured
    // only flags can have multiple occurrences
    // wtf is the difference btwn a flag and an arg?????
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
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
}
