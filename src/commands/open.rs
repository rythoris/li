use crate::link::Link;
use anyhow::Context;
use clap::Args;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    /// Link ID
    id: u32,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let link: Link = sqlx::query_as("SELECT * FROM links WHERE id = $1 LIMIT 1")
        .bind(args.id as i32)
        .fetch_one(&ctx.db)
        .await
        .context("could not fetch data from the database")?;
    open::that(&link.url).with_context(|| format!("could not open url: {}", link.url))?;
    Ok(())
}
