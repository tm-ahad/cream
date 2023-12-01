class Camel {
  static toString(data: string): string {
    let result = \"\"
    for (const [key, value] of Object.entries(data)) {
      if (Array.isArray(value)) {
        result += `${key}^${value.join(",")}\n`
      } else if (typeof value === "object") {
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
  static parse(data: string): object {
    const map = {}
    const lines = data.split("\n")
    for (let line of lines) {
      let $dol = line.indexOf("$")
      let hash = line.indexOf("#")
      let arr = line.indexOf("^")
      const prim_parse = (val) => {
        switch (val) {
          case "1": return true
          case "0": return false
          case "": return null
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
        map[key] = val.split(",").map((v) => prim_parse(v))
      }
      else if (hash != -1) {
        let obj = {};
        let entries = line.substring(hash + 1)
          .split(" ");
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
