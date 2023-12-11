function mapConcat(arr, cb) {
    let out = '';
    this.forEach((i) => {
        out += cb(i)
    })
    return out;
}
