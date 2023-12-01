function Enum(fields) {
    var len = fields.length
    for (let i = 0; i < fields.length; i++) {
        this[fields[i]] = Symbol()
    }
}