use std::collections::HashMap;

use rhai::Engine;
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
        map.insert(params[i*2 as usize].clone().cast::<&str>(), params[(i*2+1) as usize].clone().cast::<&str>());
    }
    client = client.json(&map);
    if let Ok(resp) = client.send() {
        return resp.text().unwrap_or(String::new());
    }
    return String::new();
}

fn hello() -> String {
    return String::from("hello");
}


fn source_init() -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();
    
    engine.register_fn("http_get", http_get);
    engine.register_fn("http_post", http_post);
    engine.register_fn("hello", hello);
    engine.set_max_expr_depths(0, 0);
    let engine = engine;
    return Ok(engine);
}


#[test]
fn test_bing_daily_rhai(){
    use rhai::Scope;
    use rhai::EvalAltResult;
    let engine = source_init().unwrap();
    let ast = engine.compile_file("source/bing_daily.rhai".into()).unwrap();
    let mut scope = Scope::new();
    scope.push("store", engine.parse_json("#{}", false).unwrap());
    let rst:Result<rhai::Map, Box<EvalAltResult>> = engine.call_fn(&mut scope, &ast, "list", ("", 0_i64,10_i64,0,0));
    // let rst:Result<rhai::Dynamic, Box<EvalAltResult>> = engine.call_fn(&mut scope, &ast, "next", (""));
    match rst {
        Ok(l) => {
            print!("{:?}\n\n\n", scope.get_value::<rhai::Dynamic>("store").unwrap_or_default());
            print!("{:?}", l);
        },
        Err(err) => {
            print!("{:?}", err)
        }
    }
}