use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

use sysinfo::System;

use std::env;

#[derive(Parser)]
#[command(name = "async-cli")]
#[command(version = "0.1")]
#[command(about = "async-cli", long_about = None)]
struct Cli {
    // command
    #[command(subcommand)]
    command: Option<Actions>,
}

#[derive(Subcommand)]
enum Actions {
    /// Spawn main process
    Start {
        /// flags
        #[arg(long)]
        my_flag: String,
    },
    /// Stop the service
    Stop {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Actions::Start { my_flag }) => {
            start(&my_flag);
        }
        Some(Actions::Stop {}) => stop(),
        None => {}
    }
}

fn start(_my_flag: &String) {
    // runtime configuration
    let runtime = build_runtime().unwrap();

    // spawn 1
    runtime.spawn(async move { todo!("task 1") });

    // along with other ways
    runtime.block_on(async move { todo!("N task") })

    // further we can use daemonize
}

// TODO handle this more gracefully
fn stop() {
    let sys = System::new_all();
    for (_pid, process) in sys.processes() {
        if process.name() == "async-cli" {
            if process.cmd()[1] == "start" {
                process.kill();
            }
        }
    }
}

fn build_runtime() -> Result<Runtime, std::io::Error> {
    Runtime::new()
}
