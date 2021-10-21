use std::sync::{Arc, Mutex};
use pleco::board::{Board};
use pleco::bot_prelude::Searcher;
use pleco::bots::{IterativeSearcher};
use serde::{Serialize};

pub struct MigsSearcher {}
pub struct Manager {}

// impl Manager {
//     fn init() {}
//     fn 
// }

#[derive(Clone, Debug)]
pub struct State {
    board: Arc<Mutex<Board>>,
    routes: Arc<Mutex<Vec<Route>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Route {
    pub name: String,
    pub response_type: String,
    pub method: String,
    pub path: String,
}

impl State {
    pub fn new(board: Arc<Mutex<Board>>) -> State {
        State {
            board,
            routes: Arc::new(Mutex::new(Vec::new()))
        }
    }
    pub fn fen(&self) -> String {
        return self.board.lock().unwrap().fen();
    }
    pub fn update(&self, board: Board) -> () {
        *self.board.lock().unwrap() = board;
    }
    pub fn apply_move(&self, uci: String) -> () {
        self.board.lock().unwrap().apply_uci_move(&uci);
    }
    pub fn best_move(&self) -> () {
        let b2 = self.board.lock().unwrap().shallow_clone();
        if !b2.checkmate() {
            let best = IterativeSearcher::best_move(b2, 5);
            self.board.lock().unwrap().apply_move(best);
        }
    }
    pub fn from_fen(&self, fen: String) -> () {
       let b2 = Board::from_fen(&fen).unwrap();
       self.update(b2);
    }
    pub fn get_moves(&self) -> Vec<String> {
        let moves_list = self.board.lock().unwrap().generate_moves();
        let mut moves_vec = Vec::new();
        for mov in moves_list {
            moves_vec.push(pleco::core::piece_move::BitMove::stringify(mov));
        }
        return moves_vec;
    }
    pub fn undo(&self) -> () {
        self.board.lock().unwrap().undo_move();
    }
    pub fn random(&self) -> Board {
        Board::random()
            .min_moves(15)
            .no_check()
            .one()
    }
    pub fn get_routes(&self) -> Vec<Route> {
        return self.routes.lock().unwrap().clone();
    }
    pub fn push_route(&self, rt: Route) -> () {
        self.routes.lock().unwrap().push(rt);
    }
}

impl Route {
    pub fn new(name: &str, resp_type: &str, method: &str, path: &str) -> Route {
        Route { 
            name: name.to_string(), 
            response_type: resp_type.to_string(), 
            method: method.to_string(),
            path: path.to_string(),
        }
    }
}