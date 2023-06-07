mod github;
mod utils;
#[macro_use]
mod ui;
mod command;
mod model;
mod pretty;
mod storage;

use argh::FromArgs;

const VERSION: &str = "0.0.3";

#[derive(FromArgs, Debug)]
#[argh(description = "A command line tool for laravel tips")]
pub struct Args {
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

    #[argh(option, short = 'o')]
    #[argh(
        description = "specify the output format, default is display in terminal, support [text, json]"
    )]
    output: Option<OutputFormat>,

    #[argh(switch, short = 'q')]
    #[argh(description = "quiet mode, only output the result")]
    quiet: bool,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Random(command::RandomCommand),
    Sync(command::SyncCommand),
    Search(command::SearchCommand),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    Text,
    Terminal,
    Json,
}

impl argh::FromArgValue for OutputFormat {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "text" | "t" => Ok(Self::Text),
            "json" | "j" => Ok(Self::Json),
            _ => Ok(Self::Terminal),
        }
    }
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

    if let Err(err) = command::Manager::new(args).execute().await {
        error!(format!("encountered an error: {}", err));

        std::process::exit(1);
    }
}

static WELOCOME: &str = r#"
A command line tool for laravel tips

Options:
  -v, --version     show version
  --path            specify the path to store tips, default is $HOME/.laravel
  -e, --engin       specify the search engine, default is SQLite, support [sqlite, file]
  -o, --output      specify the output format, default is display in terminal, support [text, json]
  -q, --quiet       quiet mode, only output the result
  --help            display usage information

Commands:
  random            random laravel tips
  sync              sync laravel tips from laravel docs
  search            search laravel tips by keyword
"#;
