use std::fmt::Display;

use crate::person::Person;

pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool,
    pub borrower: Option<Person>,
}

impl Display for Book {
    fn fmt(&self, 
        f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Title: {}, Author: {}, Available: {}", self.title, self.author, self.is_available)
    }
}

impl Book {
    pub fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            is_available: true,
            borrower: None,
        }
    }

    pub fn borrow_book(&mut self, borrower: Person) {
        self.is_available = false;
        self.borrower = Some(borrower);
    }

    pub fn return_book(&mut self) {
        self.is_available = true;
        self.borrower = None;
    }
}