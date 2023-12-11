function mapConcat<T>(arr: Array<T>, cb: Function): string {
    let out: string = '';
    this.forEach((i) => {
        out += cb(i)
    })
    return out;
}
