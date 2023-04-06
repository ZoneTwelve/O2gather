use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use rand::Rng;
use serde_json::{json};

struct StoreItem {
  product_name: &'static str,
  product_id: u32,
  counter: Mutex<u32>,
  visit: Mutex<u32>,
}

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn get_product(data: web::Data<StoreItem>) -> String {
  let pname = data.product_name;
  let pid = data.product_id;
  let counter = data.counter.lock().unwrap();
  let mut _visit = data.visit.lock().unwrap();
  if *counter > 0 { 
    let data = json!({
      "productName": pname,
      "productId": pid,
      "remain": *counter,
    });
    *_visit += 1;
    println!("Visit: {}", *_visit);
    data.to_string()
  } else { 
    format!("Product is out of the stock") 
  }
}

async fn buy_product(data: web::Data<StoreItem>) -> String {
  let mut counter = data.counter.lock().unwrap();
  if *counter > 0 {
    *counter -= 1;
    println!("Product remain: {}", *counter);
    String::from("Thank you for the support, we are processing your order.")
  } else {
    String::from("Out of stock!")
  }
}

async fn reset_product(data: web::Data<StoreItem>) -> String {
  let mut counter = data.counter.lock().unwrap();
  *counter = 3;
  String::from("Reset successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mut rng = rand::thread_rng();
  println!("Service is boot up.");
  let only_product = web::Data::new(StoreItem {
    product_name: "The only prodcut I have",
    product_id: rng.gen::<u32>(),
    counter: Mutex::new(3),
    visit: Mutex::new(0),
  });

  HttpServer::new(move || {
    App::new()
      .service(index)
      .app_data(only_product.clone())
      .route("/product", web::get().to(get_product))
      .route("/buy/1", web::get().to(buy_product))
      .route("/reset/1", web::get().to(reset_product))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
