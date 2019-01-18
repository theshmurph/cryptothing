use std::collections::HashMap;

pub struct Entry {
    pub id: String,
    pub name: String,
    pub pass: String,
}

pub fn map(data: String) -> HashMap<String, Entry> {
    let mut hash: HashMap<String, Entry> = HashMap::new();
    let entries = entrify(data);
    for i in entries {
        hash.insert(i.id.to_string(), i); // works because i is no longer being used twice - 'to_string()' makes a new string and not the string from i
    }
    hash
}

// could be replaced with bufreader, and may very well be done
// EDIT: cannot be done w/ bufreader or bufread until sftp is figured out
fn entrify(data: String) -> Vec<Entry> {
    enum EntryState { Id, Name, Pass, New }

    let mut state = EntryState::Id;
    let mut temp_ent = Entry::new();

    let mut entries = Vec::new();

    for i in data.lines() {
        match state {
            EntryState::Id => {
                temp_ent.id = i.to_string();
                state = EntryState::Name;
            }
            EntryState::Name => {
                temp_ent.name = i.to_string();
                state = EntryState::Pass;
            }
            EntryState::Pass => {
                temp_ent.pass = i.to_string();
                state = EntryState::New;
            }
            EntryState::New => {
                entries.push(temp_ent);
                temp_ent = Entry::new();
                state = EntryState::Id;
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