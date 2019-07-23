use actix_web::{web, App, HttpResponse, HttpServer, Error};
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::net::SocketAddr;
use futures01::future::{Future, ok};

type Map = HashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

fn get(
    state: web::Data<Arc<RwLock<Map>>>,
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
    state: web::Data<Arc<RwLock<Map>>>,
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

    let cache = web::Data::new(Arc::new(RwLock::new(Map::with_capacity(INITIAL_CAPACITY))));

    HttpServer::new(move || {
        println!("Listening on http://{} {:?}", addr, std::thread::current());
        App::new()
            .register_data(cache.clone())
            .route("/", web::get().to_async(get))
            .route("/", web::put().to_async(put))
    })
//    .workers(1)
    .bind(addr)
    .unwrap()
    .start();

    sys.run().unwrap();
}
