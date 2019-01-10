use std::collections::HashMap;

struct Entry {
    id: String,
    name: String,
    pass: String,
}

pub fn map(data: String) {
    let mut hash = HashMap::new();
    for i in entrify(data) {

    }
    for i in data.chars() {
        
    }
}

fn entrify(data: String) -> Vec<Entry> {
    let mut entries = Vec::new();
    for i in data.chars() {
        
    }
    entries
}

impl Entry {
    fn new() -> Entry {
        Entry { 
            id: String::from(""),
            name: String::from(""),
            pass: String::from("")
        }
    }
}