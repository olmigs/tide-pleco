// use async_std::sync::{Arc, Mutex};
use std::sync::{Arc, Mutex};
use pleco::Board;
use log::info;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Body};
// use tide::prelude::*;

#[derive(Clone, Debug)]
struct State {
    board: Arc<Mutex<Board>>,
    //board: Arc<Mutex<Board>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UciMove {
    uci: String,
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
        info!("apply_move:: Success: {}  FEN: {}", success, self.fen());
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

    // GET /start
    app.at("/game/restart").get(|req: Request<State>| async move {
        req.state().update(Board::start_pos());
        Ok(format!("{}", req.state().fen()))
    });

    // GET /pos
    app.at("/game/pos").get(|req: Request<State>| async move {
        Ok(format!("{}", req.state().fen()))
    });

    // GET /prev

    // GET /next

    // POST /move
    app.at("/game/move").post(|mut req: Request<State>| async move {
        let uci_move: UciMove = req.body_json().await?;
        req.state().apply_move(uci_move.uci);
        let mut res = Response::new(202);
        res.set_body(Body::from_json(&req.state().fen())?);
        Ok(res)
        // Ok(format!("{}", req.state().board.fen()))
     });
    
    // start app
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}