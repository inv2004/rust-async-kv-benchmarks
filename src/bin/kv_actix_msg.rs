use actix_web::{web, App, HttpResponse, HttpServer, Error};
use actix::{Actor, Message, Handler, SyncArbiter, SyncContext, Addr};
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
    type Result = Option<String>;
}

impl Handler<Cmd> for KVCache {
    type Result = Option<String>;

    fn handle(&mut self, msg: Cmd, _: &mut Self::Context) -> Self::Result {
        match msg {
            Cmd::Get(k) => self.cache.get(&k).cloned(),
            Cmd::Put(k, v) => self.cache.insert(k, v)
        }
    }
}

fn get(
    state: web::Data<Addr<KVCache>>,
    body: web::Bytes,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let k = String::from_utf8(body.to_vec()).unwrap();

    //    println!("get {}", k);

    Box::new(state
        .send(Cmd::Get(k.to_owned()))
        .and_then(|res| match res {
            Some(s) => Ok(HttpResponse::Ok().body(s)),
            None => Ok(HttpResponse::Ok().body(""))
        })
        .from_err()
    )
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

        Box::new(state
            .send(Cmd::Put(k.to_owned(), v.to_owned()))
            .and_then(|res| match res {
                    Some(s) => Ok(HttpResponse::Ok().body(s)),
                    None => Ok(HttpResponse::Ok().body(""))
                })
            .from_err()
        )

//        let mut cache = state.write().unwrap();
//        cache.insert(k.to_string(), v.to_string());
    } else {
        Box::new(ok::<_, Error>(HttpResponse::Ok().body("")))
    }
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let sys = actix_rt::System::new("kv_actix_async");

    let kv = SyncArbiter::start(1, || {
        KVCache{cache: Map::with_capacity(INITIAL_CAPACITY)}
    });

    HttpServer::new(move || {
        println!("Listening on http://{} {:?}", addr, std::thread::current());
        App::new()
            .data(kv.clone())
            .route("/", web::get().to(get))
            .route("/", web::put().to(put))
    })
//    .workers(1)
        .bind(addr)
        .unwrap()
        .start();

    sys.run().unwrap();
}
