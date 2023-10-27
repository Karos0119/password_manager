use std::{io, fs, io::Write};
use dialoguer::{Input, Password, Select};
use crossterm::{cursor, terminal::{self, ClearType}, ExecutableCommand, execute};
use rand::Rng;

fn main() {
    let options = ["Save new login", "Delete login", "View logins", "Generate password", "Exit"];
    loop {
        
        let selection = Select::new()
            .with_prompt("\nSelect an option")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                clear_screen();
                save_new_login();
            }
            1 => {
                clear_screen();
                delete_login();
            }
            2 => {
                clear_screen();
                view_logins();
            }
            3 => {
                clear_screen();
                println!("{}", generate_password());
            }
            4 => break,
            _ => eprintln!("Invalid option!"),
        }
    }
    
    clear_screen();
    restore_screen();
    scroll_to_top();
}

fn generate_password() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789\
                          !@#$%^&*()-_=+[{]}<.>?";

    let mut rng = rand::thread_rng();
    let mut password = String::new();

    while password.len() < 16 {
        let index = rng.gen_range(0..charset.len());
        let next_char = charset[index] as char;

        if password.len() > 2 {
            let last = password.chars().last().unwrap();
            let second_last = password.chars().rev().nth(1).unwrap();
            let third_last = password.chars().rev().nth(2).unwrap();

            if next_char != last && next_char != second_last && next_char != third_last {
                password.push(next_char);
            }
        } else {
            password.push(next_char);
        }
    }

    password
}

fn clear_screen() {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn restore_screen() {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();
}

fn scroll_to_top() {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
}

fn save_new_login() {
    let website = Input::<String>::new()
        .with_prompt("Enter website/service")
        .interact()
        .unwrap();

    let login = Password::new()
        .with_prompt("Enter login")
        .interact()
        .unwrap();

    let entry = format!("{}:{}", website, login);
    if let Err(err) = save_login(&entry) {
        eprintln!("Error: {}", err);
    } else {
        clear_screen();
        println!("login saved successfully!");
    }
}

fn delete_login() {
    let mut logins = load_logins().unwrap_or_else(|| Vec::new());

    if logins.is_empty() {
        eprintln!("No logins found.");
        return;
    }

    let selection = Select::new()
        .with_prompt("Select a login to delete")
        .items(&logins)
        .default(0)
        .interact()
        .unwrap();

    if selection < logins.len() {
        let deleted_entry = logins.remove(selection);
        if let Err(err) = save_logins(&logins) {
            eprintln!("Error: {}", err);
        } else {
            clear_screen();
            println!("login deleted: {}", deleted_entry);
        }
    } else {
        eprintln!("Invalid selection.");
    }
}

fn save_logins(logins: &[String]) -> Result<(), std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("logins.txt")?;
    for entry in logins {
        writeln!(file, "{}", entry)?;
    }
    Ok(())
}

fn view_logins() {
    let logins = load_logins().unwrap_or_else(|| Vec::new());
    if logins.is_empty() {
        eprintln!("No logins found.");
        return;
    }

    eprintln!("logins:");
    if let Some(logins) = load_logins() {
        for (index, entry) in logins.iter().enumerate() {
            println!("{}: {}", index + 1, entry);
        }
    }
}

fn save_login(entry: &str) -> Result<(), std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logins.txt")?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

fn load_logins() -> Option<Vec<String>> {
    match fs::read_to_string("logins.txt") {
        Ok(content) => Some(content.lines().map(|s| s.to_string()).collect()),
        Err(_) => {
            eprintln!("No logins found.");
            None
        }
    }
}
