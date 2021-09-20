// use async_std::sync::{Arc, Mutex};
use std::sync::{Arc, Mutex};
use pleco::board::{Board};
// use pleco::board::movegen::MoveGen;
// use pleco::core::mono_traits::GenTypeTrait;
// use pleco:: MoveList;
use log::info;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Body};
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
// use tide::prelude::*;

#[derive(Clone, Debug)]
struct State {
    board: Arc<Mutex<Board>>,
    //board: Arc<Mutex<Board>>,
}

#[derive(Deserialize, Serialize)]
struct UciMove {
    uci: String,
}

#[derive(Deserialize, Serialize)]
struct FenStr {
    fen: String,
}

#[derive(Serialize)]
struct MoveVec {
    moves: Vec<String>,
}

impl State {
    fn fen(&self) -> String {
        return self.board.lock().unwrap().fen();
    }
    fn update(&self, board: Board) -> () {
        *self.board.lock().unwrap() = board;
    }
    fn apply_move(&self, uci: String) -> () {
        //let b2 = self.board.lock().unwrap().clone();
        let success = self.board.lock().unwrap().apply_uci_move(&uci);
        info!("apply_move::  {}  FEN: {}", success, self.fen());
    }
    fn from_fen(&self, fen: String) -> () {
       let b2 = Board::from_fen(&fen).unwrap();
       self.update(b2);
    }
    fn get_moves(&self) -> Vec<String> {
        let b2 = self.board.lock().unwrap().clone();
        let moves_list = b2.generate_moves();
        let mut moves_vec = Vec::new();
        for mov in moves_list {
            moves_vec.push(pleco::core::piece_move::BitMove::stringify(mov));
        }
        return moves_vec;

    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    
    // instantiate Tide app using shared state
    let state = State {
        board: Arc::new(Mutex::new(Board::default()))
    };
    let mut app = tide::with_state(state);

    // CORS middleware
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.at("/").get(|_| async 
        { Ok(Body::from_file("public/index.html").await?) })
        .serve_dir("public/")?;

    // GET /start
    app.at("/game/restart").get(|req: Request<State>| async move {
        req.state().update(Board::start_pos());
        Ok(format!("{}", req.state().fen()))
    });

    // GET /pos
    app.at("/game/pos").get(|req: Request<State>| async move {
        Ok(format!("{}", req.state().fen()))
    });

    // PUT /set
    app.at("/game/set").put(|mut req: Request<State>| async move {
        let fen_str: FenStr = req.body_json().await?;
        req.state().from_fen(fen_str.fen);
        Ok(format!("{}", req.state().fen()))
    });

    // GET /rand
    app.at("/game/rand").get(|_req: Request<State>| async move {
        let b_rand = Board::random()
            .min_moves(15)
            .no_check()
            .one();
        Ok(format!("{}", b_rand.fen()))
    });

    // GET /prev

    // GET /next

    // POST /move
    app.at("/game/move").post(|mut req: Request<State>| async move {
        let uci_move: UciMove = req.body_json().await?;
        // info!("{}", uci_move.uci);
        req.state().apply_move(uci_move.uci);
        let mut res = Response::new(202);
        res.set_body(Body::from_json(&req.state().fen())?);
        Ok(res)
        // Ok(format!("{}", req.state().board.fen()))
     });

    // GET /moves
    app.at("/game/moves").get(|req: Request<State>| async move {
        let moves = req.state().get_moves();
        // uncomment for a learning opportunity
        // for mov in moves {
        //     info!("{}", &mov);
        // }
        Ok(Body::from_json(&moves)?)
    });
    
    // start app
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}