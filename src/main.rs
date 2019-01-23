extern crate ssh2; // could potentially use libssh2-sys in future
extern crate rusqlite;

use std::io::stdin;
use std::io::Read;

use std::net::TcpStream;
use std::collections::HashMap;

use ssh2::Session;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};



mod linter;

// represents a connected user
struct User {
    name: String,
    password: String,
    data: HashMap<String, linter::Entry>,
    active: bool
}

fn main() {

    // block needs to be in this function - possibly for scope reasons
    let tcp = TcpStream::connect("theshmurph.com:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("main", "BRMurphy35").unwrap(); // is possibly bad practice

    if sess.authenticated() {
        println!("session connected!");
        println!("give a command! 'help' for list of commands");
        loop {
            let mut temp = String::new();
            parse_command_gen(&sess, read_next(&mut temp));
        }
    } else {
        println!("unable to connect. terminating...")
    }

}

// for parsing general menu commands
fn parse_command_gen(session: &Session, command: Vec<&str>) {
    match command[0] {
        "help" => help_gen(),
        "login" => login(session, command[1]),
        "create" => println!("this function would work better with a database!"),
        "delete" => println!("this function would work better with a database!"),
        "exit" => std::process::exit(0),
        _ => println!("unknown command '{}'", command.join(" "))
    }
}

// reads from std::io into string
fn read_next(buf: &mut String) -> Vec<&str> {
    stdin().read_line(buf).unwrap();
    buf.trim().split(" ").collect()
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
fn login(session: &Session, user: &str) {
    if exists(session, user) {
        println!("welcome {}!", &user);
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
        if a[0] == user {
            println!("enter user password: ");
            let mut s = String::new();
            if read_next(&mut s)[0] == a[1] {
                return true
            }
        }
    }
    println!("incorrect password!");

    return false
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
fn parse_command_user(mut user: &mut User, session: &Session, command: Vec<&str>) {
    let data = get_data_for(session, &(user.name));
    match command[0] {
        "help" => help_user(),
        "find" => find(&command[1..], &data), // try to make OO if it makes sense
        "insert" => println!("this function would work better with a database!"),
        "remove" => println!("this function would work better with a database!"),
        "leave" => logout(&mut user),
        _ => println!("unknown command '{}'", command.join(" "))
    }
}

// return data for user
fn get_data_for(sess: &Session, user: &str) -> HashMap<String, linter::Entry> {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(format!("cat directory/{}/pass.txt", user).as_ref()).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    linter::map(s.to_string())
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
fn find(command: &[&str], hash: &HashMap<String, linter::Entry>) {
    for i in command {
        match hash.get(*i) {
            Some(entry) => println!("\nid: {}\n name: {}\n pass: {}\n", entry.id, entry.name, entry.pass),
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