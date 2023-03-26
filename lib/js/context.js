class Store {
   map = new Map()

   set(k, v, autoClean) {
      this.map.set(k, {
         autoClean,
         val: v
      })
   }

   at(k) {
      let s = Store.map[k]

      if (s.autoClean == true) {
         return s
      } else {
         Store.map[k] = undefined
         return s
      }
   }
}