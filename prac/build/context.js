 class Context {
    map

    constructor() {}
    tell(name, data, for_=null) {
        this.map[name] = data
    }

    from(name, for_=null) {
        this.map[name](for_)
    }
 }

 export default Context;