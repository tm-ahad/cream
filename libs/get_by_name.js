function GetByName(name) {
   if (name === 'name') {
      throw Error('Name should not be \"name\" try another name')
   }
   return document.getElementById(eval(name));
}
