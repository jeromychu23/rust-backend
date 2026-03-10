use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, patch, post, web::Json, web::Path};
use validator::Validate;

mod models;

#[get("/pizzars")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzars available")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name required!"),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating a pizza with {uuid}!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
