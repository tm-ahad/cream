class Store {
   map = new Map()

   set(k, v, autoClean) {
      this.map.set(k, {
         autoClean,
         val: v
      })
   }

   at(k) {
      let s = this.map.get(k)

      if (!s.autoClean) {
         return s
      } else {
         this.map.delete(k)
         return s
      }
   }
}
