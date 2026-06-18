enum Command {
    Test,
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();

    let _cmd = match argv[1].as_str() {
        "test" => Command::Test,
        _ => return,
    };

    println!("Appears if cmd = test");
}
