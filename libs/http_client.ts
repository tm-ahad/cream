interface Status {
    code: number;
    text: string;
}

interface HttpResponse {
    type: XMLHttpRequestResponseType;
    request: XMLHttpRequest;
    error: boolean;
    ok: boolean;
    url: string;
    headers: object;
    response: string;
    status: Status;
}

class HttpClient {
    #point: string | URL;
    #config: object;

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

    constructor(url: string | URL, config: object = {}) {
        this.#point = url;
        this.#config = config;
    }

    #fetch(data: {
        config: object,
        url: string | URL,
        body: string | null,
        method:
        'GET' |
        'POST' |
        'PUT' |
        'DELETE' |
        'TRACE' |
        'PATCH' |
        'OPTIONS' |
        'HEAD' |
        'CONNECT'
    }): HttpResponse {
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
        let parsed_header: any = {};

        let split = header.split(':');

        for (let v of split) {
            let pair = v.split("");

            parsed_header[pair[0]] = pair[1].substring(1);
        }

        return {
            headers: parsed_header,
            type: xhr.responseType,
            response: data.config["parseResponse"](xhr.response),
            url: xhr.responseURL,
            request: xhr,
            error: status === 404,
            ok: status === 200,
            status: new Status(status, xhr.statusText)
        };
    }

    get(url: string | URL = "", config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'GET'
        });
    }

    post(url: string | URL = "", body: string | null = "", config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'POST'
        });
    }

    patch(url: string | URL = "", body: string | null = '', config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'PATCH'
        });
    }

    delete(url: string | URL = '', body: string | null = '', config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'DELETE'
        });
    }

    connect(url: string | URL = "", body: string | null = '', config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'CONNECT'
        });
    }

    head(url: string | URL = "", config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'HEAD'
        });
    }

    options(url: string | URL = '', config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'HEAD'
        });
    }

    put(url: string | URL = "", body: string | null = '', config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: body,
            method: 'PUT'
        });
    }

    trace(url: string | URL = "", config: object = {}) {
        return this.#fetch({
            config,
            url: url,
            body: null,
            method: 'TRACE'
        });
    }
}
