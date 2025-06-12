#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
}
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add_book(&mut self, title:&str, author:&str) {
        let book = Book {
            title: title.to_owned(),
            author: author.to_owned(),
        };
        self.books.push(book);
    }

    pub fn books_list(&self){
        if self.books.is_empty(){
            println!("📭 No books in the library.");
        }else {
            println!("📚 Books in the library:");
            for (i, book) in self.books.iter().enumerate() {
                println!("{} - \"{}\" by {}", i+1, book.title, book.author);
            }
        }
    }

    pub fn search_book(&self, keyword:&str){
        let mut found:bool = false;

        for book in &self.books {
            if book.title.contains(keyword) || book.author.contains(keyword){
                println!("Found : \"{}\" by \"{}\"", book.title, book.author);
                found = true;
            }
        }
        if !found {
            println!("No book found in Library for \"{}\"", keyword);
        }

    }

    pub fn remove_last(&mut self) {
        if let Some(book) = self.books.pop() {
            println!("Removed: \"{}\" by {}", book.title, book.author);
        } else {
            println!("No books to remove.");
        }
    }
}
