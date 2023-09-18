use crate::{book::Book, person::Person};

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn checkout_book(&mut self, title: String, borrower: Person) -> Result<&Book, String> {
        let book = match self.books.iter_mut().find(|b| b.title == title) {
            Some(b) => b,
            None => return Err(String::from("Book not found")),
        };
        if book.is_available {
            print!("{} is now checked out by {}\n", book.title, borrower.name);
            book.borrow_book(borrower);
            Ok(book)
        } else {
            Err(String::from("Book not available"))
        }
    }

    pub fn return_book(&mut self, title: String) -> Result<&Book, String> {
        let book = match self.books.iter_mut().find(|b| b.title == title) {
            Some(b) => b,
            None => return Err(String::from("Book not found")),
        };

        if !book.is_available {
            book.return_book();
            Ok(book)
        } else {
            Err(String::from("Book already available"))
        }
    }


    pub fn list_all_available_books(&self) {
        println!("Available Books:");
        for book in self.books.iter().filter(|b| b.is_available) {
            println!("{}", book);
        }
    }

    pub fn list_all_checked_out_books(&self) {
        println!("Checked out Books:");
        for book in self.books.iter().filter(|b| !b.is_available) {
            println!("{}", book);
            println!("Borrower: {}", book.borrower.as_ref().unwrap());
        }
    }
}

