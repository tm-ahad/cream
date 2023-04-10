const utilUUID = (): string => {
   let hash: string = String(Math.random())

   let map = new Map<string, string>([
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

export default utilUUID