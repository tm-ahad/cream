function router(map) {
    var p = map[window.location.pathname];
    var err = map["error"];

    if (window.location.search == "/") {
        return
    }

    if (p != null) {
        window.location.replace(p);
    } else {
        window.location.replace(err);
    }
}
