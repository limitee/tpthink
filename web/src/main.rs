#[macro_use]
extern crate easy_util;

extern crate hyper;

use std::sync::Arc;
use std::collections::BTreeMap;

use std::io::prelude::*;
use std::fs::File;

extern crate url;
extern crate time;

use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

extern crate web;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

#[macro_use]
extern crate log;
extern crate elog;

extern crate service;
use service::ApiFactory;

extern crate util;
use util::DigestUtil;

extern crate cons;
use cons::CONS;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate filetime;
use filetime::FileTime;

extern crate chrono;
use chrono::*;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

/**
 * the uri type of the request.
 */
enum UriType {
    StaticFile(String),
    HtmlFile(String),
    JsApi(String),
    DataApi(String),
    FileApi(String),
    None,
}

struct SenderHandler {
    db: DataBase<MyDbPool>,
    api: ApiFactory,
}

impl SenderHandler {

    /**
     * get the js file content as text
     */
    pub fn js_api(&self, _req: Request, res: Response, path:&str) {
        let js_path = &path[7..];
        let relative_path = format!("./static/html{}", js_path);
        let mut res = res.start().unwrap();
        let mut f = File::open(&relative_path).unwrap();
        //let mut read_size = 0;
        let mut buffer = [0; 1000];
        loop {
            let read_size = f.read(&mut buffer).unwrap();
            if read_size == 0 {
                break;
            }
            res.write_all(&buffer[0..read_size]).unwrap();
        }
    }
    
    
    /**
     * get file from db.
     */
    pub fn file_api(&self, req: Request, mut res: Response, path:&str) {
        let ifms = req.headers.get_raw("If-Modified-Since");
        let send_content = match ifms {
            Some(_) => {
                false
            },
            None => {
                let m_seconds = time::get_time().sec;
                let m_seconds_str = format!("{}", m_seconds);
                let etag_str = format!("{}", m_seconds);
                let mut headers = res.headers_mut();
                headers.set_raw("Last-Modified", vec![m_seconds_str.into_bytes()]);
                headers.set_raw("ETag", vec![etag_str.into_bytes()]);

                true
            }
        };
        if send_content {
            let v: Vec<&str> = path.splitn(4, '/').collect();
            let file_id = i64::from_str(v[3]).unwrap();
            let file_table = self.db.get_table("file").expect("file_block table not exist.");
            let file_block_table = self.db.get_table("file_block").expect("file_block table not exist.");
            let cond = format!(r#"
                {{
                    "id":{}
                }}
            "#, file_id);
            let file = file_table.find_one_by_str(&cond, "{}", "{}").unwrap();
            let customer_id = file.find_path(&["customer_id"]).unwrap().as_i64().unwrap();
            let file_type = file.find_path(&["type"]).unwrap().as_i64().unwrap();
            let file_size = file.find_path(&["size"]).unwrap().as_i64().unwrap();
            let file_name = file.find_path(&["name"]).unwrap().as_string().unwrap();
            let file_size_str = format!("{}", file_size);
            let disp = format!("attachment; filename={}", file_name);
            let ct_type = CONS.id_to_code("file_type", file_type as i32).unwrap();
            println!("{}", ct_type);
            {
                let mut headers = res.headers_mut();
                headers.set_raw("Content-disposition", vec![disp.into_bytes()]);
                headers.set_raw("content-type", vec![ct_type.into_bytes()]);
                headers.set_raw("content-length", vec![file_size_str.into_bytes()]);
            }
            let mut res = res.start().unwrap();
            let mut index = 1;
            loop {
                let file_block_id = format!("{}_{}_{}", customer_id, file_id, index);
                let cond = format!(r#"
                    {{
                        "id":"{}"
                    }}
                "#, file_block_id);
                let data = file_block_table.find_one_by_str(&cond, "{}", "{}");
                match data {
                    Ok(x) => {
                        let content = x.find_path(&["content"]).unwrap().as_string().unwrap();
                        let vec = DigestUtil::base64_to_bytes(&content).unwrap();
                        res.write_all(&vec).unwrap();
                    },
                    Err(_) => {
                        break;
                    },
                }
                index += 1;
            }
        } else {
             let mut status = res.status_mut();
             *status = StatusCode::NotModified;
         }
    }

    /**
     * get data from server
     */
    pub fn data_api(&self, mut req: Request, mut res: Response, _path:&str) {
        let mut content = String::new();
        req.read_to_string(&mut content).unwrap();
        let kv = url::form_urlencoded::parse(content.as_bytes());

        let mut req_map = BTreeMap::new();
        for (key, value) in kv {
            req_map.insert(key, value);
        }
        let back = match self.api.check(&(self.db), &req_map) {
            Ok(ref msg) => {
                let uuid = json_str!(msg; "body", "uuid");
                let mut body_json = self.api.run(&(self.db), msg).unwrap();
                json_set!(&mut body_json; "uuid"; uuid);
                let body_str = body_json.to_string();
                self.api.back(msg, body_str)
            },
            Err(y) => {
                let head = Json::from_str(req_map.get("head").unwrap()).unwrap();
                let mut back_obj = BTreeMap::new();
                back_obj.insert("err".to_string(), y.to_json());
                self.api.back_err(&head, back_obj.to_json().to_string())
            },
        };
        let back_str = back.to_string();
        {
            let mut headers = res.headers_mut();
            headers.set(
                ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))
            );
        }
        let mut res = res.start().unwrap();
        res.write_all(back_str.as_bytes()).unwrap();

        //hi代表了一个头部的键值对
        /*
        for hi in req.headers.iter() {
            //println!("{}.", hi);
            let name = hi.name();
            let value = hi.value_string();
            println!("key: {}, value: {}.", name, value);
        }
        */
    }

    /**
     * static file.
     */
    pub fn static_file(&self, req: Request, mut res: Response, path:&str) {
        let mut f = File::open(path).unwrap();
        let metadata = f.metadata().unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let m_seconds = mtime.seconds_relative_to_1970();
        {
            //let utc: DateTime<UTC> = UTC::now(); 
            //let m_seconds_str = utc.to_rfc2822();
            let m_seconds_str = format!("{}", m_seconds);
            let etag_str = format!("{}", m_seconds);
            let mut headers = res.headers_mut();
            headers.set_raw("Last-Modified", vec![m_seconds_str.into_bytes()]);
            headers.set_raw("ETag", vec![etag_str.into_bytes()]);
            //headers.set_raw("Cache-Control", vec!["max-age=60, must-revalidate".to_string().into_bytes()]);
            if path.ends_with(".js") {
                headers.set(
                    ContentType(Mime(TopLevel::Application, SubLevel::Javascript, vec![]))
                );
            }
            else if path.ends_with(".css") {
                headers.set(
                    ContentType(Mime(TopLevel::Text, SubLevel::Css, vec![]))
                );
            }
        }
        let send_content = {
            let ifms = req.headers.get_raw("If-Modified-Since");
            match ifms {
                Some(x) => {
                    let lmt_str = String::from_utf8(x.get(0).unwrap().clone()).unwrap();
                    let cache_time_rst = u64::from_str(&lmt_str);
                    if cache_time_rst.is_err() {
                        let dt_rst = DateTime::parse_from_rfc2822(&lmt_str);
                        let send_content = match dt_rst {
                            Ok(dt) => {
                                let timestamp = dt.timestamp() as u64;
                                //println!("change time:{}--{}", m_seconds, timestamp);
                                if m_seconds > timestamp {
                                    true
                                } else {
                                    false
                                }
                            },
                            Err(_) => {
                                true
                            },
                        };
                        send_content 
                    } else {
                        let cache_time = cache_time_rst.unwrap();
                        if m_seconds > cache_time {
                            true 
                        } else {
                            false
                        }
                    }
                },
                None => {
                    true
                }
            }
        };
        //send_content = true;
        if send_content {
            let mut res = res.start().unwrap();
            //let mut read_size = 0;
            let mut buffer = [0; 1000];
            loop {
                let read_size = f.read(&mut buffer).unwrap();
                if read_size == 0 {
                    break;
                }
                res.write_all(&buffer[0..read_size]).unwrap();
            }
        } else {
            let mut status = res.status_mut();
            *status = StatusCode::NotModified;
        }
                
        /* 
        //hi代表了一个头部的键值对
        for hi in req.headers.iter() {
            //println!("{}.", hi);
            let name = hi.name();
            let value = hi.value_string();
            println!("key: {}, value: {}.", name, value);
        }
        */ 
    }

    /**
     * html file.
     */
    pub fn html_file(&self, req: Request, res: Response, path:&str) {
        let relative_path = self.get_html_file_path(path);
        println!("html:{}", relative_path);
        self.static_file(req, res, &relative_path);
    }

    fn get_html_file_path(&self, path:&str) -> String {
        let mut file_path = "./static/html".to_string();
        let path_vec: Vec<&str> = path.split('_').collect();
        for i in 0..path_vec.len() {
            if i > 0 {
                file_path = file_path + "/";
            }
            file_path = file_path + path_vec[i];
        }
        file_path
    }

}

impl Handler for SenderHandler {

    fn handle(&self, req: Request, res: Response) {
        let uri_type = match req.uri {
            RequestUri::AbsolutePath(ref path) => {
                info!("the path is {}.", path);
                let mut path_array:Vec<&str> = path.split('?').collect();
                let path = path_array.remove(0);
                if path == "/" {
                    UriType::HtmlFile("/index.html".to_string())
                }
                else if path.starts_with("/js") || path.starts_with("/css") || path.starts_with("/fonts") || path.starts_with("/img")
                {
                    let re_path = "./static".to_string() + path;
                    UriType::StaticFile(re_path)
                }
                else if path.ends_with(".html")
                {
                    UriType::HtmlFile(path.to_string())
                }
                else if path.starts_with("/api/js") {
                    UriType::JsApi(path.to_string())
                }
                else if path.starts_with("/api/data") {
                    UriType::DataApi(path.to_string())
                }
                else if path.starts_with("/api/file") {
                    UriType::FileApi(path.to_string())
                }
                else
                {
                    UriType::None
                }
            },
            _ => {
                UriType::None
            },
        };

        match uri_type {
            UriType::StaticFile(ref path) => {
                self.static_file(req, res, path);
            },
            UriType::HtmlFile(ref path) => {
                self.html_file(req, res, path);
            },
            UriType::JsApi(ref path) => {
                self.js_api(req, res, path);
            },
            UriType::DataApi(ref path) => {
                self.data_api(req, res, path);
            },
            UriType::FileApi(ref path) => {
                self.file_api(req, res, path);
            },
            _ => {

            },
        }
    }
}

fn main() {
    let _ = elog::init();
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db = DataBase::new("main", Arc::new(my_pool));
    let api = ApiFactory::new();

    let url = format!("0.0.0.0:{}", cfg_i64!("hyper", "port"));
    Server::http(url.as_str()).unwrap().handle_threads(SenderHandler {
        db: my_db,
        api: api,
    }, cfg_i64!("hyper", "thread_count") as usize).unwrap();

}

