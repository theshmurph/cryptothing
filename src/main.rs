extern crate ssh2; // could potentially use libssh2-sys in future
extern crate rpassword;
extern crate ring; // sha256 for now

use std::io::stdin;
use std::io::Read;
use std::str;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::collections::HashMap;

mod helper;

// represents a connected user
struct User {
    name: String,
    password: String,
    data: HashMap<String, helper::Entry>,
    active: bool
}

struct Entry {
    site: String,
    name: String,
    pass: String
}

fn main() {

    let mut file = File::open("./pass.cpt").unwrap();
    let mut buf = BufReader::new(file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    let entries = entrify(contents);
    
    for i in entries.iter() {
        println!("{}", i.site);
        println!("{}", i.name);
        println!("{}", i.pass);
    }
}

fn entrify(contents: String) -> Vec<Entry> {

    enum EntryState { Site, Name, Pass, New }

    let mut state = EntryState::Site;
    let mut entry = Entry::new();
    let mut entries = Vec::new();

    for i in contents.lines() {
    
        match state {
            EntryState::Site => {
                entry.site = i.to_string();
                state = EntryState::Name;
            }
            EntryState::Name => {
                entry.name = i.to_string();
                state = EntryState::Pass;
            }
            EntryState::Pass => {
                entry.pass = i.to_string();
                state = EntryState::New;
            }
            EntryState::New => {
                entries.push(entry);
                entry = Entry::new();
                state = EntryState::Site;
            }
        }
    
    }

    entries

}

impl Entry {
    fn new() -> Entry {
        Entry {
            site: String::from(""),
            name: String::from(""),
            pass: String::from("")
        }
    }
}

/*
// runs the program
fn shell(sess: &Session) {
    println!("\nsession connected!");
    println!("give a command! 'help' for list of commands");

    while sess.authenticated() {
        let mut temp = String::new();
        parse_command_gen(&sess, read_next(&mut temp));
    }
}

// for parsing general menu commands
fn parse_command_gen(session: &Session, command: Option<Vec<&str>>) {
    match command {
        Some(command) => {
            match command[0] {
            "help" => help_gen(),
            "login" => login(session, command),
            "create" => println!("this function would work better with a database!"),
            "delete" => println!("this function would work better with a database!"),
            "exit" => std::process::exit(0),
            _ => println!("unknown command '{}'", command.join(" "))
            }
        }
        None => println!("unable to process command. enter 'help' for valid commands.")
    }
}

// reads from std::io into string
fn read_next(buf: &mut String) -> Option<Vec<&str>> {
    stdin().read_line(buf).unwrap();
    match buf.trim() {
        "" => None,
        _ => Some(buf.trim().split(" ").collect())
    }
}

// help menu
fn help_gen() {
    print!("\nVALID TERMINAL COMMANDS:
    'login' - log in via username
    'create' - create new user --UNIMPLEMENTED
    'delete' - delete a stored user --UNIMPLEMENTED
    'exit' - exit session\n");
}

// logs a user in if they exist, allows for user creation if not
fn login(session: &Session, command: Vec<&str>) {
    match command.len() {
        1 => login_menu(session),
        2 => login_user(session, command[1]),
        _ => println!("unknown command '{}'", command.join(" "))
    }
}

fn login_menu(session: &Session) {
    println!("who would you like to log in as? ('enter' to exit)");
    let mut s = String::new();
    if let Some(s) = read_next(&mut s) {
        if s.len() == 1 {
            login_user(session, &s[0])
        } else {
            println!("invalid command");
        }
    }
}

fn login_user(session: &Session, user: &str) {
    if exists(session, user) {
        println!("\nwelcome {}!", &user);
        start_user(&session, user)
    } else {
        println!("user does not exist. would you like to create that user?"); // unimplemented
    }
}

// returns whether or not a user exists
fn exists(session: &Session, user: &str) -> bool {
    let mut channel = session.channel_session().unwrap();
    channel.exec("cat directory/users.txt").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    let user_pass: Vec<&str> = s.trim().split('\n').collect();

    for i in user_pass {
        let a: Vec<&str> = i.split(" ").collect();
        if a[0] == ec(user) {
            let pass = ec(&rpassword::prompt_password_stdout("enter user password: ").unwrap()); // seems a bit slow, also messy
            if pass == a[1] {
                return true
            }
        }
    }

    false
}

// starts new user session
fn start_user(session: &Session, user: &str) {
    let mut new_user = User::new(session, user);
    while new_user.active {
        let mut temp = String::new();
        parse_command_user(&mut new_user, &session, read_next(&mut temp));
    }
}

// implementation for user struct
impl User {
    fn new(session: &Session, user: &str) -> User {
        User { name: user.to_string(), password: "none".to_string(), data: get_data_for(session, user), active: true }
    }
}

// parses command on user menu
fn parse_command_user(mut user: &mut User, session: &Session, command: Option<Vec<&str>>) {
    let data = get_data_for(session, &(user.name));
    match command {
        Some(command) => {
            match command[0] {
            "help" => help_user(),
            "find" => find(&command[1..], &data), // try to make OO if it makes sense
            "insert" => println!("this function would work better with a database!"),
            "remove" => println!("this function would work better with a database!"),
            "leave" => logout(&mut user),
            _ => println!("unknown command '{}'", command.join(" "))
            }
        }
        None => println!("unable to process command. enter 'help' for valid commands.")
    }
}

// return data for user
fn get_data_for(sess: &Session, user: &str) -> HashMap<String, helper::Entry> {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(format!("cat directory/{}/pass.txt", user).as_ref()).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    helper::map(s.to_string())
}

// help menu for user commands
fn help_user() {
    print!("\nVALID USER COMMANDS:
    'find' - find data for requested sites
    'insert' - create new user --UNIMPLEMENTED
    'remove' - delete a stored user --UNIMPLEMENTED
    'leave' - log out\n");
}

// finds entry in pass.txt file
fn find(command: &[&str], hash: &HashMap<String, helper::Entry>) {
    for i in command {
        match hash.get(&ec(*i)) {
            Some(entry) => println!("\nid: {}\n name: {}\n pass: {}\n", *i, entry.name, entry.pass),
            None => println!("No entry found for that ID.")
        }
    }
}

// logs specified user out
fn logout(user: &mut User) {
    // very basic, should be beefed up in future
    user.active = false;
    println!("logging out {}...", user.name);
    // doesn't clear user data, just sets active to false
}

fn ec(str: &str) -> String {
    let result = digest(&SHA256, str.as_bytes());
    result.as_ref().iter().map(|b| format!("{:x}", b)).collect()
}
*/

