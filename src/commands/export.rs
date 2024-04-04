use crate::link::Link;
use anyhow::Context;
use clap::Args;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    /// Number of links to output
    #[clap(short, long, default_value = "30")]
    limit: u32,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let links: Vec<Link> = sqlx::query_as("SELECT * FROM links LIMIT $1")
        .bind(args.limit as i32)
        .fetch_all(&ctx.db)
        .await
        .context("could not fetch data from the database")?;
    serde_json::to_writer(std::io::stdout().lock(), &links).context("serialization error")?;
    Ok(())
}
