#![feature(async_await)]
extern crate futures;
extern crate hyper;
extern crate tokio;

use std::collections::HashMap;
use futures::{FutureExt, TryFutureExt};
use futures::compat::{Future01CompatExt};
use hyper::{Body, Request, Response, Server, service::service_fn, Method, rt::Stream};
use std::net::SocketAddr;
use tokio::runtime::Builder;
use crate::hyper::rt::Future;
use core::cmp;
use crossbeam_requests::{Responder, Requester};

type Map = HashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

fn start_server(addr: SocketAddr, req: Requester<Cmd, Option<String>>) -> impl hyper::rt::Future<Item=(), Error=()> {
    println!("Listening on http://{}", addr);

    Server::bind(&addr).serve(move || {
        let req= req.clone();
        service_fn(move |r| serve_req(r, req.clone()).boxed().compat())
    } )
        .map_err(|e| eprintln!("server error: {}", e))
}

async fn serve_req(r: Request<Body>, req: Requester<Cmd, Option<String>>) -> Result<Response<Body>, hyper::Error> {
    let m = r.method();
    match *m {
        Method::GET => {
            let body = r.into_body().concat2().compat().await?;
            let k = String::from_utf8(body.to_vec()).unwrap();

//            println!("get {}", k);

            if let Some (res) = req.request(Cmd::Get(k)) {
                Ok(Response::new(Body::from(res)))
            } else {
                Ok(Response::new(Body::from("")))
            }
        },
        Method::PUT => {
            let body = r.into_body().concat2().compat().await?;
            let s = String::from_utf8(body.to_vec()).unwrap();
            let s = s.split(":").collect::<Vec<_>>();
            if s.len() == 2 {
                let (k, v) = (s[0], s[1]);

//                println!("put {} => {}", k, v);

                req.request(Cmd::Put(k.to_owned(), v.to_owned()));
            }
            Ok(Response::new(Body::from("")))
        },
        _ => panic!("unknown")
    }
}

fn storage_thread(resp: Responder<Cmd, Option<String>>) {
    println!("storage started");

    let mut cache = Map::with_capacity(INITIAL_CAPACITY);

    resp.poll_loop(move |mut req| {
        let res = match req.body().clone() {
            Cmd::Get(k) => cache.get(&k).cloned(),
            Cmd::Put(k, v) => cache.insert(k, v)
        };

        req.respond(res)
    })
}

#[derive(Clone)]
enum Cmd {
    Get(String),
    Put(String, String)
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let cores = cmp::max(num_cpus::get() - 1, 1);
    println!("hyper is on {} threads", cores);

    let (resp, req) = crossbeam_requests::channel::<Cmd, Option<String>>();

    std::thread::spawn(|| storage_thread(resp));

    let mut runtime = Builder::new().core_threads(cores).build().unwrap();

    runtime.block_on(start_server(addr, req)).ok();


}

