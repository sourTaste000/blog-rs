extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use std::sync::{Arc, Mutex};

use iron::prelude::*;
use iron::status;
use mime::SubLevel::Plain;
use router::Router;
use serde::{Deserialize, Serialize};
use urlencoded::UrlEncodedBody;


#[derive(Serialize, Deserialize)]
struct Post {
    name: String,
    description: String,
    contents: String,
}

#[derive(Clone)]
struct Context {
    posts: Arc<Mutex<Vec<Post>>>
}

fn main() {
    let posts: Vec<Post> = vec![Post {
        name: "First post!!".to_string(),
        description: "First post!!".to_string(),
        contents: "First post!!".to_string(),
    }];

    let context = Context {
        posts: Arc::new(Mutex::new(posts))
    };

    let mut router = Router::new();
    {
        let context = context.clone();
        router.get("/", move |request: &mut Request| latest_post(request, &context), "root");
    }
    {
        let context = context.clone();
        router.post("/create", move |request: &mut Request| create_post(request, &context), "create");
    }
    {
        let context = context.clone();
        router.get("/all", move |request: &mut Request| view_posts(request, &context), "all");
    }

    Iron::new(router).http("localhost:3000").unwrap();
}

fn latest_post(_request: &mut Request, context: &Context) -> IronResult<Response> {
    let mut response = Response::new();
    let posts = &context.posts.lock().unwrap();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let last_post = posts.last().unwrap();
    response.set_mut(format!(r#"
        <p>
            Latest post:
            Title: {}
            Description: {}
            Content: {}
        </p>
    "#, last_post.name, last_post.description, last_post.name));
    return Ok(response);
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

    println!("{}", serialized_json);

    response.set_mut(status::Created);
    ctx_posts.push(serde_json::from_str(&serialized_json).unwrap());
    response.set_mut(mime!(Text/Plain; Charset=Utf8));
    response.set_mut(format!("Post Created! Post: {}", serialized_json));

    return Ok(response);
}

fn view_posts(reg: &mut Request, context: &Context) -> IronResult<Response> {
    let mut response = Response::new();
    let mut ctx_posts = context.posts.lock().unwrap();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Plain; Charset=Utf8));
    response.set_mut(serde_json::to_string(&ctx_posts)).unwrap();
    return Ok(response);
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