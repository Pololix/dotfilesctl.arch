use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Deploy,
    Update {
        target: Option<String>,
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
        Cmd::Deploy => {
            println!("clean up whatever writes might have been done by previous deploys");
            println!("set up git globals");
            println!("download recorded packages");
            println!("fecth from github");
            println!("set up symlinks");
        }
        Cmd::Update { target } => match target.as_deref() {
            Some("system") => {
                println!("record installed packages");
                println!("update recorded packages");
            }
            Some("config") => {
                println!("clean up symlinks");
                println!("fecth from github");
                println!("set up symlinks");
            }
            Some(package) => {
                println!("update {}", package);
            }
            None => {
                println!("record installed packages");
                println!("update recorded packages");
                println!("clean up symlinks");
                println!("fecth from github");
                println!("set up symlinks");
            }
        },
        Cmd::Record { print, groups } => {
            if print {
                if groups {
                    println!("print record file with groups");
                } else {
                    println!("print record file raw");
                }
            } else {
                println!("record installed packages");
            }
        }
        Cmd::Group { name, action } => match action {
            Action::Create => {
                println!("create new group inside the record: {}", name);
            }
            Action::Delete => {
                println!("delete a group from the record: {}", name);
            }
            Action::Rename { new } => {
                println!("rename {} to {}", name, new);
            }
            Action::Add { packages } => {
                println!("adding packages to: {}", name);
            }
            Action::Remove { packages } => {
                println!("removing packages from: {}", name);
            }
            Action::Print => {
                println!("printing {} contents", name);
            }
        },
    }
}
