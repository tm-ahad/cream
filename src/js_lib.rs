use crate::std_err::ErrType::NotFound;
use crate::std_err::StdErr;

pub fn libs(key: &str) -> String {
    match key {
        "context" => "\
class Context {
   static map = new Map()
   static set(k, v, autoClean=true) {
      Context.map.set(k, {
         autoClean,
         val: v
      })
   }
   static get(k) {
      let s = Context.map.get(k)
      if (!s.autoClean) {
         return s
      } else {
         Context.map.delete(k)
         return s.val
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
        _ => {
            StdErr::exec(NotFound, &format!("Library {key} not found"));
            ""
        }

    }.to_string()
}