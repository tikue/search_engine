#[macro_use]
extern crate lazy_static;

extern crate bodyparser;
extern crate inverted_index;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;
extern crate urlencoded;

use iron::status;
use iron::headers::AccessControlAllowOrigin;
use iron::modifiers::Header;
use iron::prelude::*;
use mount::Mount;
use router::Router;
use staticfile::Static;
use urlencoded::UrlEncodedQuery;
use rustc_serialize::json;
use inverted_index::{Document, InvertedIndex};

use std::path::Path;
use std::sync::RwLock;

lazy_static! {
    static ref INDEX: RwLock<InvertedIndex> = RwLock::new(InvertedIndex::new());
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("web/")));
    mount.mount("/static/css/search.css", Static::new(Path::new("web/search.css")));
    mount.mount("/static/js/", Static::new(Path::new("web/js/")));

    let mut router = Router::new();
    router.get("/", search);
    mount.mount("/search", router);

    let mut router = Router::new();
    router.post("/", index);
    mount.mount("/index", router);

    Iron::new(mount).http("localhost:3000").unwrap();
}

fn search(req: &mut Request) -> IronResult<Response> {
    match req.get::<UrlEncodedQuery>() {
        Ok(query) => match query.get("q") {
            Some(query) if query.len() == 1 => {
                let results = INDEX.read().unwrap().search(&query[0]);
                Ok(Response::with((
                            status::Ok, 
                            json::encode(&results).unwrap(), 
                            Header(AccessControlAllowOrigin::Any))))
            }
            _ => Err(IronError::new(Failed, "Provide exactly one 'q' param")),
        },
        _ => Err(IronError::new(Failed, "Provide exactly one 'q' param")),
    }
}

fn index(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Struct<Document>>();
    match json_body {
        Ok(Some(doc)) => {
            let json = json::encode(&doc).unwrap();
            INDEX.write().unwrap().index(doc);
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
