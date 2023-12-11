function mapConcat<T>(arr: Array<T>, cb: Function): string {
    let out: string = '';
    arr.forEach((i) => {
        out += cb(i)
    })
    return out;
}
