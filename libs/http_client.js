class Status {
    constructor(code, text) {
        this.code = code;
        this.text = text;
    }
}

class HttpResponse {
    constructor(data) {
        this.response = data.response
        this.request = data.request
        this.headers = data.headers
        this.error = data.error
        this.type = data.type
        this.url = data.url
        this.ok = data.ok
    }

    json() {
        return JSON.parse(this.response)
    }
}

class HttpClient {
    #point
    #config

    static methods = [
        'GET',
        'POST',
        'PUT',
        'DELETE',
        'TRACE',
        'PATCH',
        'OPTIONS',
        'HEAD',
        'CONNECT'
    ];

    constructor(url, config = {}) {
        this.#point = url;
        this.#config = config;
    }

    #fetch(data) {
        let xhr = new XMLHttpRequest();
        let { config } = data;

        config = { ...config, ...this.#config };
        let headers = config['headers'];

        for (let key in config) {
            xhr[key] = config[key];
        }
        for (let key in headers) {
            xhr.setRequestHeader(key, headers[key]);
        }

        xhr.open(data.method, `${this.#point}${data.url}`, false);
        xhr.send(data.body);

        let status = xhr.status;

        let header = xhr.getAllResponseHeaders();
        let parsed_header = {};

        let split = header.split(':');

        for (let v of split) {
            let pair = v.split("");

            parsed_header[pair[0]] = pair[1].substring(1);
        }

        return new HttpResponse({
            headers: parsed_header,
            type: xhr.responseType,
            response: data.config["parseResponse"](xhr.response),
            url: xhr.responseURL,
            request: xhr,
            error: status === 404,
            ok: status === 200,
            status: new Status(status, xhr.statusText)
        });
    }

    get(url = "", config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'GET'
        });
    }

    post(url = "", body = "", config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'POST'
        });
    }

    patch(url = "", body = '', config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'PATCH'
        });
    }

    delete(url = '', body = '', config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'DELETE'
        });
    }

    connect(url = "", body = '', config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'CONNECT'
        });
    }

    head(url = "", config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'HEAD'
        });
    }

    options(url = '', config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'OPTIONS'
        });
    }

    put(url = "", body = '', config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'PUT'
        });
    }

    trace(url = "", config = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'TRACE'
        });
    }
}
