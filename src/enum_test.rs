enum Book {
    Papery(u32),
    Electronic(String),
}

pub fn run() {

    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url..."));

    match ebook {
        Book::Papery { 0: index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { 0: url } => {
            println!("E-book {}", url);
        }
    }
}