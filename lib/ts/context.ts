interface Cell<v> {
   autoClean: boolean
   val: v
   state: State
}

enum State {
   Taken,
   Data = 1
}

class _Store<k, v> {
   map = new Map<k, Cell<v>>()

   set(k: k, v: v, autoClean: boolean) {
      this.map.set(k, {
         autoClean,
         val: v,
         state: State.Data
      }) 
   }

   at(k): v | undefined | State {
      let s = this.map.get(k)

      if (s?.state == State.Taken) {
         return State.Taken
      } else if (s?.autoClean == true) {
         return s.val
      } else {
         this.map.delete(k)
         return s?.val
      }
   }
}