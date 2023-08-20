use crate::routes::game::oxydized2048::*;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MoveInfo {
    action: Action,
}

#[derive(Serialize)]
pub struct Update {
    board: [[u32; 4]; 4],
    score: u32,
}

pub async fn new_game(session: Session) -> Result<HttpResponse, Error> {
    let game = Game::new();
    session.insert("game_state", game.clone())?;
    Ok(HttpResponse::Ok().json(Update {
        board: game.board,
        score: game.score,
    }))
}

pub async fn make_move(data: web::Json<MoveInfo>, session: Session) -> Result<HttpResponse, Error> {
    let mut game: Game = session
        .get("game_state")
        .unwrap_or(Some(Game::new()))
        .unwrap_or(Game::new());

    let result = game.action(&data.action);
    session.insert("game_state", game.clone())?;
    println!("In Here");
    match result {
        GameState::Ok | GameState::Gameover => Ok(HttpResponse::Ok().json(Update {
            board: game.board,
            score: game.score,
        })),
        GameState::InvalidMove => {
            println!("Invalid move");
            Ok(HttpResponse::BadRequest().json(Update {
                board: game.board,
                score: game.score,
            }))
        }
    }
}

pub async fn undo(session: Session) -> Result<HttpResponse, Error> {
    let mut game: Game = session.get("game_state").unwrap().unwrap();
    game.undo();
    session.insert("game_state", game.clone())?;
    Ok(HttpResponse::Ok().json(Update {
        board: game.board,
        score: game.score,
    }))
}

pub async fn reset(session: Session) -> Result<HttpResponse, Error> {
    let game = Game::new(); // This will reset the game.
    session.insert("game_state", game.clone())?;
    Ok(HttpResponse::Ok().json(Update {
        board: game.board,
        score: game.score,
    }))
}
