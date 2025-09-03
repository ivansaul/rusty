use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct CustomCli {
    /// Sets a custom config file
    #[clap(short, long)]
    pub config: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    #[clap(name = "daemon", about = "Starts the daemon")]
    Daemon(DaemonSubCommand),
}

#[derive(Parser)]
pub struct DaemonSubCommand {}
