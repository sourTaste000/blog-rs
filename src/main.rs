extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate mime;

#[derive(Serialize, Deserialize)]
struct Post {
    name: String,
    description: String,
    contents: String
}

#[derive(Clone)]
struct Context {
    posts: Arc<Mutex<Vec<Post>>>
}

fn main() {
    let posts: Vec<Post> = vec![Post {
        name: "First post!!".to_string(),
        description: "First post!!".to_string(),
        contents: "First post!!".to_string()
    }];

    let context = Context {
        posts: Arc::new(Mutex::new(posts))
    };

    let mut router = Router::new();
    {
        let context = context.clone();
        router.get("/", move |request: &mut Request| get_form(request, &context), "root");
    }
    {
        let context = context.clone();
        router.post("/create", move |request: &mut Request| create_post(request, &context), "create");
    }

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request, context: &Context) -> IronResult<Response> {
    let mut response = Response::new();
    let posts = &context.posts.lock().unwrap();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let last_post = posts.last().unwrap();
    response.set_mut(format!(r#"
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
    "#, last_post.name, last_post.description, last_post.name));
    return Ok(response)
}

fn create_post(req: &mut Request, context: &Context) -> IronResult<Response> {
    let mut response = Response::new();
    let mut ctx_posts = context.posts.lock().unwrap();

    let form_data = req.get_ref::<UrlEncodedBody>().unwrap();

    let posts = form_data.get("new_post").unwrap();

    let mut serialized_json: String = "".to_owned();

    for i in posts {
        serialized_json.push_str(format!("{} ", i).as_str())
    }

    response.set_mut(status::Accepted);
    ctx_posts.push(serde_json::from_str(&serialized_json).unwrap());
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