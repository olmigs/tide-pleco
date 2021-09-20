import { callEndpoint } from './client_http';

export function endpointHandler(server, name, type, method, data) {
    var data_wrapper;
    switch (name) {
        case '/game/move':
            data_wrapper = { uci: data };
            break;
        case '/game/set':
            data_wrapper = { fen: data };
            break;
    }
    callEndpoint(server, name, type, method, data_wrapper)
        .then((resp) => console.log(resp))
        .catch((err) => console.log(err));
}
