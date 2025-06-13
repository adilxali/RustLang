mod account;
mod menu;

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
        menu::display_menu();

        let choice = read_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let name = read_input("Enter account holder's name: ");
                let email = read_input("Enter email address: ");

                let phone: String = loop {
                    let input = read_input("Enter phone number: ");
                    if !input.chars().all(|c| c.is_numeric()) {
                        println!("⚠️ Invalid phone number. Try again.");
                        continue;
                    }
                    break input;
                };

                let phone: u64 = phone.parse().unwrap_or(0);

                let address = read_input("Enter address: ");
                accounts.create_new_account(&name, &email, phone, &address);
            }
            "2" => {
                accounts.show_account_details();
            }
            "3" => {
               let account_id = read_input("Enter the Account ID : ");
                accounts.show_transactions(&account_id);
            }
            "4" => {
                let account_id = read_input( "Enter the Account ID to show balance: ");
                accounts.show_balance(&account_id);
            }
            "5" => {
                let account_id = read_input("Enter the Account ID to deposit: ");
                let amount: f64 = loop {
                    let input = read_input("Enter amount to deposit: ");
                    match input.parse::<f64>() {
                        Ok(num) if num > 0.0 => break num,
                        _ => println!("⚠️ Invalid amount. Please enter a positive number."),
                    }
                };
                accounts.deposit(&account_id, amount);
            }
            "6" => {
                let account_id = read_input("Enter the Account ID to withdraw: ");
                let amount: f64 = loop {
                    let input = read_input("Enter amount to withdraw: ");
                    match input.parse::<f64>() {
                        Ok(num) if num > 0.0 => break num,
                        _ => println!("⚠️ Invalid amount. Please enter a positive number."),
                    }
                };
                accounts.withdraw(&account_id, amount);
            }
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
