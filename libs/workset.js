class WorkSet {
    #error
    #state
    #valueFunc
    #value

    constructor(value, error = null) {
        this.#valueFunc = () => value
        this.#error = error


        if (!error) {
            try {
                let v = this.#valueFunc()

                this.#state = State.Unwraped
                this.#value = v
            } catch (err) {
                this.#state = State.Error;
                this.#error = err
            }
        }
    }


    do(cb, params = []) {
        switch (this.#state) {
            case State.Unwraped : {
                try {
                    let v = cb(...[this.#value, params])

                    return new WorkSet(v)
                } catch (err) {
                    this.#state = State.Error;
                    this.#error = err

                    return this
                }
            }
            case State.Error : return new WorkSet(null, this.#error);
        }
    }

    check(cb) {
        cb(this.#error)
        return this
    }

    state() {
        return this.#state
    }

    error() {
        return this.#error
    }
}