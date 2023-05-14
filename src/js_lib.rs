use crate::std_err::ErrType::NotFound;
use crate::std_err::StdErr;

pub fn libs(key: &str) -> String {
    let r = "\r";
    let ob = "{";
    let cb = "}";

    let http_client_js = &format!("
    class Status {ob}
       code
       text
    
        constructor(data) {ob}
            for (let key in data) {ob}
                this[key] = data[key]
            {cb}
        {cb}
    {cb}
    
    class HttpResponse {ob}
       type
       request
       error
       ok
       url
       headers
       response
       status
    
        constructor(data) {ob}
            for (let key in data) {ob}
                this[key] = data[key]
            {cb}
        {cb}
    {cb}
    
    class HttpClient {ob}
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
       ]
    
       constructor(url, config = {ob}{cb}) {ob}
          this.#point = url;
          this.#config = config
       {cb}
    
       #fetch(data) {ob}
          let xhr = new XMLHttpRequest();
          let {ob} config {cb} = data
    
          config = {ob}...config, ...this.#config{cb}
          let headers = config['headers']
    
          for (let key in config) {ob}
             xhr[key] = config[key]
          {cb}
          for (let key in headers) {ob}
             xhr.setRequestHeader(key, headers[key])
          {cb}
    
          xhr.open(data.method, `${ob}this.#point{cb}${ob}data.url{cb}`, false)
          xhr.send(data.body)
    
          let status = xhr.status;
    
          let header = xhr.getAllResponseHeaders();
          let parsed_header = {ob}{cb};
    
          let split = header.split({:?});
    
          for (let v of split) {ob}
             let pair = v.split(\"\");
    
             parsed_header[pair[0]] = pair[1].substring(1)
          {cb}
    
          return new HttpResponse({ob}
             headers: parsed_header,
             type: xhr.responseType,
             response: data.config[\"parseResponse\"](xhr.response),
             url: xhr.responseURL,
             request: xhr,
             error: status === 404,
             ok: status === 200,
             status: new Status(status, xhr.statusText)
          {cb})
       {cb}
    
       get(url = \"\", config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: null,
             method: 'GET'
          {cb})
       {cb}
    
       post(url = \"\", body = \"\", config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: body,
             method: 'POST'
          {cb})
       {cb}
    
       patch(url = \"\", body = '', config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: body,
             method: 'PATCH'
          {cb})
       {cb}
    
       delete(url = '', body = '', config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: body,
             method: 'DELETE'
          {cb})
       {cb}
    
       connect(url = \"\", body = '', config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: body,
             method: 'CONNECT'
          {cb})
       {cb}
    
       head(url = \"\", config = {ob}{cb}) {cb}
          return this.#fetch({ob}
             config,
             url: url,
             body: null,
             method: 'HEAD'
          {cb})
       {cb}
    
       options(url = '', config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: null,
             method: 'HEAD'
          {cb})
       {cb}
    
       put(url = \"\", body = '', config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: body,
             method: 'PUT'
          {cb})
       {cb}
    
       trace(url = \"\", config = {ob}{cb}) {ob}
          return this.#fetch({ob}
             config,
             url: url,
             body: null,
             method: 'TRACE'
          {cb})
       {cb}
    {cb}", r);


       let http_cleint_ts = &format!("
       interface Status {ob}
          code: number
          text: string
       {cb}
       
       interface HttpResponse {ob}
          type: XMLHttpRequestResponseType,
          request: XMLHttpRequest,
          error: boolean
          ok: boolean,
          url: string,
          headers: object
          response: string
          status: Status
       {cb}
       
       class HttpClient {ob}
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
          ]
       
          constructor(url: string | URL, config = {ob}{cb}) {ob}
             this.#point = url;
             this.#config = config
          {cb}
       
          #fetch(data: {ob}
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
          {cb}): HttpResponse {ob}
             let xhr = new XMLHttpRequest();
             let {ob} config {cb} = data
       
             config = {ob}...config, ...this.#config{cb}
             let headers = config['headers']
       
             for (let key in config) {ob}
                xhr[key] = config[key]
             {cb}
             for (let key in headers) {ob}
                xhr.setRequestHeader(key, headers[key])
             {cb}
       
             xhr.open(data.method, `${ob}this.#point{cb}${ob}data.url{cb}`, false)
             xhr.send(data.body)
       
             let status = xhr.status;
       
             let header = xhr.getAllResponseHeaders();
             let parsed_header = {ob}{cb};
       
             let split = header.split({:?});
       
             for (let v of split) {ob}
                let pair = v.split(\"\");
       
                parsed_header[pair[0]] = pair[1].substring(1)
             {cb}
       
             return {ob}
                headers: parsed_header,
                type: xhr.responseType,
                response: data.config[\"parseResponse\"](xhr.response),
                url: xhr.responseURL,
                request: xhr,
                error: status === 404,
                ok: status === 200,
                status: new Status(status, xhr.statusText)
             {cb}
          {cb}
       
          get(url: string | URL = \"\", config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: null,
                method: 'GET'
             {cb})
          {cb}
       
          post(url: string | URL = \"\", body: string | null = \"\", config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: body,
                method: 'POST'
             {cb})
          {cb}
       
          patch(url: string | URL = \"\", body: string | null = '', config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: body,
                method: 'PATCH'
             {cb})
          {cb}
       
          delete(url: string | URL = '', body: string | null = '', config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: body,
                method: 'DELETE'
             {cb})
          {cb}
       
          connect(url: string | URL = \"\", body: string | null = '', config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: body,
                method: 'CONNECT'
             {cb})
          {cb}
       
          head(url: string | URL = \"\", config: object = {ob}{cb}) {cb}
             return this.#fetch({ob}
                config,
                url: url,
                body: null,
                method: 'HEAD'
             {cb})
          {cb}
       
          options(url: string | URL = '', config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: null,
                method: 'HEAD'
             {cb})
          {cb}
       
          put(url: string | URL = \"\", body: string | null = '', config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: body,
                method: 'PUT'
             {cb})
          {cb}
       
          trace(url: string | URL = \"\", config: object = {ob}{cb}) {ob}
             return this.#fetch({ob}
                config,
                url: url,
                body: null,
                method: 'TRACE'
             {cb})
          {cb}", r);

    String::from(match key {
        "context" => "\
class Context {
   map = new Map()
   set(k, v, autoClean=true) {
      this.map.set(k, {
         autoClean,
         val: v
      })
   }
   get(k) {
      let s = this.map.get(k)
      if (s) {
        if (!s.autoClean) {
            return s
        } else {
            this.map.delete(k)
            return s.val
        }
      } else {
          throw Error(\"Property not found\")
      }

   }
}

        ",
        "camel" => "\
class Camel {
  static toString(data) {
    let result = \"\"
    for (const [key, value] of Object.entries(data)) {
      if (Array.isArray(value)) {
        result += `${key}^${value.join(\",\")}\n`
      } else if (typeof value === \"object\") {
        const entries = Object.entries(value)
        const serializedEntries = entries
          .map(([entryKey, entryValue]) => `${entryKey} ${entryValue}`)
          .join(\" \")
        result += `${key}#${serializedEntries}\n`
      } else {
        result += `${key}$${value}\n`
      }
    }
    return result;
  }
  static parse(data) {
    const map = {}
    const lines = data.split(\"\n\")
    for (let line of lines) {
      let $dol = line.indexOf(\"$\")
      let hash = line.indexOf(\"#\")
      let arr = line.indexOf(\"^\")
      const prim_parse = (val) => {
        switch (val) {
          case \"1\": return true
          case \"0\": return false
          case \"\": return null
          default:
            let num_test = Number(val)
            return isNaN(num_test) ? val: num_test
        }
      };
      if ($dol != -1) {
        let key = line.substring(0, $dol)
        let val = prim_parse(line.substr($dol+1))
        map[key] = val;
      }
      else if (arr != -1) {
        let key = line.substring(0, arr)
        let val = line.substring(arr+1)
        map[key] = val.split(\",\").map((v) => prim_parse(v))
      }
      else if (hash != -1) {
        let obj = {};
        let entries = line.substring(hash + 1)
          .split(\" \");
        let len = entries.length - 1;
        for (let i = 0; i < len; i+=2) {
          obj[entries[i]] = entries[i+1]
        }
        map[line.substring(0, hash)] = obj
      }
    }
    return map;
  }
}

        ",
        "utilQuery" => "\
function utilQuery {
    return new Proxy(new URLSearchParams(window.location.search), {
      get: (searchParams, prop) => searchParams.get(prop),
    })
}

        ",
        "utilRandomNumber" => "\
function utilRandomNumber(n) {
   let frs = Math.round(Math.random() * n);
   for (let i = 0; i < n; i++) {
      let curr = Math.round(Math.random() * n)
      if (frs == curr) {
         return i
      }
   }
   return -1
};

        ",
        "utilUUID" => "\
const utilUUID = () => {
   let hash = String(Math.random())
   let map = new Map([
      ['1', 'a'],
      ['2', 'b'],
      ['3', 'c'],
      ['4', 'd'],
      ['5', 'e'],
      ['6', 'f'],
      ['7', 'g'],
      ['8', 'h'],
      ['9', 'i']
   ])
   for (let [k, v] of map) { hash = hash.replace(k, v); }
   return '$' + hash.substring(2)
};

        ",
        "nitro" => "\
class HashGen {
    constructor() {
        this.secure = 2;
        this.hash_len = 15;
        this.salt = '';
        this.rihl = 3;
    }
    _hash(s) {
        const index = (s, i) => {
            let len = s.length;
            return i >= len ? s[i - len] : s[i];
        }

        let char_v = ['g', 'h', 'j', 'k', 'l', '1', '2', '3', '-', '4', '5', '6', '7', '8', '9', '0', '!', '@', '#', '$', '%', '^', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd'];
        let sa = `${s.replace('', '')}${this.salt}`;
        for (let c of this.salt) {
            char_v.push(c);
        }
        char_v = char_v.concat(['z', '&', '*', 'q', 'f', 'x', 'c', 'v', 'b', 'n', 'm', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'N', 'M']);
        let len = sa.length;
        if (len >= this.hash_len) {
            sa = sa.substring(0, this.rihl);
            len = sa.length;
        }
        let offset = this.hash_len % len;
        let r = (this.hash_len - offset) / len;
        let hmm = len % 2 == 0;
        let state = '';
        let fin = '';
        let jesus = sa;
        for (let i = 0; i < r; i++) {
            for (let a of jesus) {
                let founded = false;
                let b = 0;
                for (let c of char_v) {
                    console.log(a, c);
                    if (a == c) {
                        if (hmm && b > len) {
                            state += index(char_v, b - len);
                        }
                        else {
                            state += index(char_v, b + len);
                        }
                        founded = true;
                    }
                    b += 1;
                }
                if (!founded) {
                    throw new Error(`Char not found ${a}`);
                }
            }
            jesus = state;
            fin = `${fin}${state}`;
            state = '';
        }
        fin = `${fin}${jesus.substring(0, offset)}`;
        return fin;
    }
    hash(s) {
        let _h = s;
        for (let i = 0; i < this.secure; i++) {
            _h = this._hash(_h);
        }
        return '$' + _h;
    }
}
        ",
        "enum" => "\
function Enum(fields) {
    var len = fields.length
    for (let i = 0; i < fields.length; i++) {
        this[fields[i]] = Symbol()
    }
}
        ",
        "routine_ts" =>
            "
class Routine {

    private value: Function;
    private args: any[];

    constructor(init: Function, args: any[] = []) {
        this.value = init;
        this.args = args
    }

    do(then: Function) {
        try {
            let _res = this.value(...this.args);
            let obj = {
                state: \"done\",
                error: null,
                value: _res
            }

            let res = then(obj);

            return new Routine(then, [obj]);
        } catch (e) {
           throw e;
        }
    }
}
",
        "routine" =>
            "
class Routine {

    #value;
    #args;

    constructor(init, args = []) {
        this.value = init;
        this.args = args
    }

    do(then) {
        try {
            let _res = this.value(...this.args);
            let obj = {
                state: \"done\",
                error: null,
                value: _res
            }

            let res = then(obj);

            return new Routine(then, [obj]);
        } catch (e) {
           throw e;
        }
    }
}
",
        "http_client" => http_client_js,
        "http_client_ts" => http_cleint_ts,
        "get_by_name" => "
function GetByName(name) {

   if (name === 'name') {
      throw Error('Name should not be \"name\" try another name')
   } 
   return document.getElementById(eval(name));
}
        ",
        "get_by_name_ts" => "
function GetByName<T = HTMLElement>(name): T {
        
   if (name === 'name') {
      throw Error('Name should not be \"name\" try another name')
   } 
   return document.getElementById(eval(name));
}
                ",
        _ => {
            StdErr::exec(NotFound, &format!("Library {key} not found"));
            ""
        }

    })
}