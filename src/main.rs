use std::fs;
use std::env;

fn main() {

    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()
    //     .await?;
    let contents = fs::read_to_string(&args[0].to_string())
        .expect("Something went wrong reading the file");
    println!("{}", contents)
}
