mod commands;
mod link;

use anyhow::Context;
use clap::{ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = true,
    color = ColorChoice::Never
)]
/// Yet another li(nk)/bookmark manager
struct Cli {
    /// Databas connection url
    #[arg(short, long, env = "LI_DATABASE_URL")]
    database_url: String,

    /// Subcommand
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new link
    #[clap(alias = "a")]
    Add(commands::add::Arguments),

    /// Query links in the links directory
    #[clap(alias = "q")]
    Query(commands::query::Arguments),

    /// Remove link by ID
    #[clap(alias = "rm")]
    Remove(commands::remove::Arguments),

    /// Edit link meta data
    #[clap(alias = "ed")]
    Edit(commands::edit::Arguments),

    /// Open link by ID
    #[clap(alias = "o")]
    Open(commands::open::Arguments),

    /// list tags
    Tags(commands::tags::Arguments),

    /// Import links from a json file
    Import(commands::import::Arguments),

    /// Export data
    Export(commands::export::Arguments),

    /// Initialize the database
    #[clap(name = "initdb")]
    InitDb(commands::initdb::Arguments),

    /// Shell completion
    Complete(commands::complete::Arguments),
}

pub(crate) struct Ctx {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sigpipe::reset();

    let cli = Cli::parse();

    let db_conn = sqlx::PgPool::connect(&cli.database_url)
        .await
        .with_context(|| format!("could not connect to the database: {}", cli.database_url))?;

    let ctx = Ctx { db: db_conn };
    match cli.commands.unwrap() {
        Commands::Add(args) => commands::add::action(ctx, args).await,
        Commands::Query(args) => commands::query::action(ctx, args).await,
        Commands::Remove(args) => commands::remove::action(ctx, args).await,
        Commands::Edit(args) => commands::edit::action(ctx, args).await,
        Commands::Import(args) => commands::import::action(ctx, args).await,
        Commands::Export(args) => commands::export::action(ctx, args).await,
        Commands::InitDb(args) => commands::initdb::action(ctx, args).await,
        Commands::Open(args) => commands::open::action(ctx, args).await,
        Commands::Complete(args) => commands::complete::action(ctx, args).await,
        Commands::Tags(args) => commands::tags::action(ctx, args).await,
    }
}
