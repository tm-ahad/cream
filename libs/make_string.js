Array.prototype.string = function (cb) {
    let out = '';
    this.forEach((i) => {
        out += cb(i)
    })
    return out;
}
