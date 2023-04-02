use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use rand::Rng;
use serde_json::{json};

struct AppStateWithCounter {
  //productName: Mutex<str>,
  product_id: u32,
  counter: Mutex<i32>,
}

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn get_product(data: web::Data<AppStateWithCounter>) -> String {
  // let mut counter = data.counter.lock().unwrap();
  let mut counter = data.counter.lock().unwrap();
  let pid = data.product_id;
  if *counter > 0 { 
    let data = json!({
      "productId": pid,
      "remain": *counter,
    });
    *counter -= 1;
    data.to_string()
  } else { 
    format!("Product is out of the stock") 
  }
}

//async fn manual_hello() -> impl Responder {
//  HttpResponse::Ok().body("Hello user!")
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mut rng = rand::thread_rng();
  println!("Service is boot up.");
  let only_product = web::Data::new(AppStateWithCounter {
    //productName: Mutex("Hello world T-shirt"),
    product_id: rng.gen::<u32>(),
    counter: Mutex::new(10),
  });

  HttpServer::new(move || {
    App::new()
      .service(index)
      .app_data(only_product.clone())
      .route("/hey", web::get().to(get_product))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
