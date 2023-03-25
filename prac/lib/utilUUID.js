
const hash = {
    '1': 'a',
    '2': 'b'
    '3': 'c',
    '4': 'd',
    '5': 'e',
    '6': 'f',
    '7': 'g',
    '8': 'h',
    '9': 'i'
}

const utilUUID = () => {
    let str = '$' + Math.random().toString()

    for (let v in str) {
        str.replaceAll(v, hash[v])
    }

    return str
}

export default utilUUID