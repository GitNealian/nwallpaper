use std::{
    collections::HashMap, error::Error, fmt::Display, fs::File, io::Write, path::Path, thread,
};

use rhai::{Engine, Scope};

pub struct Source {}
#[derive(Debug)]
pub enum ScriptError {
    ReturnValueNotFound(String),
}

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScriptError::ReturnValueNotFound(name) => {
                write!(f, "variable \"{}\" is not found in return values", name)
            }
        }
    }
}

impl Error for ScriptError {}

pub struct Repository {}

impl Source {
    fn new_engine() -> Result<Engine, Box<dyn std::error::Error>> {
        let mut engine = Engine::new();
        engine.register_fn("http_get", http_get);
        engine.register_fn("http_post", http_post);
        engine.set_max_expr_depths(0, 0);
        let engine = engine;
        return Ok(engine);
    }

    pub fn new() -> Self {
        let engine = Self::new_engine().unwrap();
        Source {}
    }

    pub async fn get_image<F>(
        &self,
        repository: &str,
        query: String,
        page: i32,
        page_size: i32,
        w: i32,
        h: i32,
        callback: &'static F,
    ) -> Result<i64, Box<dyn Error>>
    where
        F: Fn(i32, String) + Send + Sync,
    {
        let image_url = self.get_image_url(repository, query, page, page_size, w, h)?;
        // 判断url是否已下载
        thread::spawn(move || {
            let mut index = 0;
            for url in image_url.1 {
                let image_path = format!("{}", time::get_time().nsec);
                if let Ok(_) = download_image(&url, &image_path) {
                    callback(index, image_path);
                }
                index = index + 1;
            }
        });
        return Ok(image_url.0);
    }

    fn get_image_url(
        &self,
        repository: &str,
        query: String,
        page: i32,
        page_size: i32,
        w: i32,
        h: i32,
    ) -> Result<(i64, Vec<String>), Box<dyn Error>> {
        let engine = Self::new_engine()?;
        let ast = engine
            .compile_file("source/bing_daily.rhai".into())
            .unwrap();
        let mut scope = Scope::new();
        scope.push("store", engine.parse_json("#{}", false).unwrap());
        let rst: rhai::Map =
            engine.call_fn(&mut scope, &ast, "list", (query, page, page_size, w, h))?;
        if let Some(total) = rst.get("total") {
            if let Some(l) = rst.get("list") {
                Ok((
                    total.as_int()?,
                    l.clone_cast::<rhai::Array>()
                        .iter()
                        .map(|x| x.clone_cast::<String>())
                        .collect(),
                ))
            } else {
                Err(Box::new(ScriptError::ReturnValueNotFound(String::from(
                    "total",
                ))))
            }
        } else {
            Err(Box::new(ScriptError::ReturnValueNotFound(String::from(
                "list",
            ))))
        }
    }
}

fn http_get(url: &str) -> String {
    let client = reqwest::blocking::Client::new().get(url);
    if let Ok(resp) = client.send() {
        return resp.text().unwrap_or(String::new());
    }
    return String::new();
}

fn http_post(url: &str, params: rhai::Array) -> String {
    let mut client = reqwest::blocking::Client::new().post(url);
    let mut map = HashMap::new();
    for i in 0..(params.len()) {
        map.insert(
            params[i * 2 as usize].clone().cast::<&str>(),
            params[(i * 2 + 1) as usize].clone().cast::<&str>(),
        );
    }
    client = client.json(&map);
    if let Ok(resp) = client.send() {
        return resp.text().unwrap_or(String::new());
    }
    return String::new();
}

fn download_image(url: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::Client::new().get(url);
    let path = Path::new(dst);
    let mut file = File::create(&path)?;
    let content = response.send()?.bytes()?;
    file.write_all(&content[..])?;
    Ok(())
}
