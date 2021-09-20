import { callEndpoint } from './client_http';

export function getRandomBoard(serv) {
    callEndpoint(serv, 'rand', 'text')
        .then((fen) => updateFEN(fen))
        .catch((err) => console.log(err));
}

export function connectToServer(serv) {
    callEndpoint(serv, 'pos', 'text')
        .then((fen) => {
            updateFEN(fen, serv);
        })
        .catch((err) => console.log(err));
}

export function sendUCI(uci, serv) {
    var uci_wrapper = { uci: uci };
    callEndpoint(serv, 'move', 'text', 'POST', uci_wrapper)
        .then((fen) => updateFEN(fen.slice(1, fen.length - 1), serv)) // removes double quotes
        .catch((err) => console.log(err));
}

export function sendFEN(fen, serv) {
    var fen_wrapper = { fen: fen };
    callEndpoint(serv, 'set', 'text', 'PUT', fen_wrapper)
        .then((data) => console.log(data))
        .catch((err) => console.log(err));
}
