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

fn main() {
    let mut new_game = true;
    clear();
    loop {
        let input;
        if new_game {
            input = prompt("Welcome to the pendu game ! Start ? (y/n) > ");
        } else {
            input = prompt("Restart ? (y/n) > ");
        }
        if input == "y" || input == "o" || input.len() == 0 {
            new_game = false;
            start();
        } else if input == "n" || input == "exit" || input == "quit" || input == "q" {
            break;
        } else {
            clear();
        }
    }
}
