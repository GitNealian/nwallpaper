// 在下拉框中显示的名字
fn source_name() {
    return "Bing每日图片";
}

fn api() {
    return "http://bing.getlove.cn/latelyBingImageStory";
}

fn bing() {
    return "https:";
}

/**********************
let store = #{
    list: [],
    last_update: 
}
 **********************/

// 每页查询
fn list(query, page, page_size, w, h){
    let start = page * page_size;
    let limit = page_size;
    // 只有第一页从接口请求数据
    if page == 0 {
        let response = http_get(api());
        if response != "" {
            response.replace("{","#{");
            eval("let response = " + response + ";");
            response = response.map(|x| #{url:bing() + x.CDNUrl,name:x.copyright});
            if store.list == () || store.list.len == 0 {
                store.list = response;
            }else{
                let first = store.list[0];
                let index = response.index_of(|x| x.url == last.url);
                if index == -1 {
                    index = response.len;
                }
                let update_arr = response.extract(0, index);
                update_arr.append(update_arr);
                store.list = update_arr;
            }
        }
    }else{
        if store.list == () || store.list == 0{
            return #{
                total: 0,
                list: [],
            }
        }
    }
    if store.len <= start {
        return #{
            total: store.list.len,
            list: [],
        }
    }
    if limit > store.list.len - start {
        limit = store.list.lenn - start;
    }
    
    return #{
        total: store.list.len,
        list: store.list.extract(start, limit)
    }
} 

fn next(query) {
    let rst = http_get(api());
    if (rst!=""){
        rst.replace("{","#{");
        eval(rst);
        return bing() + l[0].url;
    }
    return "";
}