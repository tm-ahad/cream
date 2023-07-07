const fs = require('fs');

function replaceRange(string, start, end) {
    return string.substring(0, start) +
        '%'.repeat(end-start) +
        string.substring(end)
}

fs.readFile('./build/mp.chan', 'utf8', (err, data) => {
    let point = data.indexOf('&');

    let match = data.substring(0, point);
    let message = data.substring(point);

    if (err) {
        return;
    }

    let out = "";

    if (match) {

        let exp = match == "<temp>" ? new RegExp("<temp> *(\n?[\\S| ]?\n*)*<temp/>") :
            new RegExp(`${match} *{(\n?[\\S| ]?\n*)*}`, 'i');

        let mat = message.match(exp)

        out = `${message.search(exp)}&${mat ? mat[0]: null}`
    } else {
        let regs = [
            /dom *{(\n?[\S| ]?\n*)*}/i,
            /cam *{(\n?[\S| ]?\n*)*}/i,
            /sin *{(\n?[\S| ]?\n*)*}/i,
        ]

        let section_id = 0;
        let arr = [[], [], []];

        for (let exp of regs) {
            let curr = exp.exec(message);

            while (curr != null) {
                let idx = curr.index;

                let start = idx;
                let end = start + curr[0].length;

                arr[section_id].push([start, end])
                message = replaceRange(message, start, end)

                message.match(exp).forEach((element) => {
                    console.log(element);
                });
                curr = exp.exec(message)
            }

            section_id++
        }

        for (let $arr of arr) {
            out += "#\n"
            for (let match of $arr) {
                out += `${match[0]}$${match[1]} `
            }

            out += "\n"
        }
}

    let emp = new Function();

    fs.writeFile("./build/mp.chan", "\n" + out, emp)
});
