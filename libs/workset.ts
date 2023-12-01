enum State {
    Unwraped,
    Error
}

type OptionalError = Error | null

class WorkSet<vT=any> {
    #error: OptionalError
    #state: State
    #valueFunc: () => vT
    #value: vT

    constructor(value: vT, error: OptionalError = null) {
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


    do<pT=any>(cb: Function, params: Array<pT> = []): WorkSet {
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
            case State.Error : return new WorkSet<null>(null, this.#error);
        }
    }

    check(cb: Function): WorkSet {
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