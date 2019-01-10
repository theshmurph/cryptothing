extern crate ssh2;

use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::collections::HashMap;

mod linter;

fn main() {

    let tcp = TcpStream::connect("pi.theshmurph.com:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("main", "BRMurphy35").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("cat directory/pass.txt").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();

    map_to_struct(s.to_string());

}

fn map_to_struct(data: String) {
    let hash = linter::map(data);
    find("amazon.com", hash);
}

fn find(name: &str, hash: HashMap<String, linter::Entry>) {
    match hash.get(name) {
        Some(entry) => println!("id: {}\n name: {}\n pass: {}\n", entry.id, entry.name, entry.pass),
        None => println!("No entry found for that ID.")
    }
}
