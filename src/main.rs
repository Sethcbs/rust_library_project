//Seth Stigall, 28/11/2025
// main.rs

mod v1;

use v1::items::{Book, Dvd};
use v1::catalog::Catalog;
use v1::member::Member;

fn main() {
    let mut cat = Catalog::new();
    
    cat.add(Box::new(Book::new("B1", "Rust for Humans"))).unwrap();
    cat.add(Box::new(Book::new("B2", "Pythonic Patterns"))).unwrap();
    cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
    cat.add(Box::new(Dvd::new("D2", "Patterns in Motion"))).unwrap();

    let mut m = Member::new("Clark");
    m.borrow("B1").unwrap();
    m.borrow("D1").unwrap();
    
    println!("{} has borrowed: {:?}", m.name(), m.borrowed_ids());
    
    let details = cat.get_item_details(m.borrowed_ids());
    for line in details {
        println!(" * {}", line);
    }
    
    m.return_item("B1").unwrap();
    println!("{} has now borrowed: {:?}", m.name(), m.borrowed_ids());
}
