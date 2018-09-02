use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Key {
    offset: u32
}

fn main() {
    //let data = read("example.txt");
    let key = read("key.txt");
    match get_key(key) {
        Some(k) => {
            println!("all good");
            encrypt("example.txt");
        } None => {
            println!("not good");
        }
    }

}

fn read(filename: &str) -> String {
    let mut data = String::new();
    File::open(filename).expect("file not found")
        .read_to_string(&mut data).expect("something went wrong");
    data
}

fn get_key(key: String) -> Option<Key> {
    let keys = File::open("keys.txt").unwrap();
    let reader = BufReader::new(keys);
    for line in reader.lines() {
        if line.unwrap() == key {
            return Some(Key::new(key))
        } else {
            println!("no")
        }
    }
    None
}

/*
fn decrypt(filename: String) {

}*/


fn encrypt(filename: &str) {
    let mut data = File::open("example.txt").unwrap();
    let mut reader = BufReader::new(data);
    for line in reader.lines() {
        for c in line.unwrap().chars() {
            println!("{}", c);
        }
    }
}

impl Key {

    fn new(key: String) -> Key {
        let mut degree: u32 = 0;
        for k in key.chars() {
            // VERY BASIC ENCRYPTION: FIX
            degree += k.to_digit(36).unwrap();
        }
        Key {
            offset: degree
        }
    }

}
