use std::sync::{Arc, Mutex};
use pleco::board::{Board};
use pleco::bot_prelude::Searcher;
use pleco::bots::{IterativeSearcher};
// use log::info;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Body};
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
// use tide::prelude::*;

#[derive(Clone, Debug)]
struct State {
    board: Arc<Mutex<Board>>,
    routes: Arc<Mutex<Vec<Route>>>,
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

#[derive(Debug, Clone, Serialize)]
struct Route {
    name: String,
    response_type: String,
    method: String,
    path: String,
}

fn def_route(name: &str, resp_type: &str, method: &str, path: &str) -> Route {
    Route { 
        name: name.to_string(), 
        response_type: resp_type.to_string(), 
        method: method.to_string(),
        path: path.to_string(),
     }
}

impl State {
    fn fen(&self) -> String {
        return self.board.lock().unwrap().fen();
    }
    fn update(&self, board: Board) -> () {
        *self.board.lock().unwrap() = board;
    }
    fn apply_move(&self, uci: String) -> () {
        self.board.lock().unwrap().apply_uci_move(&uci);
    }
    fn best_move(&self) -> () {
        let b2 = self.board.lock().unwrap().clone();
        let best = IterativeSearcher::best_move(b2, 5);
        self.board.lock().unwrap().apply_move(best);
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
    // fn set_routes(&self, rts: Vec<Route>) -> () {
    //     *self.routes.lock().unwrap() = rts;
    // }
    fn get_routes(&self) -> Vec<Route> {
        return self.routes.lock().unwrap().clone();
    }
    fn push_route(&self, rt: Route) -> () {
        self.routes.lock().unwrap().push(rt);
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    
    // instantiate Tide app using shared state
    let state = State {
        board: Arc::new(Mutex::new(Board::default())),
        routes: Arc::new(Mutex::new(Vec::new()))
    };
    let mut app = tide::with_state(state);
    // let mut rts = Vec::new();

    // CORS middleware
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.at("/").get(|_| async 
        { Ok(Body::from_file("public/index.html").await?) })
        .serve_dir("public/")?;

    let next_rt = def_route(
        "reset", 
        "text", 
        "GET", 
        "/game/restart"
    );
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        req.state().update(Board::start_pos());
        Ok(format!("{}", req.state().fen()))
    });
    app.state().push_route(next_rt);

    let next_rt = def_route(
        "position", 
        "text", 
        "GET", 
        "/game/pos"
    );
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        Ok(format!("{}", req.state().fen()))
    });
    app.state().push_route(next_rt);

    let next_rt = def_route(
        "set", 
        "text", 
        "PUT", 
        "/game/set"
    );
    app.at(&next_rt.path).put(|mut req: Request<State>| async move {
        let fen_str: FenStr = req.body_json().await?;
        req.state().from_fen(fen_str.fen);
        Ok(format!("{}", req.state().fen()))
    });
    app.state().push_route(next_rt);

    let next_rt = def_route(
        "random", 
        "text", 
        "GET", 
        "/game/rand"
    );
    app.at(&next_rt.path).get(|_req: Request<State>| async move {
        let b_rand = Board::random()
            .min_moves(15)
            .no_check()
            .one();
        Ok(format!("{}", b_rand.fen()))
    });
    app.state().push_route(next_rt);

    // GET /prev

    // GET /next

    // POST /move
    let next_rt = def_route(
        "uci", 
        "text", 
        "POST", 
        "/game/move"
    );
    app.at(&next_rt.path).post(|mut req: Request<State>| async move {
        let uci_move: UciMove = req.body_json().await?;
        // info!("{}", uci_move.uci);
        req.state().apply_move(uci_move.uci);
        let mut res = Response::new(202);
        res.set_body(Body::from_json(&req.state().fen())?);
        Ok(res)
        // Ok(format!("{}", req.state().board.fen()))
     });
     app.state().push_route(next_rt);

    let next_rt = def_route(
        "uci_play", 
        "text", 
        "POST", 
        "/game/play/move"
    );
    app.at(&next_rt.path).post(|mut req: Request<State>| async move {
        let uci_move: UciMove = req.body_json().await?;
        // info!("{}", uci_move.uci);
        req.state().apply_move(uci_move.uci);
        req.state().best_move();
        // thread::sleep(time::Duration::from_millis(1000));
        let mut res = Response::new(202);
        res.set_body(Body::from_json(&req.state().fen())?);
        Ok(res)
        // Ok(format!("{}", req.state().board.fen()))
     });
     app.state().push_route(next_rt);

    let next_rt = def_route(
        "moves", 
        "json", 
        "GET", 
        "/game/moves"
    );
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        let moves = req.state().get_moves();
        // uncomment for a learning opportunity
        // for mov in moves {
        //     info!("{}", &mov);
        // }
        Ok(Body::from_json(&moves)?)
    });
    app.state().push_route(next_rt);
    
    // let j = serde_json::to_string(&routes)?;
    let next_rt = def_route(
        "routes", 
        "json", 
        "GET", 
        "/routes"
    );
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        Ok(Body::from_json(&req.state().get_routes())?)
    });

    // let path = Path::new("routes.json");
    // let display = path.display();
    // let mut file = match File::create("routes.json") {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };
    // match file.write_all(j.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }

    // start app
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}