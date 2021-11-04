use std::sync::{Arc, Mutex};
// use log::info;
use http_types::headers::HeaderValue;
use pleco::Board;
use serde::{Deserialize, Serialize};
use tide::security::{CorsMiddleware, Origin};
use tide::{Body, Request, Response};
use tide_pleco::{Route, State, Manager};

#[derive(Deserialize, Serialize)]
struct ClientMove {
    uci: String,
}

#[derive(Deserialize, Serialize)]
struct ClientFEN {
    fen: String,
}

#[derive(Serialize)]
struct MoveVec {
    moves: Vec<String>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // let b = Board::default();
    // let manage = Manager::init(21);
    let board = Arc::new(Mutex::new(Board::default()));
    tide::log::start();
    // instantiate Tide app using shared state
    let state = State::new(board.clone());
    let mut app = tide::with_state(state);

    // CORS middleware
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.at("/")
        .get(|_| async { Ok(Body::from_file("public/index.html").await?) })
        .serve_dir("public/")?;

    let next_rt = Route::new("reset", "text", "GET", "/game/restart");
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        req.state().update(Board::start_pos());
        Ok(format!("{}", req.state().fen()))
    });
    app.state().push_route(next_rt);

    let next_rt = Route::new("position", "text", "GET", "/game/pos");
    app.at(&next_rt.path)
        .get(|req: Request<State>| async move { Ok(format!("{}", req.state().fen())) });
    app.state().push_route(next_rt);

    let next_rt = Route::new("set", "text", "PUT", "/game/set");
    app.at(&next_rt.path)
        .put(|mut req: Request<State>| async move {
            let fen_str: ClientFEN = req.body_json().await?;
            req.state().from_fen(fen_str.fen);
            Ok(format!("{}", req.state().fen()))
        });
    app.state().push_route(next_rt);

    let next_rt = Route::new("random", "text", "GET", "/game/rand");
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        let b_rand = req.state().random();
        Ok(format!("{}", b_rand.fen()))
    });
    app.state().push_route(next_rt);

    let next_rt = Route::new("previous", "text", "GET", "/game/prev");
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        req.state().undo();
        let mut res = Response::new(202);
        res.set_body(Body::from_json(&req.state().fen())?);
        Ok(res)
    });
    app.state().push_route(next_rt);

    // GET /next

    // POST /move
    let next_rt = Route::new("uci", "text", "POST", "/game/move");
    app.at(&next_rt.path)
        .post(|mut req: Request<State>| async move {
            let uci_move: ClientMove = req.body_json().await?;
            // info!("{}", uci_move.uci);
            req.state().apply_move(uci_move.uci);
            let mut res = Response::new(202);
            res.set_body(Body::from_json(&req.state().fen())?);
            Ok(res)
            // Ok(format!("{}", req.state().board.fen()))
        });
    app.state().push_route(next_rt);

    let next_rt = Route::new("uci_play", "text", "POST", "/game/play/move");
    app.at(&next_rt.path)
        .post(|mut req: Request<State>| async move {
            let uci_move: ClientMove = req.body_json().await?;
            // info!("{}", uci_move.uci);
            req.state().apply_move(uci_move.uci);
            // req.state().best_move(&manage);
            // thread::sleep(time::Duration::from_millis(1000));
            let mut res = Response::new(202);
            res.set_body(Body::from_json(&req.state().fen())?);
            Ok(res)
            // Ok(format!("{}", req.state().board.fen()))
        });
    app.state().push_route(next_rt);

    let next_rt = Route::new("moves", "json", "GET", "/game/moves");
    app.at(&next_rt.path).get(|req: Request<State>| async move {
        let moves = req.state().get_moves();
        // uncomment for a learning opportunity
        // for mov in moves {
        //     info!("{}", &mov);
        // }
        Ok(Body::from_json(&moves)?)
    });
    app.state().push_route(next_rt);

    let next_rt = Route::new("routes", "json", "GET", "/routes");
    app.at(&next_rt.path)
        .get(|req: Request<State>| async move { Ok(Body::from_json(&req.state().get_routes())?) });

    // start app
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
