extern crate ssh2; // could potentially use libssh2-sys in future

use std::io::stdin;
use std::io::Read;
use ssh2::Session;
use std::net::TcpStream;
use std::collections::HashMap;

mod linter;

// represents a connected user
struct User {
    name: String,
    password: String,
    data: HashMap<String, linter::Entry> // probably inefficient and dumb
}

fn main() {

    // block needs to be in this function - possibly for scope issues
    let tcp = TcpStream::connect("pi.theshmurph.com:22").unwrap();
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
        "help" => help(),
        "login" => login(session, command[1]),
        "create" => println!("unimplemented user creation. sorry! :("),
        "delete" => println!("unimplemented user deletion. sorry! :("),
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
fn help() {
    print!("\nVALID COMMANDS:
    'login'
    'create'
    'delete'
    'exit'\n");
}

// logs a user in if they exist, allows for user creation if not
fn login(session: &Session, user: &str) {
    if exists(session, user) { // no security thus far for login, add please
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
    let a: Vec<&str> = s.trim().split('\n').collect();
    return a.contains(&user)
}

fn start_user(session: &Session, user: &str) {
    let data = get_data_for(session, user);
    let new_user = User::new(session, user);
    loop {
        let mut temp = String::new();
        new_user.parse_command_user(&session, read_next(&mut temp));
    }
}

impl User {
    fn new(session: &Session, user: &str) -> User {
        User { name: user.to_string(), password: "none".to_string(), data: get_data_for(session, user) }
    }
    fn parse_command_user(&self, session: &Session, command: Vec<&str>) {
        match command[0] {
            "find" => find(command[1], &self.data), // try to make OO if it makes sense // also, extend funtionality to include multiple searches per command (not just limited to command[1])
            "insert" => println!("unimplemented entry insertion. sorry! :("),
            "remove" => println!("unimplemented entry remove. sorry! :("),
            "exit" => std::process::exit(0),
            _ => println!("unknown command '{}'", command.join(" "))
        }
    }
}

// return data for user
fn get_data_for(sess: &Session, user: &str) -> HashMap<String, linter::Entry> { // not very open to other users - actually not at all
    let mut channel = sess.channel_session().unwrap();
    channel.exec("cat directory/pass.txt").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    linter::map(s.to_string())
}

fn find(command: &str, hash: &HashMap<String, linter::Entry>) {
    //println!("{}", command.len());
    //println!("{}", command);
    match hash.get(command) {
        Some(entry) => println!("\nid: {}\n name: {}\n pass: {}\n", entry.id, entry.name, entry.pass),
        None => println!("No entry found for that ID.")
    }
    /* // get whenever multiple searches are supported
    match command.len() {
        1 => {
            match hash.get(command) {
                Some(entry) => println!("id: {}\n name: {}\n pass: {}\n", entry.id, entry.name, entry.pass),
                None => println!("No entry found for that ID.")
            }
        }
        _ => println!("unexpected command after 'find'. try again.")
    }*/
}