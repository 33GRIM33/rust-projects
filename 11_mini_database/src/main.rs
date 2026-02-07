use std::collections::HashMap;

// PHASE 1: BASIC IN-MEMORY STORE
// Goal: Build a simple key-value store that works in memory

struct Database {
    // TODO: Add a HashMap<String, String> field to store data
    // Suggested name: store, data, or storage
    store: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    fn list_all(&self) {
        for (key, value) in self.store.iter() {
            println!("{}={}", key, value);
        }
    }

    fn write_to_json(&self, filename: &str){
        
    }

}

fn main() {
    let mut db = Database::new();
    db.set("name1".to_string(),"alpha".to_string());
    db.set("name2".to_string(),"beta".to_string());
    db.set("name3".to_string(),"gamma".to_string());

    //get returns a option therefore we have to do this match statement
    match db.get("name1"){
        Some(v) => println!("name1 = {}", v),
        None => println!("not found"),
    }
    println!("Printing all the values ");
    db.list_all();

    db.delete("name1");

    // CHECKPOINT: Once all TODOs done, you should be able to:
    // - Create database
    // - Add multiple key-value pairs
    // - Retrieve values
    // - Delete values
    // - List all entries
}
