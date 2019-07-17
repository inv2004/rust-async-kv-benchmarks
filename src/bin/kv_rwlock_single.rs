#![feature(async_await)]
extern crate futures;
extern crate hyper;

use std::collections::HashMap;
use futures::{FutureExt, TryFutureExt};
use futures::compat::{Future01CompatExt};
use hyper::{Body, Request, Response, Server, service::service_fn, Method, rt::Stream};
use std::net::SocketAddr;
use tokio::runtime::current_thread::Runtime;
use std::sync::{Arc, RwLock};

type Map = HashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

async fn start_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let cache = Arc::new(RwLock::new(HashMap::with_capacity(INITIAL_CAPACITY)));

    let start_fut = Server::bind(&addr).serve(move || {
        let cache = cache.clone();
        service_fn(move |req| serve_req(req, cache.clone()).boxed().compat())
    } );

    if let Err(e) = start_fut.compat().await {
        eprintln!("server error: {}", e);
    }
}

async fn serve_req(req: Request<Body>, cache: Arc<RwLock<Map>>) -> Result<Response<Body>, hyper::Error> {
    let m = req.method();
    match *m {
        Method::GET => {
            let body = req.into_body().concat2().compat().await?;
            let k = String::from_utf8(body.to_vec()).unwrap();

//            println!("get {}", k);

            let cache = cache.read().unwrap();
            if let Some (res) = cache.get(&k).cloned() {
                Ok(Response::new(Body::from(res)))
            } else {
                Ok(Response::new(Body::from("")))
            }
        },
        Method::PUT => {
            let body = req.into_body().concat2().compat().await?;
            let s = String::from_utf8(body.to_vec()).unwrap();
            let s = s.split(":").collect::<Vec<_>>();
            if s.len() == 2 {
                let (k, v) = (s[0], s[1]);

//                println!("put {} => {}", k, v);

                let mut cache = cache.write().unwrap();
                cache.insert(k.to_string(), v.to_string());
            }
            Ok(Response::new(Body::from("")))
        },
        _ => panic!("unknown")
    }
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let fut = start_server(addr).unit_error().boxed().compat();

    let mut rt = Runtime::new().unwrap();
    rt.block_on(fut).unwrap();
}

