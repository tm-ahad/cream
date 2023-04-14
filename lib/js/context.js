class Store {
   map = new Map()

   set(k, v, autoClean) {
      this.map.set(k, {
         autoClean,
         val: v
      })
   }

   at(k) {
      let s = Store.map.get(k)

      if (!s.autoClean) {
         return s
      } else {
         Store.map.delete(k)
         return s
      }
   }
}