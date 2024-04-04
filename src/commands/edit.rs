use std::collections::HashSet;

use crate::link::Link;
use anyhow::Context;
use clap::Args;
use sqlx::Acquire;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    /// Append tags
    #[clap(short, long, default_value = "false")]
    append: bool,

    /// New title
    #[clap(short, long)]
    url: Option<String>,

    /// New title
    #[clap(long)]
    title: Option<String>,

    /// New description
    #[clap(long)]
    desc: Option<String>,

    /// New tags
    #[clap(short, long, value_delimiter = ',')]
    tags: Option<Vec<String>>,

    /// Link ID
    id: u32,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let mut link = Link::get_link_by_id(ctx.db.acquire().await?.acquire().await?, args.id as i32)
        .await
        .context("link with given id doesn't exists")?;

    if let Some(title) = args.title {
        link.title = title;
    }

    if let Some(desc) = args.desc {
        link.description = Some(desc);
    }

    if let Some(url) = args.url {
        let parsed =
            url::Url::parse(&url).with_context(|| format!("could not parse url: {}", url))?;
        if parsed.host_str().is_none() {
            anyhow::bail!("invalid url: {}", url)
        }
        link.url = parsed.as_str().to_string()
    }

    if let Some(tags) = args.tags {
        if args.append {
            link.tags.append(&mut tags.clone());
            link.tags = link
                .tags
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
        } else {
            link.tags = tags;
        }
    }

    link.update(ctx.db.acquire().await?.acquire().await?)
        .await
        .context("could not update the link in the database")?;

    Ok(())
}
