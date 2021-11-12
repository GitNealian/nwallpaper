use std::{collections::HashMap, error::Error, fmt::Display, fs::File, io::Write, path::Path};

use super::app::Action;
use glib::Sender;
use rhai::{Engine, Scope};

#[derive(Debug, Clone)]
pub struct Source {
    sender: Sender<Action>,
}
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

    pub(crate) fn new(sender: Sender<Action>) -> Self {
        Source { sender }
    }

    pub fn get_image(
        &self,
        repository: &str,
        query: String,
        page: i64,
        page_size: i64,
        w: i64,
        h: i64,
    ) -> Result<i64, Box<dyn Error>> {
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
                let mut index = 0;
                let items = l.to_owned().cast::<rhai::Array>();
                self.sender.send(Action::ShowImagePlaceHolder(items.len() as i32)).unwrap();
                for item in items {
                    let map = item.cast::<rhai::Map>();
                    let image_path = format!("{}", time::get_time().nsec);
                    let url = map.get("url").unwrap().to_owned().cast::<String>();
                    let name  = map.get("name").unwrap().to_owned().cast::<String>();
                    if let Ok(_) = download_image(
                        &url,
                        &image_path,
                    ) {
                        self.sender
                            .send(Action::ShowImage(image_path, name,  index.clone()))
                            .unwrap();
                    }
                    index = index + 1;
                }
                return Ok(total.as_int()?);
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
    print!("正在下载图片：{}\n", url);
    let response = reqwest::blocking::Client::new().get(url);
    let path = Path::new(dst);
    let mut file = File::create(&path)?;
    let content = response.send()?.bytes()?;
    file.write_all(&content[..])?;
    print!("下载完成：{}\n", url);
    Ok(())
}
