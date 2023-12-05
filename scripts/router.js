function router(map) {
    var p = map[window.location.pathname];
    var err = p["error"];

    if (p != null) {
        window.location.replace(p);
    } else {
        window.location.replace(err);
    }
}
