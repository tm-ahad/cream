
const utilRandomNumber = (n) => {
   let frs = Math.round(Math.random() * n);

   for (let i = 0; i < n; i++) {
      let curr = Math.round(Math.random() * n)

      if (frs == curr) {
         return i
      }
   }

   return -1
};