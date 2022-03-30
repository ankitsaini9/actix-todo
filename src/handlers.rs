use std::{collections::HashMap, fs::File, sync::Mutex};

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub password: String,
    pub todo: Mutex<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Username {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct AddContent {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct DeleteContent {
    idx: usize,
}

#[derive(Serialize, Deserialize)]
struct EditContent {
    idx: usize,
    content: String,
}

// saving the dataset
fn save_dataset(path: String, dataset: &HashMap<String, User>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, &dataset).expect("Falied to save data!");
}

#[get("/{username}")]
async fn user_todo(username: Path<Username>, state: Data<HashMap<String, User>>) -> impl Responder {
    let user = state.get(&username.username);
    // Pattern match to retrieve the value
    match user {
        Some(_x) => HttpResponse::Ok().json(format!("Welcome {}", username.username)),
        None => HttpResponse::Ok().json(format!("{} this user doesn't exist!", username.username)),
    }
}


#[get("/{username}/todo")]
async fn see(username: Path<Username>, state: Data<HashMap<String, User>>) -> impl Responder {
    let user = state.get(&username.username);
    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            HttpResponse::Ok().json(format!("{} your todo:- {:?}", username.username, &x.todo))
        }
        None => HttpResponse::Ok().json(format!("{} this user doesn't exist!", username.username)),
    }
}

#[post("/{username}/add")]
async fn add(
    username: Path<Username>,
    todo: Json<AddContent>,
    state: Data<HashMap<String, User>>,
) -> impl Responder {
    let user = state.get(&username.username);
    let todo = todo.into_inner();

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            {
                let mut new_todo = x.todo.lock().unwrap();
                new_todo.push(todo.content);
            }

            save_dataset(String::from("dataset.json"), &state);

            HttpResponse::Ok().json(format!("{} your todo:- {:?}", username.username, &x.todo))
        }
        None => HttpResponse::Ok().json(format!("{} this user doesn't exist!", username.username)),
    }
}

#[post("/{username}/delete")]
async fn delete(
    username: Path<Username>,
    todo: Json<DeleteContent>,
    state: Data<HashMap<String, User>>,
) -> impl Responder {
    let user = state.get(&username.username);
    let todo = todo.into_inner();

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            {
                let mut new_todo = x.todo.lock().unwrap();
                new_todo.remove(todo.idx);
            }

            save_dataset(String::from("dataset.json"), &state);

            HttpResponse::Ok().json(format!("{} your todo:- {:?}", username.username, &x.todo))
        }
        None => HttpResponse::Ok().json(format!("{} this user doesn't exist!", username.username)),
    }
}

#[post("/{username}/edit")]
async fn edit(
    username: Path<Username>,
    todo: Json<EditContent>,
    state: Data<HashMap<String, User>>,
) -> impl Responder {
    let user = state.get(&username.username);
    let todo = todo.into_inner();

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            {
                let mut new_todo = x.todo.lock().unwrap();
                new_todo[todo.idx] = todo.content;
            }

            save_dataset(String::from("dataset.json"), &state);

            HttpResponse::Ok().json(format!("{} your todo:- {:?}", username.username, &x.todo))
        }
        None => HttpResponse::Ok().json(format!("{} this user doesn't exist!", username.username)),
    }
}


