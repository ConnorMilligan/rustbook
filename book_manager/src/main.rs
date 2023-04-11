struct Book {
    title: String,
    author: String,
    year: u32,
}

struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn book_count(&self) -> usize {
        self.books.len()
    }
}

fn main() {
    println!("Hello, world!");
    let mut my_lib = Library {
        name: String::from("Howe Library"),
        books: Vec::new()
    };

    let my_book = Book {
        title: String::from("Dave's book"),
        author: String::from("Dave David"),
        year: 1931
    };

    my_lib.add_book(my_book);
    my_lib.add_book(Book { 
        title: String::from("Test Book"), 
        author: String::from("Test author"), 
        year: 1922
    });
    
    println!("{} has {} books in it!", my_lib.name, my_lib.book_count());

    for book in my_lib.books {
        println!("{}, written by {} in {}", book.title, book.author, book.year);
    }
}
