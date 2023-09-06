struct Person {
    name: String,
    age:  u32,
}

struct Book {
    title: String,
    author: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}
