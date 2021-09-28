import { callEndpoint } from './client_http';

export function endpointHandler(server, path, type, method, data) {
    var data_wrapper;
    switch (path) {
        case '/game/move':
            data_wrapper = { uci: data };
            break;
        case '/game/set':
            data_wrapper = { fen: data };
            break;
    }
    callEndpoint(server, path, type, method, data_wrapper)
        .then((resp) => console.log(resp))
        .catch((err) => console.log(err));
}
