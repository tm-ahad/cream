class Work {
    #value;
    constructor(init) {
        this.#value = init;
    }

    do(then) {
        try {
            let _res = this.#value();

            let res = then({
                state: "done",
                error: null,
                value: _res
            });

            return res;
        } catch (e) {
            throw e;
        }
    }
}