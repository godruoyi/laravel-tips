use crate::pretty::Pretty;
use crate::storage::{new_storage, Storage};
use crate::{github, Args, OutputFormat, SubCommands};
use argh::FromArgs;
use async_trait::async_trait;

struct Opts {
    format: OutputFormat,
    quiet: bool,
}

#[async_trait]
trait Commander {
    async fn execute(&self, storage: Box<dyn Storage>, opts: &Opts) -> anyhow::Result<()>;
}

pub struct Manager {
    args: Args,
}

impl Manager {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let com = self.args.nested.as_ref().unwrap();
        let storage = new_storage(self.args.engin.clone(), self.args.path.clone());

        let opt = Opts {
            format: self.args.output.clone().unwrap_or(OutputFormat::Terminal),
            quiet: self.args.quiet,
        };

        match com {
            SubCommands::Random(cmd) => cmd.execute(storage, &opt).await,
            SubCommands::Sync(cmd) => cmd.execute(storage, &opt).await,
            SubCommands::Search(cmd) => cmd.execute(storage, &opt).await,
        }
    }
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "sync")]
#[argh(description = "sync laravel tips from laravel docs")]
pub struct SyncCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "random")]
#[argh(description = "random laravel tips")]
pub struct RandomCommand {}

#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand, name = "search")]
#[argh(description = "search laravel tips by keyword")]
pub struct SearchCommand {
    #[argh(positional)]
    keyword: String,

    #[argh(option, short = 'g')]
    #[argh(description = "specify the group to search, such as 'eloquent', 'artisan', 'arr'")]
    group: Option<String>,
}

#[async_trait]
impl Commander for SyncCommand {
    async fn execute(&self, storage: Box<dyn Storage>, opts: &Opts) -> anyhow::Result<()> {
        if !opts.quiet {
            log!("Start sync all laravel tips from LaravelDaily/laravel-tips");
        }

        let entities = github::parse_all_laravel_tips(opts.quiet).await?;

        storage.store(entities).await?;

        if !opts.quiet {
            success!("Sync all laravel tips from successfully, run [random] to get a lucky tip");
        }

        Ok(())
    }
}

#[async_trait]
impl Commander for RandomCommand {
    async fn execute(&self, storage: Box<dyn Storage>, opts: &Opts) -> anyhow::Result<()> {
        let result = storage.random().await?;
        let e = result.ok_or_else(|| {
            anyhow::anyhow!("can not load tips from disk, please run [sync] first")
        })?;

        Pretty::new(opts.format.clone()).print_tip(e)
    }
}

#[async_trait]
impl Commander for SearchCommand {
    async fn execute(&self, storage: Box<dyn Storage>, opts: &Opts) -> anyhow::Result<()> {
        let entities = storage.search(&self.keyword, self.group.as_deref()).await?;

        Pretty::new(opts.format.clone()).print_tips(entities)
    }
}
