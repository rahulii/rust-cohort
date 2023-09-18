use crate::book::Book;
use crate::library::Library;
use crate::person::Person;

mod person;
mod book;
mod library;

fn main() {
   
    let p1 = Person::new(String::from("John"), 32);
    p1.display();

    let b1 = Book::new(String::from("The Lord of the Rings"), String::from("J.R.R. Tolkien"));
    let b2 = Book::new(String::from("A Song of Ice and Fire"), String::from("George R.R. Martin"));

    let mut lib = Library::new();
    lib.add_book(b1);
    lib.add_book(b2);

    lib.list_all_available_books();

    let _b3 = lib.checkout_book(String::from("The Lord of the Rings"), p1).unwrap();

    // checked out books should include The Lord of the Rings and borrower should be John
    lib.list_all_checked_out_books();

    // available books should not include The Lord of the Rings
    lib.list_all_available_books();

    println!("Returning book");
    lib.return_book(String::from("The Lord of the Rings")).unwrap();

    lib.list_all_available_books();
}
