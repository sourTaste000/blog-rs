extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate mime;

#[derive(Serialize, Deserialize)]
struct Post {
    name: String,
    description: String,
    contents: String
}

static mut POSTS: Vec<Post> = vec![Post {
    name: "First post!!".to_string(),
    description: "First post!!".to_string(),
    contents: "First post!!".to_string()
}];


fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/create", create_post, "create");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    unsafe {
        response.set_mut(format!(r#"
        <title>Command Interface</title>
        <form action="/create" method="post">
          <input type="text" name="new_post"> </input>
          <button type="submit">Create!</button>
        </form>
        <p>
        Latest post:
        Title: {}
        Description: {}
        Content: {}
        </p>
    "#, POSTS[POSTS.len()].name, POSTS[POSTS.len()].description, POSTS[POSTS.len()].contents));
    }
    return Ok(response)
}

fn create_post(req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = req.get_ref::<UrlEncodedBody>().unwrap();

    let posts = form_data.get("new_post").unwrap();

    let mut serialized_json: String = "".to_owned();
    for i in posts {
        serialized_json.push_str(format!("{} ", i).as_str())
    }

    response.set_mut(status::Accepted);
    unsafe { POSTS.push(serde_json::from_str(&serialized_json).unwrap()); }
    response.set_mut(mime!(Text/Plain; Charset=Utf8));
    response.set_mut(format!("Post Created! Post: {}", serialized_json));

    return Ok(response)
}

/*
 {
        Err(err) => {
            response.set_mut(status::BadRequest);
            response.set_mut(mime!(Text/Plain; Charset=Utf8));
            response.set_mut(format!("{}", err))
        }
        Ok(ok) => {
            response.set_mut(status::Continue);
            response.set_mut(mime!(Text/Plain; Charset=Utf8));
            response.set_mut(format!("Processing: {}", ok))
        }
    };
 */