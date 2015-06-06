extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate bodyparser;
extern crate persistent;
extern crate inverted_index;
extern crate rustc_serialize;

use iron::status;
use iron::headers::AccessControlAllowOrigin;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::typemap::Key;
use persistent::State;
use router::Router;
use urlencoded::UrlEncodedQuery;
use rustc_serialize::json;
use inverted_index::{Document, InvertedIndex};

pub struct IndexHolder;
impl Key for IndexHolder { type Value = InvertedIndex; }

fn main() {
    let mut router = Router::new();
    router.get("/", hello);
    router.get("/search", search);
    router.post("/index", index);
    
    let mut chain = Chain::new(router);
    chain.link(State::<IndexHolder>::both(InvertedIndex::new()));
    Iron::new(chain).http("localhost:3000").unwrap();

}

fn hello(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello!")))
}

fn search(req: &mut Request) -> IronResult<Response> {
    let query = req.get::<UrlEncodedQuery>().unwrap();
    match query.get("q") {
        Some(query) if query.len() == 1 => {
            let rwlock = req.get::<State<IndexHolder>>().unwrap();
            let results: Vec<_> = rwlock.read().unwrap().search(&query[0]);
            Ok(Response::with((
                        status::Ok, 
                        json::encode(&results).unwrap(), 
                        Header(AccessControlAllowOrigin::Any))))
        }
        _ => Err(IronError::new(Failed, "Provide exactly one 'q' param")),
    }
}

fn index(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Struct<Document>>();
    match json_body {
        Ok(Some(doc)) => {
            let rwlock = req.get::<State<IndexHolder>>().unwrap();
            let mut search_index = rwlock.write().unwrap();
            let json = json::encode(&doc).unwrap();
            search_index.index(doc);
            Ok(Response::with((status::Ok, json)))
        },
        Ok(None) => Err(IronError::new(Failed, "Provide a document with an id and content.")),
        Err(err) => Err(IronError::new(err, "Provide a document with an id and content.")),
    }
}

#[derive(Debug)]
pub struct Failed;

impl std::error::Error for Failed {
    fn description(&self) -> &str {
        "Something went wrong"
    }

    fn cause(&self) -> Option<&std::error::Error> { None }
}
impl std::fmt::Display for Failed {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(fmt, "Incorrect number of 'q' params")
        }
}
