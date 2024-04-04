use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use crate::link::Link;
use anyhow::Context;
use clap::{Args, ValueHint};

use crate::Ctx;

// TODO: add support for different file format (specially the browser html/csv bookmark format)

#[derive(Args)]
pub(crate) struct Arguments {
    /// File to import. use '-' for stdin
    #[clap(value_hint = ValueHint::FilePath)]
    file: PathBuf,
}

pub(crate) async fn action(ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let reader: Box<dyn BufRead> = if args.file.to_str().unwrap() == "-" {
        Box::new(io::stdin().lock())
    } else {
        let file = File::open(&args.file).unwrap();
        Box::new(io::BufReader::new(file))
    };

    let mut tx = ctx
        .db
        .begin()
        .await
        .context("could not initialize a new transaction")?;

    let links: Vec<Link> = serde_json::from_reader(reader).context("deserialization error")?;
    for link in links {
        link.insert(&mut *tx)
            .await
            .context("could not insert the link to transaction")?;
    }

    tx.commit()
        .await
        .context("could not commit the transaction")?;

    Ok(())
}
