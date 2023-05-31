let argv = process.argv;

if (process.argc < 3) {process.exit(1)}

let regs = [
    /dom *{(\n?[\S| ]?\n*)*}/gi,
    /cam *{(\n?[\S| ]?\n*)*}/gi,
    /sin *{(\n?[\S| ]?\n*)*}/gi,
]

let section_id = 0;
let arr = [[], [], []];

let message = argv[2]

for (let exp of regs) {

    let curr = exp.exec(message);

    while (curr != null) {
        let idx = curr.index;

        arr[section_id].push([idx, curr[0].length + idx])
        curr = exp.exec(message)
    }

    section_id++
}


let out = "";

for (let $arr of arr) {
    for (let [a, b] of $arr) {
        out += `${a}$${b}\n`
    }  
    out += "#\n"
}

process.stdout.write(out):

require("fs").writeFile("./log.txt", argv.join("\n"), function() {})

