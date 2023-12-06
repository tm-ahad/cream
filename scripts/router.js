function router(map) {
    var p = map[window.location.pathname];
    var err = map["error"];

    if (window.location.search == "/") {
        return
    }

    if (p != null) {
        document.body.innerHTML = p
    } else {
        document.body.innerHTML = err;
    }
}
