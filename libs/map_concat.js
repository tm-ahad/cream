function mapConcat(arr, cb) {
    let out = '';
    arr.forEach((i) => {
        out += cb(i)
    })
    return out;
}
