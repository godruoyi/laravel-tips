mod github;
mod utils;
#[macro_use]
mod ui;
mod model;
mod storage;

use crate::SubCommands::Sync;
use argh::FromArgs;

const VERSION: &str = "0.0.3";

#[derive(FromArgs, Debug)]
#[argh(description = "A command line tool for laravel tips")]
struct Args {
    #[argh(switch, short = 'v')]
    #[argh(description = "show version")]
    version: bool,

    #[argh(subcommand)]
    nested: Option<SubCommands>,

    #[argh(option, long = "file-path")]
    #[argh(description = "specify the file path to store tips, default is $HOME/.laravel/.tips")]
    file_path: Option<String>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Random(RandomCommand),
    Sync(SyncCommand),
    Search(SearchCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "random")]
#[argh(description = "random laravel tips")]
struct RandomCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "sync")]
#[argh(description = "sync laravel tips from laravel docs")]
struct SyncCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "search")]
#[argh(description = "search laravel tips by keyword")]
struct SearchCommand {
    #[argh(positional)]
    keyword: String,

    #[argh(option, short = 'g')]
    #[argh(description = "specify the group to search, such as 'eloquent', 'artisan', 'arr'")]
    group: Option<String>,
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{}", VERSION);
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("{}", WELOCOME);
        std::process::exit(0);
    }

    let command = args.nested.unwrap_or_else(|| {
        std::process::exit(1);
    });

    let storage = storage::new_storage(args.file_path);

    match command {
        SubCommands::Random(_) => match storage.random().await {
            Ok(Some(e)) => {
                pretty_tip!(e.title, e.content)
            }
            Ok(None) => {
                error!("can not load tips from disk, please run [sync] first");
            }
            Err(e) => {
                error!(format!("we got an error: {}", e));
            }
        },
        SubCommands::Search(command) => {
            log!(format!(
                "Start search laravel tips by keyword: {}, group: {}",
                command.keyword,
                command.group.unwrap_or_default()
            ));

            log!("to be continue...")
        }
        Sync(_) => {
            log!("Start sync all laravel tips from LaravelDaily/laravel-tips");

            let entities = match github::parse_all_laravel_tips().await {
                Ok(entities) => entities,
                Err(err) => {
                    error!(format!("encountered an error: {}", err));
                    std::process::exit(1);
                }
            };

            if let Err(err) = storage.store(entities).await {
                error!(format!("encountered an error: {}", err));
                std::process::exit(1);
            }

            success!("Sync all laravel tips from successfully, run [random] to get a lucky tip");
        }
    }
}

static WELOCOME: &str = r#"
Usage: laraveltips [-v] [<command>] [<args>]

A command line tool for laravel tips

Options:
  -v, --version     show version
  --help            display usage information
  --path|-p         set laravel tips storage path, default is ~/.laravel/tips.json

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
"#;
