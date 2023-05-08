//! `cargo run --example blocking --features=blocking`
extern crate reqwest;
extern crate minidom;

// use minidom::Element;
// use error_chain::error_chain;
// use std::io::Read;


use reqwest::{blocking::Response, Error};


fn main() {
    println!("Hello, world!");
    match parse_xml() {
        Ok(parsed) => print!("{}", parsed.text().unwrap()),
        Err(e) => panic!("{}", e),
    }
}   

fn parse_xml() -> Result<Response, Error> {
    let arch_news_url = String::from("https://archlinux.org/feeds/news/");
    let res = reqwest::blocking::get(arch_news_url);
    return res;
}



