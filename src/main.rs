extern crate ssh2;

use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

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
    linter::map(data);

}
