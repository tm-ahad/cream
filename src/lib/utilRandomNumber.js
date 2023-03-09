
const utilRandomNumber = () => {
    let first = Math.round(Math.random()*seed)

    let i = 0
    let newd = Math.round(Math.random()*seed)
    console.log(newd)

    while (newd != first) {
    	i++
    	newd = Math.round(Math.random()*seed)
    }

    return i
}

export default utilRandomNumber