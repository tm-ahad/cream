const NUM_SPLIT = "\x99"
const STR_SPLIT = "\x88"
const BOOL_SPLIT = "\x81"
const ARR_SPLIT = "\x94"
const OBJ_SPLIT = "\x84"
const NUL_SPLIT = "\x79"

type CamelParsable = boolean | string | number | Array<CamelParsable> | object

class Camel {
    static toString(data: CamelParsable) {
        if (data == null)
        {
            return NUL_SPLIT
        }

        switch (typeof data)
        {
            case 'string': return STR_SPLIT + data
            case 'number': return NUM_SPLIT + data
            case 'boolean': return BOOL_SPLIT + data
        }

        if (Array.isArray(data))
        {
            return data.reduce((f, d) => f + this.toString(d))
        }

        let serialized = '';
        for (let key in data) {
            if (data.hasOwnProperty(key)) {
                let value = data[key];
                if (value === null) {
                    serialized += key + NUL_SPLIT;
                } else if (typeof value === 'object') {
                    serialized += key + OBJ_SPLIT + Camel.toString(value) + '\n';
                } else if (Array.isArray(value)) {
                    serialized += key + ARR_SPLIT + value.map(item => {
                        if (item === null) {
                            return NUL_SPLIT;
                        } else if (typeof item === 'object') {
                            return OBJ_SPLIT + Camel.toString(item);
                        } else {
                            return item.toString();
                        }
                    }).join(' ') + '\n';
                } else if (typeof value === 'number') {
                    serialized += key + NUM_SPLIT + value + '\n';
                } else if (typeof value === 'boolean') {
                    serialized += key + BOOL_SPLIT + (value ? '1' : '0') + '\n';
                } else {
                    serialized += key + STR_SPLIT + value + '\n';
                }
            }
        }
        return serialized;
    }

    static parse(data) {
        const map = {};
        const lines = data.split("\n");
        for (let line of lines) {
            let num = line.indexOf(NUM_SPLIT);
            let str = line.indexOf(STR_SPLIT);
            let bool = line.indexOf(BOOL_SPLIT);
            let hash = line.indexOf(OBJ_SPLIT);
            let arr = line.indexOf(ARR_SPLIT);
            let nullInd = line.indexOf(NUL_SPLIT);
       
            const prim_parse = (val: string) => {
                val = val.substring(1);
          
                switch (val.substring(0, 1)) {
                    case OBJ_SPLIT: return Camel.parse(val);
                    case NUM_SPLIT: return Number(val);
                    case NUL_SPLIT: return null;
                    case ARR_SPLIT: return val.split(" ").map(s => prim_parse(s));
                    case STR_SPLIT: return val;
                    case BOOL_SPLIT: return Number(Boolean(val));
                    default:
                        let num_test = Number(val);
                        return isNaN(num_test) ? val : num_test;
                }
            };
       
          var key;
          var val;
          
            switch (true) {
                case num != -1:
                    key = line.substring(0, num);
                    val = Number(line.substr(num + 1));
                    map[key] = val;
                case str != -1:
                    key = line.substring(0, str);
                    val = line.substr(str + 1);
                    map[key] = val == "" ? null : val;
                case bool != -1:
                    key = line.substring(0, bool);
                    val = Boolean(Number(line.substr(bool + 1)));
                    map[key] = val;
                case hash != -1:
                    key = line.substring(0, hash);
                    map[key] = Camel.parse(line.substring(hash + 1));
                case arr != -1:
                    key = line.substring(0, arr);
                    val = line.substring(arr + 1);
                    map[key] = Camel.parse(line.substring(hash + 1));
                case nullInd:
                    map[key] = null;
            }
      
            return map;
        }
    }
}