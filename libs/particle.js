function particle(value) {
    let val = value;
    let subs = []

    return {
        get value() {return val;},
        entangle(fn) {
            subs.push(fn)
        },
        set value(_val) {
            val = _val
            for (let i=0; i<subs.length; ++i) {
                subs[i](val)
            }
        }
    }
}
export {particle}
