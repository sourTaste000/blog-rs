mod fns;

use std::fs;
use crate::fns::{post_req, get_req, auth_post_req};
use toml::Value;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use ureq::post;

#[derive(Serialize, Deserialize)]
struct PasteDotEE {
    description: String,
    sections: [Sections; 1]
}

#[derive(Serialize, Deserialize)]
struct Sections {
    name: String,
    syntax: String,
    contents: String
}

fn main(){
    let config = fs::read_to_string("config.toml")
        .expect("Error while finding file!")
        .parse::<Value>()
        .expect("Error while parsing config!");

    let array_of_sections: [Sections; 1] = [Sections {name: "test".to_string(), syntax: "autodetect".to_string(), contents: "foo".to_string()}];
    let paste: PasteDotEE = PasteDotEE {description: "test case of rust kek".to_string(), sections: array_of_sections};

    // // post_req(config["url"].as_str().unwrap(),test);
    //
    // println!(get_req("https://httpbin.org/get"));
    println!("{}", serde_json::to_string(&paste).unwrap())
    auth_post_req()
}
