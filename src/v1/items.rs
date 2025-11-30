//Seth Stigall, 28/11/2025
// v1/items.rs

use std::fmt::Debug;

//similar to the dataclasses from python, this will get us some basic functionality for our Id struct
#[derive(Debug, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new(value: &str) -> Self {
        Id(value.to_string())
    }
    
    // Getter for the inner string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

pub trait Item: {
    //these are the abstract methods that each 'Item' will have to implement |
    fn id(&self) -> &Id;          //<----------------------------------------/
    fn title(&self) -> &str;      //<----------------------------------------/
    fn days_allowed(&self) -> u32;//<----------------------------------------/ 

    //This method will always return 5 for every item
    fn borrows_allowed(&self) -> u32 { 5 } 

}

//Create what a Book is, just an id and a title
pub struct Book {
    id: Id,
    title: String,
}

//Then make sure the program knows what to do when you make a book
impl Book {
    pub fn new(id: &str, title: &str) -> Self {
        //whenever we make a new book you gotta make the book
        Book {
            id: Id::new(id),
            title: title.to_string(),
        }
    }
}

//Now we want to make sure that Book is an Item
impl Item for Book {
    //That also means we have to implement those abstract methods
    fn id(&self) -> &Id { &self.id }
    fn title(&self) -> &str { &self.title }
    
    fn days_allowed(&self) -> u32 {
        14
    }
}

//everything said for book above is applicable here
pub struct Dvd {
    id: Id,
    title: String,
}

impl Dvd {
    pub fn new(id: &str, title: &str) -> Self {
        Dvd {
            id: Id::new(id),
            title: title.to_string(),
        }
    }
}

impl Item for Dvd {
    fn id(&self) -> &Id { &self.id }
    fn title(&self) -> &str { &self.title }
    
    fn days_allowed(&self) -> u32 {
        7
    }
}


//Testing for our code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_days_allowed() {
        let book = Book::new("B1", "Test Book");
        assert_eq!(book.days_allowed(), 14);
    }

    #[test]
    fn test_dvd_days_allowed() {
        let dvd = Dvd::new("D1", "Test DVD");
        assert_eq!(dvd.days_allowed(), 7);
    }
    
    #[test]
    fn test_default_borrows_allowed() {
        let book = Book::new("B1", "Test Book");
        assert_eq!(book.borrows_allowed(), 5);
        let dvd = Dvd::new("D1", "Test DVD");
        assert_eq!(dvd.borrows_allowed(), 5);
    }
}
