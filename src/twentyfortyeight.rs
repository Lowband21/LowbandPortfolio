use actix_session::Session;
use actix_web::{web, App, HttpResponse, HttpServer, Error};
use serde::{Serialize, Deserialize};
use crate::game::oxydized2048::*;

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
    Ok(HttpResponse::Ok().json(Update {board: game.board, score: game.score}))
}

pub async fn make_move(data: web::Json<MoveInfo>, session: Session) -> Result<HttpResponse, Error> {
    let mut game: Game = session.get("game_state").unwrap().unwrap();
    
    let result = game.action(&data.action);
    session.insert("game_state", game.clone())?;
    match result {
        GameState::Ok | GameState::Gameover => Ok(HttpResponse::Ok().json(Update {board: game.board, score: game.score})),
        GameState::InvalidMove => Ok(HttpResponse::BadRequest().json(Update {board: game.board, score: game.score})),
    }
}

pub async fn undo(session: Session) -> Result<HttpResponse, Error> {
    let mut game: Game = session.get("game_state").unwrap().unwrap();
    game.undo();
    session.insert("game_state", game.clone())?;
    Ok(HttpResponse::Ok().json(Update {board: game.board, score: game.score}))
}

pub async fn reset(session: Session) -> Result<HttpResponse, Error> {
    let mut game = Game::new();  // This will reset the game.
    session.insert("game_state", game.clone())?;
    Ok(HttpResponse::Ok().json(Update {board: game.board, score: game.score}))
}