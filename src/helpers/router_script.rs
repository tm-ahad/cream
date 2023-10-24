pub fn router_script() -> String {
    String::from(
        "\
function main() {
    var path = window.location.pathname
    let not_found = true
    for (let k in Route) {
        if (path == k) {
            not_found = false
            document.body.innerHTML = Route[k]
            window.history.pushState({}, \"\", k)
        }
    }
    if (not_found) {
        document.body.innerHTML = Route[\"404\"]
        window.history.pushState({}, \"\", path)
    }
}
main()
    ",
    )
}
