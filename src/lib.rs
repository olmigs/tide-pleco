use pleco::board::movegen::Legal;
use pleco::board::{movegen::MoveGen, Board};
use pleco::bot_prelude::Searcher;
use pleco::bots::IterativeSearcher;
use pleco::core::mono_traits::AllGenType;
use pleco::tools::eval::Eval;
use pleco::tools::tt::{Entry, NodeBound, TranspositionTable};
use pleco::{BitMove, ScoringMove, ScoringMoveList};
use serde::Serialize;
use std::sync::{Arc, Mutex};
// use std::thread;

// pub struct MigsSearcher {}
// #[derive(Clone)]
pub struct Manager {
    max: u16,
    depth: u16,
    table: TranspositionTable,
    status: TableStatus,
}

enum TableStatus {
    Ready,
    Finding,
}

impl Manager {
    pub fn init(max: u16) -> Manager {
        let table = TranspositionTable::new_num_entries(40000);
        Manager {
            max,
            depth: 0,
            table,
            status: TableStatus::Ready,
        }
    }
    pub fn check_or_find_dumb_best(&self, board: Board, req_depth: u16) -> BitMove {
        let key = board.zobrist();
        let (found, entry) = self.table.probe(key);
        let mut besty;
        if !found {
            besty = IterativeSearcher::best_move(board.shallow_clone(), req_depth);// Manager::init_successor(*self, &board);
            let (found, entry) = self.table.probe(key);
        } else {
            besty = entry.best_move;
        }
        besty
    }
    // fn init_successor(mut self, board: &Board) -> BitMove {
    //     check Manager status
    //     if serving, can't add, can generate
    //     if ready, can add
    //     match self.status {
    //         TableStatus::Ready => {} // besty = Manager::await_table(&board),
    //             _ => {},
    //     }
    //     IterativeSearcher::best_move(board.shallow_clone(), self.depth)
    // }
    fn shallow_add(&self, board: Board) {
        // let besty = IterativeSearcher::best_move(board.shallow_clone(), self.depth);
        // let foo = board.generate_moves();
        // let b = pleco::board::movegen::Legal::gen_legal();
        let scored = MoveGen::generate_scoring::<Legal, AllGenType>(&board);
        for mov in scored {
            let mut newb = board.parallel_clone();
            newb.apply_move(mov.bitmove());
            let key = newb.zobrist();
            let (found, entry) = self.table.probe(key);
            if !found {
                self.add_safely(entry, key, &newb, mov, 1);
            }
        }
    }
    // requires Entry
    fn add_safely(&self, entry: &mut Entry, key: u64, board: &Board, scored: ScoringMove, depth: i16) {
        let eval = Eval::eval_low(&board) as i16;
        entry.place(
            key,
            scored.bitmove(),
            scored.score(),
            eval,
            depth,
            NodeBound::Exact,
            0,
        );
    }
}

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
            routes: Arc::new(Mutex::new(Vec::new())),
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
    pub fn best_move(&self, manager: &Manager) -> () {
        let b2 = self.board.lock().unwrap().shallow_clone();
        if !b2.checkmate() {
            let best = manager.check_or_find_dumb_best(b2, 5);
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
        Board::random().min_moves(15).no_check().one()
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
