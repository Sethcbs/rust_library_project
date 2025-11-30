//Seth Stigall, 29/11/2025
//v1/catalog.rs

use std::collections::HashMap;
use crate::v1::items::Item;

//There don't seem to be any dictionaries in Rust, so a hash map will have to do.
//Stores items using their Id as the key, and a Box<dyn Item> as the value.
//Box<dyn Item> allows us to store different types that implement Item in the same map.
pub struct Catalog {
    items: HashMap<String, Box<dyn Item>>,
}

impl Catalog {
    //Make sure that our Catalogs can make new Catalogs
    pub fn new() -> Self {
        Catalog {
            items: HashMap::new(),
        }
    }

    //add our new items to the hash map, and return a Result since it's different returning errors in Rust
    pub fn add(&mut self, item: Box<dyn Item>) -> Result<(), String> {
        let id_value = item.id().value().to_string();
        
        //but make sure to check if that item already exists, and skip adding if it does
        if self.items.contains_key(&id_value) {
            return Err(format!("An item with key '{}' is already in the catalog.", id_value));
        }

        self.items.insert(id_value, item);
        Ok(())
    }

    //get our item from the hash map based on the provided id
    pub fn get_item(&self, id: &str) -> Option<&Box<dyn Item>> {
        //returns None if the id does not exist
        //I want to test this, but I'm not sure how
        self.items.get(id)
    }

    pub fn get_item_details(&self, borrowed_ids: &[String]) -> Vec<String> {
        let mut details_vector = Vec::new();
        
        for id_str in borrowed_ids {
            if let Some(item) = self.get_item(id_str) {
                let detail_string = format!("ID: {}, Title: {}, Days Allowed: ({}) days)",
                    item.id().value().to_string(),
                    item.title().to_string(),
                    item.days_allowed(),
                );
                details_vector.push(detail_string);
            }
        }
        details_vector
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::v1::items::{Book, Dvd};
    
    fn setup_sample() -> Catalog {
        let mut cat = Catalog::new();
        cat.add(Box::new(Book::new("B1", "Rust for Humans"))).unwrap();
        cat.add(Box::new(Book::new("B2", "Pythonic Patterns"))).unwrap();
        cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
        cat
    }

    #[test]
    fn test_add_and_get() {
        let cat = setup_sample();
        
        assert_eq!(cat.get_item("B1").unwrap().title(), "Rust for Humans");
        assert_eq!(cat.get_item("D1").unwrap().days_allowed(), 7);
    }

//will possibly bring this test up as a question in class
    /*
    #[test]
    fn test_getting_false_ID() {
        let cat = setup_sample();
        
        let result = cat.get_item("C1");
        assert_eq!(result, None);
    }
    */

    #[test]
    fn test_duplicate_id_rejected() {
        let mut cat = setup_sample();
        let result = cat.add(Box::new(Book::new("B1", "Duplicate")));
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already in the catalog"));
    }
    
    #[test]
    fn test_get_item_details() {
        let cat = setup_sample();
        let borrowed_ids = vec!["B1".to_string(), "D1".to_string()];
        
        let details = cat.get_item_details(&borrowed_ids);
        
        assert_eq!(details.len(), 2);

        assert!( details[0].contains("Rust for Humans"));
        assert!( details[0].contains("(14) days)"));
        assert!( details[1].contains("Taking Flight"));
        assert!( details[1].contains("(7) days)"));
    }
}
