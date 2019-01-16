use std::collections::HashMap;

pub struct Entry {
    pub id: String,
    pub name: String,
    pub pass: String,
}

pub fn map(data: String) -> HashMap<String, Entry> {
    let mut hash: HashMap<String, Entry> = HashMap::new();
    for i in entrify(data) {
        hash.insert(i.id.to_string(), i); // works because i is no longer being used twice
    }
    hash
}

// could be replaced with bufreader, and may very well be done
// EDIT: cannot be done w/ bufreader or bufread until sftp is figured out
fn entrify(data: String) -> Vec<Entry> {
    enum EntryState { Id, Name, Pass, New }

    let mut state = EntryState::Id;
    let mut entries = Vec::new();

    let mut temp_str = String::from("");
    let mut temp_ent = Entry::new();

    for i in data.chars() {
        match state {
            EntryState::Id => {
                if i != '\n' {
                    temp_str.push(i);
                } else {
                    temp_ent.id = (&temp_str).to_string();
                    temp_str.clear();
                    state = EntryState::Name;
                }
            } EntryState::Name => {
                if i != '\n' {
                    temp_str.push(i);
                } else {
                    temp_ent.name = (&temp_str).to_string();
                    temp_str.clear();
                    state = EntryState::Pass;
                }
            } EntryState::Pass => {
                if i != '\n' {
                    temp_str.push(i);
                } else {
                    temp_ent.pass = (&temp_str).to_string();
                    temp_str.clear();
                    state = EntryState::New;
                }
            } EntryState::New => {
                if i == '\n' { // not very loose on syntax, but will work for now
                    entries.push(temp_ent);
                    temp_ent = Entry::new();
                    state = EntryState::Id;
                }
            }
        }
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