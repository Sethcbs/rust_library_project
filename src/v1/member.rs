// Seth Stigall, 29/11/2025
// v1/member.rs

pub struct Member {
    name: String,
    //Store borrowed item IDs as a vector of Strings
    borrowed_ids: Vec<String>,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Member {
            name: name.to_string(),
            borrowed_ids: Vec::new(),
        }
    }

    //Getter for name
    pub fn name(&self) -> &str {
        &self.name
    }

    //Getter for the vector of borrowed IDs
    pub fn borrowed_ids(&self) -> &Vec<String> {
        &self.borrowed_ids
    }
    
    // Borrow method 
    //Just return an error or ok message since you don't return any other values
    pub fn borrow(&mut self, item_id: &str) -> Result<(), String> {
    //I tried to figure out how to use Item::borrows_allowed, or a const MAX_BORROWS, but couldn't get it to work
        let max_borrows = 5;
        let current_borrows = self.borrowed_ids.len();// as u32;

        if current_borrows >= max_borrows {
            return Err(format!("You are at the max number of borrows. \nBorrows = {}.", max_borrows));
        }
        
        if self.borrowed_ids.contains(&item_id.to_string()) {
            return Err(format!("You are already borrowing an item with the ID '{}'.", item_id));
        }
        
        self.borrowed_ids.push(item_id.to_string());
        Ok(())
    }

    //Return item method
    //Similar to borrow method, just return error messages
    pub fn return_item(&mut self, item_id: &str) -> Result<(), String> {
        if let Some(index) = self.borrowed_ids.iter().position(|id| id == item_id) {
            self.borrowed_ids.remove(index);
            Ok(())
        } else {
            Err(format!("You don't have any items with the ID '{}'.", item_id))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::v1::catalog::Catalog;
    use crate::v1::items::{Book, Dvd};

    fn setup_sample_catalog() -> Catalog {
        let mut cat = Catalog::new();
        cat.add(Box::new(Book::new("B1", "Rust for Humans"))).unwrap();
        cat.add(Box::new(Book::new("B2", "Pythonic Patterns"))).unwrap();
        cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
        cat
    }

    #[test]
    fn test_name() {
        let m = Member::new("Y");
        let m2 = Member::new("X");
        assert_eq!(m.name(), "Y");
        assert_eq!(m2.name(), "X");
    }

    #[test]
    fn test_member_borrow_and_return() {
        let mut m = Member::new("Clark");
        
        m.borrow("B1").unwrap();
        assert_eq!(m.borrowed_ids(), &["B1".to_string()]);
        
        m.return_item("B1").unwrap();
        assert_eq!(m.borrowed_ids(), &Vec::<String>::new());
    }

    #[test]
    fn test_member_cannot_borrow_twice() {
        let mut m = Member::new("X");
        m.borrow("K1").unwrap();
        
        //The second borrow should return an error
        let result = m.borrow("K1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already borrowing an item"));
    }

    #[test]
    fn test_member_limit_enforced() {
        let mut m = Member::new("X");
        
        //Borrow up to the limit, 5
        for i in 0..5 {
            m.borrow(&format!("K{}", i)).unwrap();
        }
        
        //The 6th borrow should return an error
        let result = m.borrow("K5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("max number of borrows"));
    }
    
    #[test]
    fn test_unborrowed_ids() {
        let mut m = Member::new("Y");
        m.borrow("B2").unwrap();
        
        //Attempting to return an item not borrowed should return an Err
        let result = m.return_item("K5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("don't have any items"));
    }
    
    //Test that combines member state with catalog details
    #[test]
    fn test_details_lines() {
        let cat = setup_sample_catalog();
        let mut m = Member::new("Y");
        m.borrow("B2").unwrap();
        
        //Get the details from the catalog based on member's borrowed IDs
        
        let lines = cat.get_item_details(m.borrowed_ids());
        
        assert_eq!(lines.len(), 1);
        
        let book_title = String::from("Pythonic Patterns");
        let book_borrow = String::from("(14)");
        
        // Check the specific data points retrieved from the Catalog's get_item_details method
        assert!(lines[0].contains(&book_title));
        assert!(lines[0].contains(&book_borrow));
    }
}
