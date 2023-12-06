function router(map) {
    var p = map[window.location.pathname];
    var err = map["error"];
    if (window.location.pathname == "/") {
        return
    }

    if (p[1] != null) {
        document.body.innerHTML = p[1]
    } else {
        document.body.innerHTML = err;
    }

    new Function(p[0])()
}
