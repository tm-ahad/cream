type EntangleFn<T> = (value: T) => void;

function particle<T>(value: T) {
    let val: T = value;
    let subs: Array<EntangleFn<T>> = []

    return {
        get value() {return value;},
        entangle(fn: EntangleFn<T>) {
            subs.push(fn)
            fn(val)
        },
        set value(_vall: T) {
            val = _val
            for (let i=0; i<subs.length; ++i) {
                subs[i](_val)
            }
        }
    }
}
