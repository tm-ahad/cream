type EntangleFn<T> = (value: T) => void;

function particle<T>(value: T) {
    let val: T = value;
    let subs: Array<EntangleFn<T>> = []

    return {
        get value() {return val;},
        subscribe(fn: EntangleFn<T>) {
            subs.push(fn)
        },
        set value(_val: T) {
            val = _val
            for (let i=0; i<subs.length; ++i) {
                subs[i](val)
            }
        }
    }
}
export {particle}
