use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Deploy,
    Update {
        target: Option<String>,
    },
    Search {
        #[arg(short, long)]
        with: bool,
        manager: Option<String>,
        packages: Vec<String>,
    },
    Install {
        #[arg(short, long)]
        with: bool,
        #[arg(requires = "with")]
        manager: Option<String>,
        packages: Vec<String>,
    },
    Remove {
        packages: Vec<String>,
    },
    Record {
        #[arg(short, long)]
        print: bool,
        #[arg(short, long, requires = "print")]
        groups: bool,
    },
    Group {
        name: String,
        #[command(subcommand)]
        action: Action,
    },
}

#[derive(Subcommand, Debug)]
enum Action {
    Create,
    Delete,
    Rename { new: String },
    Add { packages: Vec<String> },
    Remove { packages: Vec<String> },
    Print,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Command::Deploy => {
            println!("update");
        }
        Command::Update { target } => {
            println!("update");
        }
        Command::Search {
            with,
            manager,
            packages,
        } => {
            println!("update");
        }
        Command::Install {
            with,
            manager,
            packages,
        } => {
            println!("update");
        }
        Command::Remove { packages } => {
            println!("update");
        }
        Command::Record { print, groups } => {
            println!("update");
        }
        Command::Group { name, action } => {
            println!("update");
        }
    }
}
