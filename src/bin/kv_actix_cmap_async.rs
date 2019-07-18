use actix_web::{web, App, HttpResponse, HttpServer, Error};
use std::sync::Arc; // судя по коду chashmap здесь RwLock не нужен
use std::net::SocketAddr;
use chashmap::CHashMap;
use futures01::future::{Future, ok};

type Map = CHashMap<String, String>;

const INITIAL_CAPACITY: usize = 100_000;

fn get(
    state: web::Data<Arc<Map>>,
    body: web::Bytes,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let k = String::from_utf8(body.to_vec()).unwrap();

    //    println!("get {}", k);

    // let cache = state.read().unwrap();
    let res = state.get(&k);
    if let Some(res) = res {
        Box::new(ok::<_, Error>(HttpResponse::Ok().body(res.to_string())))
    } else {
        Box::new(ok::<_, Error>(HttpResponse::Ok().body("")))
    }
}

fn put(
    state: web::Data<Arc<Map>>,
    body: web::Bytes,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let s = String::from_utf8(body.to_vec()).unwrap();
    let s = s.split(":").collect::<Vec<_>>();
    if s.len() == 2 {
        let (k, v) = (s[0], s[1]);

        //        println!("put {} => {}", k, v);

        // let cache = state.write().unwrap();
        state.insert(k.to_string(), v.to_string());
    }

    Box::new(ok::<_, Error>(HttpResponse::Ok().body("")))
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let cache = web::Data::new(Arc::new(Map::with_capacity(INITIAL_CAPACITY)));

    HttpServer::new(move || {
        println!("Listening on http://{} {:?}", addr, std::thread::current());
        App::new()
            .register_data(cache.clone())
            .route("/", web::get().to_async(get))
            .route("/", web::put().to_async(put))
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
