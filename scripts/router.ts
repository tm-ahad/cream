type paramType = {
    [key: string]: string
}
type routerFn = (_: paramType) => void
type routerMap = {
    [key: string]: routerFn
}

function router(map: routerMap) {
    var p = map[window.location.pathname];

    if (p == undefined) {
        let curr_tok = window.location.pathname.split("/").filter(Boolean)
        for (var [key, val] of Object.entries(map)) {
            let key_tok = key.split("/").filter(Boolean)
            if (key_tok.length != curr_tok.length) {
                continue;
            }

            let param_data: paramType = {}
            let match = true

            for (var i=0; i<key_tok.length; i++) {
                if (!key_tok[i].startsWith(":") && key_tok[i] != curr_tok[i]) {
                    match = false
                    break;
                } else if (key_tok[i].startsWith(":")) {
                    param_data[key_tok[i].substring(1)] = curr_tok[i]
                }   
            }

            if (match) {val(param_data)}
        } 
    } else {p({})}
}