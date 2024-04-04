use anyhow::Context;
use clap::Args;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) async fn action(ctx: Ctx, _args: Arguments) -> anyhow::Result<()> {
    sqlx::query(include_str!("../../sql/initdb.sql"))
        .execute(&ctx.db)
        .await
        .context("could not initialize the database")?;

    Ok(())
}
