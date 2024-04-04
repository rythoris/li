use crate::link::Link;
use anyhow::Context;
use clap::{Args, ValueHint};
use sqlx::Acquire;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    /// Link to add
    #[clap(value_hint = ValueHint::Url)]
    link: String,

    /// Link Tags
    #[clap(short, long, value_delimiter = ',')]
    tags: Vec<String>,

    /// Owerwrite title
    #[clap(long)]
    title: Option<String>,

    /// Owerwrite description
    #[clap(long)]
    desc: Option<String>,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let mut link = if let Some(title) = args.title {
        Link::new(&args.link, &title).with_context(|| format!("add: {}", args.link))?
    } else {
        Link::from_url(&args.link)
            .await
            .with_context(|| format!("add: {}", args.link))?
    };

    link.tags = args.tags;
    if let Some(desc) = args.desc {
        link.description = Some(desc)
    }

    let res = link.insert(ctx.db.acquire().await?.acquire().await?).await;

    if let Err(sqlx::Error::Database(e)) = &res {
        if e.is_unique_violation() {
            anyhow::bail!("link already exists")
        }
    }
    res.context("could not insert data into database")?;

    Ok(())
}
