mod account;
use crate::account::Accounts;
use std::io::{self, Write};
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    println!("🏦 Welcome to Rust Bank CLI!");
    let mut accounts = Accounts::new();
    loop {
        println!("\n====== 💳 BANKING CLI MENU ======");
        println!("1️⃣  Create Account");
        println!("2️⃣  Show Account Details");
        println!("3️⃣  🚪 Exit");
        println!("==================================");

        let choice = read_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let name = read_input("Enter account holder's name: ");
                let email = read_input("Enter email address: ");
                let phone: u32 = read_input("Enter phone number: ").parse().unwrap_or(0);
                let address = read_input("Enter address: ");
                accounts.create_new_account(&name, &email, phone, &address);
            }
            "2" => {
                accounts.show_account_details();
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
