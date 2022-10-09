use std::io::Write;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn clear() {
    if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    } else if cfg!(windows) {
        std::process::Command::new("cls").status().unwrap();
    }
}
