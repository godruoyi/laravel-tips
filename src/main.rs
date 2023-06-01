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

    #[argh(option, short = 'e')]
    #[argh(description = "specify the search engine, default is SQLite, support [sqlite, file]")]
    engin: Option<SearchEngine>,

    #[argh(option, long = "path")]
    #[argh(description = "specify the path to store tips, default is $HOME/.laravel")]
    path: Option<String>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Random(RandomCommand),
    Sync(SyncCommand),
    Search(SearchCommand),
}

#[derive(Debug, PartialEq)]
pub enum SearchEngine {
    SQLite,
    File,
}

impl argh::FromArgValue for SearchEngine {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "sqlite" | "s" => Ok(Self::SQLite),
            "file" | "f" => Ok(Self::File),
            _ => Err(format!(
                "unknown search engine: {}, only support [sqlite, file]",
                value
            )),
        }
    }
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

    let storage = storage::new_storage(args.engin, args.path);

    match command {
        SubCommands::Random(_) => match storage.random().await {
            Ok(Some(e)) => {
                if let Err(err) = utils::pretty_tip(e) {
                    error!(format!("encountered an error: {}", err));
                }
            }
            Ok(None) => {
                error!("can not load tips from disk, please run [sync] first");
            }
            Err(e) => {
                error!(format!("we got an error: {}", e));
            }
        },
        SubCommands::Search(command) => match storage
            .search(&command.keyword, command.group.as_deref())
            .await
        {
            Ok(entities) => {
                if entities.len() == 0 {
                    log!("no tips found");
                } else if let Err(err) = utils::pretty_tips(entities) {
                    error!(format!("encountered an error: {}", err));
                }
            }
            Err(e) => {
                error!(format!("encountered an error: {}", e));
            }
        },
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
A command line tool for laravel tips

Options:
  -v, --version     show version
  -e, --engin       specify the search engine, default is SQLite, support
                    [sqlite, file]
  --path            specify the path to store tips, default is $HOME/.laravel
  --help            display usage information

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
"#;
