mod github;
mod utils;
#[macro_use]
mod ui;

use crate::github::Entity;
use crate::SubCommands::Sync;
use argh::FromArgs;
use rand::prelude::SliceRandom;

const VERSION: &str = "0.0.2";

#[derive(FromArgs, Debug)]
#[argh(description = "A command line tool for laravel tips")]
struct Args {
    #[argh(switch, short = 'v')]
    #[argh(description = "show version")]
    version: bool,

    #[argh(subcommand)]
    nested: Option<SubCommands>,

    #[argh(option, short = 'p')]
    #[argh(description = "path to save all tips, default it $HOME/.laravel-tips")]
    path: Option<String>,
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
            if let Ok(entities) = utils::load_tips_from_disk::<Entity>(args.path) {
                if !entities.is_empty() {
                    let mut rng = rand::thread_rng();
                    let entity = entities.choose(&mut rng).unwrap();

                    pretty_tip!(entity.title, entity.content);
                    std::process::exit(0);
                }
            }
            error!("can not load tips from disk, please run [sync] first");
        }
        Sync(_) => {
            log!("Start sync all laravel tips from LaravelDaily/laravel-tips\n");

            let (trees, total) = github::get_get_laravel_tips_trees_with_size()
                .expect("can not get trees from github");
            ui::progress_bar(total, github::process_trees(args.path, trees));

            success!("Sync all laravel tips from successfully, run [random] to get a lucky tip");
        }
    }
}
