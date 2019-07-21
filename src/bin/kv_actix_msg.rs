use actix_web::{web, App, HttpResponse, HttpServer, Error};
use actix::{Actor, Message, Handler, SyncArbiter, SyncContext, Addr};
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::net::SocketAddr;
use futures01::future::{Future, ok};

type Map = HashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

struct KVCache {
    cache: Map
}

impl Actor for KVCache {
    type Context = SyncContext<Self>;
}

enum Cmd {
    Get(String),
    Put(String, String)
}

impl Message for Cmd {
    type Result = Result<Option<String>, Error>;
}

impl Handler<Cmd> for KVCache {
    type Result = Result<Option<String>, Error>;

    fn handle(&mut self, msg: Cmd, _: &mut Self::Context) -> Self::Result {
        Ok(None)
    }
}

fn get(
    state: web::Data<Addr<KVCache>>,
    body: web::Bytes,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let k = String::from_utf8(body.to_vec()).unwrap();

    //    println!("get {}", k);

    let cache = state.read().unwrap();
    if let Some(res) = cache.get(&k).cloned() {
        Box::new(ok::<_, Error>(HttpResponse::Ok().body(res)))
    } else {
        Box::new(ok::<_, Error>(HttpResponse::Ok().body("")))
    }
}

fn put(
    state: web::Data<Addr<KVCache>>,
    body: web::Bytes,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let s = String::from_utf8(body.to_vec()).unwrap();
    let s = s.split(":").collect::<Vec<_>>();
    if s.len() == 2 {
        let (k, v) = (s[0], s[1]);

        //        println!("put {} => {}", k, v);

        let mut cache = state.write().unwrap();
        cache.insert(k.to_string(), v.to_string());
    }

    Box::new(ok::<_, Error>(HttpResponse::Ok().body("")))
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let sys = actix_rt::System::new("kv_actix_async");

    let kv = SyncArbiter::start(1, || {
        let cache = Map::with_capacity(INITIAL_CAPACITY);
        KVCache{cache}
    });

    HttpServer::new(move || {
        println!("Listening on http://{} {:?}", addr, std::thread::current());
        App::new()
            .register_data(kv.clone())
            .route("/", web::get().to(get))
            .route("/", web::put().to(put))
    })
//    .workers(1)
        .bind(addr)
        .unwrap()
        .start();

    sys.run().unwrap();
}
