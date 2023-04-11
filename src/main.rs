mod parser;
mod http;
mod github;
mod utils;

#[macro_use]
mod ui;

use argh::FromArgs;
use crate::parser::Entity;
use rand::seq::SliceRandom;

const VERSION: &str = "0.0.1";

#[derive(FromArgs, Debug)]
#[argh(description = "A sample program")]
struct Args {
    #[argh(switch, short = 'v')]
    #[argh(description = "show version")]
    version: bool,

    #[argh(subcommand)]
    nested: Option<SubCommands>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Random(RandomCommand),
    Sync(SyncCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "random")]
#[argh(description = "random laravel tips")]
struct RandomCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "sync")]
#[argh(description = "sync laravel tips from laravel docs")]
struct SyncCommand {}

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{}", VERSION);
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("\nWelcome to laravel tips\n");
        std::process::exit(0);
    }

    let command = args.nested.unwrap_or_else(|| {
        std::process::exit(1);
    });

    match command {
        SubCommands::Random(_) => {
            let x: Vec<Entity> = utils::load_tips_from_disk().unwrap();
            let mut rng = rand::thread_rng();
            let entity = x.choose(&mut rng).unwrap();

            println!("{}\n\n{}", entity.title, entity.content)
        }
        SubCommands::Sync(_) => {
            log!("Start sync all laravel tips from {} ...", "LaravelDaily/laravel-tips");

            let entities = parser::parse().unwrap();
            utils::save_tips_to_disk(entities).unwrap();

            success!("Sync all laravel tips from {} successfully", "LaravelDaily/laravel-tips");
        }
    }
}