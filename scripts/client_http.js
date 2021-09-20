function responseHandler(response, type) {
    switch (type) {
        case 'text':
            return response.text();
        case 'json':
            return response.json();
    }
}
export function callEndpoint(server, endpoint, respType, method, data = '') {
    const url = server + endpoint;
    if (method === 'GET') {
        return fetch(url, {
            method: 'GET',
            credentials: 'same-origin',
        }).then((resp) => {
            return responseHandler(resp, respType);
        });
    } else {
        return fetch(url, {
            method: method,
            credentials: 'same-origin',
            body: JSON.stringify(data),
        }).then((resp) => {
            return responseHandler(resp, respType);
        });
    }
}
