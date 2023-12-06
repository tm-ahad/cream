function router(map) {
    var p = map[window.location.pathname];
    if (window.location.pathname == "/") {
        return
    }

    p = p === undefined ? map["/error"]: p;

    document.body.innerHTML = p[0];
    new Function(p[1])()
}