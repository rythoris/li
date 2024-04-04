use std::collections::HashSet;

use crate::link::{Link, PrintFormat};
use anyhow::Context;
use clap::{Args, ValueEnum};

use crate::Ctx;

#[derive(ValueEnum, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FilterBy {
    Title,
    Desc,
}

#[derive(Args)]
pub(crate) struct Arguments {
    /// Fields to filter
    #[clap(short = 'F', long, default_value = "title", value_delimiter = ',')]
    pub filter: Vec<FilterBy>,

    /// Query string
    pub query: Option<String>,

    /// Match using regular expression
    #[clap(short, long)]
    pub regex: bool,

    /// Perform case insensitive matching
    #[clap(short, long)]
    pub ignore_case: bool,

    /// Output format
    #[clap(short, long, default_value = "pretty")]
    pub format: PrintFormat,

    /// Filter by tags
    #[clap(short, long, value_delimiter = ',')]
    pub tags: Vec<String>,

    /// Number of links to output
    #[clap(short, long, default_value = "30")]
    pub limit: u32,
}

trait SqlxValue<'q>: 'q + Send + sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> {}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let mut qb = sqlx::QueryBuilder::new("SELECT * FROM links");

    if args.query.is_some() || !args.tags.is_empty() {
        qb.push(" WHERE ");
    }

    if let Some(query) = &args.query {
        let cmp = if args.regex {
            "~* "
        } else {
            if args.ignore_case {
                "ILIKE "
            } else {
                "LIKE "
            }
        };

        qb.push("(");
        for (index, filter) in args
            .filter
            .into_iter()
            .collect::<HashSet<FilterBy>>()
            .iter()
            .enumerate()
        {
            if index != 0 {
                qb.push(" OR ");
            }
            match filter {
                FilterBy::Title => qb.push("title "),
                FilterBy::Desc => qb.push("description "),
            };
            qb.push(cmp);
            if args.regex {
                qb.push_bind(query);
            } else {
                qb.push(r#"CONCAT('%', "#);
                qb.push_bind(query);
                qb.push(r#", '%')"#);
            };
        }
        qb.push(") ");
    }

    // Add condition for tags if provided
    if !args.tags.is_empty() {
        if args.query.is_some() {
            qb.push(" AND ");
        };
        qb.push("tags @> ARRAY[");
        for tag in args.tags.into_iter().collect::<HashSet<_>>().into_iter() {
            qb.push_bind(tag);
        }
        qb.push("]::text[]");
    }

    qb.push(" LIMIT ");
    qb.push_bind(args.limit as i32);

    let links: Vec<Link> = qb
        .build_query_as()
        .fetch_all(&ctx.db)
        .await
        .context("could not fetch data from the database")?;
    for link in links {
        link.print(&args.format)?;
    }

    Ok(())
}
