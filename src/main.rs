use actix_web::{web, App, HttpServer};

mod handlers;
use handlers::User;

use serde_json::Result;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

// Reading the dataset
fn read_dataset(path: String) -> Result<HashMap<String, User>> {
    let file = File::open(path).expect("Failed to load file");
    let reader = BufReader::new(file);
    let dataset: HashMap<String, User> = serde_json::from_reader(reader)?;

    Ok(dataset)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dataset = read_dataset(String::from("dataset.json")).unwrap();
    let dataset = web::Data::new(dataset);
    
    HttpServer::new(move || {
        App::new()
            .service(handlers::user_todo)
            .service(handlers::see)
            .service(handlers::add)
            .service(handlers::delete)
            .service(handlers::edit)
            .app_data(dataset.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
