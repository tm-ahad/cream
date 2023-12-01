interface Array<T> {
    string(this: T[], cb: (arg: T) => string): string;
}

if (typeof Array.prototype.string !== 'function') {
  Array.prototype.string = function (cb: Function): string {
    let out: string = '';
    this.forEach((i) => {
        out += cb(i)
    })
    return out;
  }
}
