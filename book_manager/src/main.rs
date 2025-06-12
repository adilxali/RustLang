mod library;
use crate::library::Library;
use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    let mut library = Library::new();

    loop {
        println!("\n====== 📘 LIBRARY MENU ======");
        println!("1. Add Book");
        println!("2. List Books");
        println!("3. Search Book");
        println!("4. Remove Book");
        println!("5. Exit");

        let choice = read_input("Enter your choice: ");

        let choice = choice.trim();

        match choice {
            "1" => {
                let title = read_input("Enter book title : ");
                let author = read_input("Enter author name : ");
                library.add_book(&title, &author);
                println!("\n\"{}\" Book Added Successfully!", title);
            },
            "2" => library.books_list(),
            "3" => {
                let keyword = read_input("Please enter keyword: ");
                library.search_book(&keyword);
            },
            "4" => library.remove_last(),
            "5"=> {
                println!("Exiting...");
                break
            },

            _ => println!("Invalid choice"),
        }
    }
}
