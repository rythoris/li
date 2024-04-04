use std::io;

use crate::Cli;
use clap::{Args, CommandFactory};
use clap_complete::Shell;

use crate::Ctx;

#[derive(Args)]
pub(crate) struct Arguments {
    shell: Shell,
}

pub(crate) async fn action(_ctx: Ctx, args: Arguments) -> anyhow::Result<()> {
    let mut command = Cli::command();
    let bin_name = command.get_name().to_string();
    clap_complete::generate(args.shell, &mut command, bin_name, &mut io::stdout());
    Ok(())
}
