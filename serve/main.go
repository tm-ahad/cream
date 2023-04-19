package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
)

var config, _ = os.ReadFile("./config.dsp")

func get_dsp_val(data []byte, key string, def string) string {
    fields := strings.Split(string(data), "\n")

    for _, f := range fields {
        key_l := len(key)

        if f != "" && len(f) > len(key) && f[:key_l] == key {
            return f[key_l+1:]
        }
    }

    return def
}

func app(w http.ResponseWriter, req *http.Request) {
    arg := fmt.Sprintf("./%s", get_dsp_val(config, "_app_html", "build/index.html"))
    body, _ := ioutil.ReadFile(arg)

    fmt.Fprintf(w, string(body))
}

func main() {
    stat := fmt.Sprintf("./%s", get_dsp_val(config, "static_dir", ""))
    s_path := fmt.Sprintf("%s/", get_dsp_val(config, "static_dir_render", ""))
    addr := fmt.Sprintf("%s:%s", get_dsp_val(config, "host", "localhost"),
        get_dsp_val(config, "port", "8871"))

    entry := get_dsp_val(config, "home", "/");

    if s_path != "/" && stat != "./" {
        http.Handle(s_path, http.StripPrefix(s_path,
            http.FileServer(http.Dir(stat))))
    }

    http.HandleFunc(entry, app)

    fmt.Printf("Serving App on http://%s\n", addr)
    http.ListenAndServe(addr, nil)
}