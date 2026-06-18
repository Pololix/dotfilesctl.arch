use clap::{Parser, Subcommand};

#[derive(Parser, Debug)] // so that it can turn cli args to a struct
struct Args {
    #[command(subcommand)] // means that the cmd field has its own parser
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Update, // -Syu + new git fetch or either separately

    Record,  // records currently installed packages
    Install, // -S + record new package
    Remove,  // -R + remove old package
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Command::Update => {
            println!("updating");
        }
        Command::Record => {
            println!("recording");
        }
        Command::Install => {
            println!("installing");
        }
        Command::Remove => {
            println!("removing");
        }
    }
}
