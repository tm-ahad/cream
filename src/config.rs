use std::fs::read_to_string;

pub struct Config {
    pub name: String,
    pub home: String,
    pub static_dir: String,
    pub static_dir_render: String,
    pub lang: String,
    pub head: String,
    pub pre_make: String,
    pub build: String,
    pub pre_start: String,
    pub keywords: String,
    pub rcwp: String,
    pub script: String,
    pub author: String,
    pub description: String,
    pub title: String,
    pub port: String,
    pub host: String,
    pub _app_js: String,
    pub _app_html: String
}

impl Config {
    pub fn new() -> Self {
        let empty = String::new();

        Config {
            name: String::new(),
            home: String::new(),
            static_dir: String::new(),
            static_dir_render: String::new(),
            lang: String::new(),
            head: String::new(),
            pre_make: String::new(),
            build: String::new(),
            pre_start: String::new(),
            keywords: String::new(),
            rcwp: String::new(),
            script: String::new(),
            author: String::new(),
            description: String::new(),
            title: String::new(),
            port: String::new(),
            host: String::new(),
            _app_js: String::new(),
            _app_html: String::new(),
        }
    }

    pub fn load(&mut self, path: String) {
        let cont = read_to_string(path)
            .unwrap_or_else( |e| panic!("{e}"));

        let lines = cont.lines();

        for lin in lines {
            let (k, v) = match lin.split_once('$') {
                Some(a) => a,
                None => panic!("Invalid pair"),
            };

            match k {
                "name" => self.name = v.to_string(),
                "home" => self.home = v.to_string(),
                "static_dir" => self.static_dir = v.to_string(),
                "static_dir_render" => self.static_dir_render = v.to_string(),
                "lang" => self.lang = v.to_string(),
                "head" => self.head = v.to_string(),
                "pre_make" => self.pre_make = v.to_string(),
                "build" => self.build = v.to_string(),
                "pre_start" => self.pre_start = v.to_string(),
                "keywords" => self.keywords = v.to_string(),
                "rcwp" => self.rcwp = v.to_string(),
                "script" => self.script = v.to_string(),
                "author" => self.author = v.to_string(),
                "description" => self.description = v.to_string(),
                "title" => self.title = v.to_string(),
                "port" => self.port = v.to_string(),
                "host" => self.host = v.to_string(),
                "_app_js" => self._app_js = v.to_string(),
                "_app_html" => self._app_html = v.to_string(),
                _ => panic!("Field not found for key: {}", k),
            };
        }   
    }

    pub fn get(&self, prop: &str) -> Option<&String> {
        match prop {
            "name" => Some(&self.name),
            "home" => Some(&self.home),
            "static_dir" => Some(&self.static_dir),
            "static_dir_render" => Some(&self.static_dir_render),
            "lang" => Some(&self.lang),
            "head" => Some(&self.head),
            "pre_make" => Some(&self.pre_make),
            "build" => Some(&self.build),
            "pre_start" => Some(&self.pre_start),
            "keywords" => Some(&self.keywords),
            "rcwp" => Some(&self.rcwp),
            "script" => Some(&self.script),
            "author" => Some(&self.author),
            "description" => Some(&self.description),
            "title" => Some(&self.title),
            "port" => Some(&self.port),
            "host" => Some(&self.host),
            "_app_js" => Some(&self._app_js),
            _ => None,
        } 
    }

    pub fn expect(&self, prop: &str) -> &String {
        &self.get(prop)
            .unwrap_or_else(|| panic!("Property {prop} not found on configuration"))
    }

    pub fn get_or(&self, prop: &str, or: &str) -> &String {
        match &self.get(prop) {
            Some(v) => v,
            None => &String::from(or)
        }
    }
    
}