use anyhow::Context;
use clap::Args;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) async fn action(ctx: Ctx, _args: Arguments) -> anyhow::Result<()> {
    let tags: Vec<(String, i64)> = sqlx::query_as(
        r#"SELECT DISTINCT tag,
           COUNT(*) AS count
           FROM links, unnest(tags) AS tag
           GROUP BY tag
           ORDER BY count DESC"#,
    )
    .fetch_all(&ctx.db)
    .await
    .context("could not fetch data from the database")?;

    for (tag, count) in tags {
        println!("{count: <7} {tag}");
    }

    Ok(())
}
