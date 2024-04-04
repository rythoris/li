use anyhow::Context;
use clap::Args;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    /// Link ID
    id: u16,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let res = sqlx::query("DELETE FROM links WHERE id = $1")
        .bind(args.id as i64)
        .execute(&ctx.db)
        .await
        .context("database delete error")?;

    if res.rows_affected() == 0 {
        anyhow::bail!("link not found: {}", args.id)
    };

    Ok(())
}
