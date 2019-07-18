use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;
use std::collections::HashMap;
use std::net::SocketAddr;

type Map = HashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

fn get(state: web::Data<RwLock<Map>>, body: web::Bytes) -> impl Responder {
    let k = String::from_utf8(body.to_vec()).unwrap();

//    println!("get {}", k);

    let cache = state.read().unwrap();
    if let Some (res) = cache.get(&k).cloned() {
        HttpResponse::Ok().body(res)
    } else {
        HttpResponse::Ok().body("")
    }
}

fn put(state: web::Data<RwLock<Map>>, body: web::Bytes) -> impl Responder {
    let s = String::from_utf8(body.to_vec()).unwrap();
    let s = s.split(":").collect::<Vec<_>>();
    if s.len() == 2 {
        let (k, v) = (s[0], s[1]);

//        println!("put {} => {}", k, v);

        let mut cache = state.write().unwrap();
        cache.insert(k.to_string(), v.to_string());
    }

    HttpResponse::Ok().body("")
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let cache = web::Data::new(RwLock::new(Map::with_capacity(INITIAL_CAPACITY)));

    HttpServer::new(move || {
        println!("Listening on http://{} {:?}", addr, std::thread::current());
        App::new()
            .register_data(cache.clone())
            .route("/", web::get().to(get))
            .route("/", web::put().to(put))
    })
        .bind(addr)
        .unwrap()
        .run()
        .unwrap();
}
