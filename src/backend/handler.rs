use std::mem::take;
use actix_web::{HttpResponse, post, Responder};
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Data, Json, ServiceConfig};

use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::backend::pool::{Bot, GameTask};

#[derive(Deserialize)]
struct PlayRequest{
    bots: Vec<Bot>
}

#[post("/game/play")]
async fn play(state: Data<AppState>, param: Json<PlayRequest>) -> impl Responder{

    let task = GameTask{
        bots: param.bots.clone()
    };

    state.game_pool.lock().add_task(task);

    HttpResponse::Ok().json(param.bots.clone())
}

pub fn configure(cfg: &mut ServiceConfig){

    cfg
        .service(play);
}