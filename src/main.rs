extern crate ssh2;

use std::io::stdin;
use std::io::Read;
use ssh2::Session;
use std::net::TcpStream;
use std::collections::HashMap;

mod linter;

fn main() {

    let tcp = TcpStream::connect("pi.theshmurph.com:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("main", "BRMurphy35").unwrap();

    if sess.authenticated() {
        println!("session connected!");
        println!("enter user token: ");

        let mut t = String::new();
        let token = read_next(&mut t);
        let data: HashMap<String, linter::Entry>; // probably inefficient and dumb
        // put in something to auth token
        if exists(&token) {
            data = get_data_for(sess, token);

            // terminal
            loop {
                let mut s = String::new();
                parse_command(read_next(&mut s), &data);
            }
        } else {
            println!("user does not exist.");
        }
    } else {
        println!("unable to connect. terminating...")
    }

}

fn read_next(buf: &mut String) -> Vec<&str> {
    stdin().read_line(buf).unwrap();
    buf.trim().split(" ").collect()
}
// return data for valid user, or fails
fn get_data_for(sess: Session, token: Vec<&str>) -> HashMap<String, linter::Entry> {
    let mut channel = sess.channel_session().unwrap();
    channel.exec("cat directory/pass.txt").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    linter::map(s.to_string())
}

// could very well be replaced by get_data_for having a little more logic
fn exists(token: &Vec<&str>) -> bool {
    true
    /*
    let mut channel = sess.channel_session().unwrap();
    channel.exec("cat directory/users.txt").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();

    let m = linter::map(s.to_string());
    if */
}

fn parse_command(command: Vec<&str>, data: &HashMap<String, linter::Entry>) {
    match command[0] {
        "find" => find(command[1], data), // try to make OO if it makes sense // also, extend funtionality to include multiple searches per command (not just limited to command[1])
        "exit" => std::process::exit(0),
        _ => println!("unknown command '{}'", command.join(" "))
    }
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
