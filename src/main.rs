mod fns;

use std::fs;
use crate::fns::{post_req, get_req};
use toml::Value;

struct Config {
    url: String,
    token: String
}

fn main(){
    let config = fs::read_to_string("config.toml")
        .expect("Error while finding file!")
        .parse::<Value>()
        .expect("Error while parsing config!");

    let what_commit = get_req("http://whatthecommit.com/index.txt");

    post_req(config["url"].as_str().unwrap(), what_commit);
}
