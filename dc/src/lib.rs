use std::sync::{Arc, Mutex};

extern crate easydb;
use easydb::Column;
use easydb::Table;
use easydb::DbPool;

use std::collections::BTreeMap;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate postgres;
use postgres::{Connection, SslMode};
use postgres::types::Type;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

#[macro_use]
extern crate log;

pub struct MyDbPool {
    dsn:String,
    conns:Vec<Mutex<Connection>>,
}

pub fn get_back_json(rows:postgres::rows::Rows) -> Json {
    let mut rst_json = json!("{}");
    let mut data:Vec<Json> = Vec::new();
    for row in &rows {
        let mut back_json = json!("{}");
        let columns = row.columns();
        for column in columns {
            let name = column.name();
            let col_type = column.type_();
            match *col_type {
                Type::Int4 => {
                    let op:Option<postgres::Result<i32>> = row.get_opt(name);
                    let mut true_value:i32 = 0;
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(&mut back_json; name; true_value);
                },
                Type::Int8 => {
                    let op:Option<postgres::Result<i64>> = row.get_opt(name);
                    let mut true_value:i64 = 0;
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(back_json; name; true_value);
                },
                Type::Varchar | Type::Text => {
                    let op:Option<postgres::Result<String>> = row.get_opt(name);
                    let mut true_value:String = String::new();
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(back_json; name; true_value);
                },
                _ => {
                    println!("ignore type:{}", col_type.name());
                },
            }
        }
        data.push(back_json);
    }
    json_set!(&mut rst_json; "data"; data);
    json_set!(&mut rst_json; "rows"; rows.len());
    rst_json
}

impl MyDbPool {

    pub fn new(dsn:&str, size:u32) -> MyDbPool {
        let mut conns = vec![];
        for _ in 0..size {
            let conn = match Connection::connect(dsn, SslMode::None) {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Connection error: {}", e);
                    break;
                }
            };
            conns.push(Mutex::new(conn));
        }
        MyDbPool {
            dsn:dsn.to_string(),
            conns:conns,
        }
    }

    /**
     * 获得dsn字符串
     */
    pub fn get_dsn(&self) -> String {
        self.dsn.clone()
    }

}

impl DbPool for MyDbPool {

    fn get_connection(&self) -> Result<Connection, i32> {
        let rst = match Connection::connect(self.dsn.as_str(), SslMode::None) {
            Ok(conn) => Result::Ok(conn),
            Err(e) => {
                println!("Connection error: {}", e);
                Result::Err(-1)
            }
        };
        rst
    }

    fn execute(&self, sql:&str) -> Result<Json, i32> {
        info!("{}", sql);
        let between = Range::new(0, self.conns.len());
        let mut rng = rand::thread_rng();
        let rand_int = between.ind_sample(&mut rng);
        let conn = self.conns[rand_int].lock().unwrap();

        let out_rst = {
            let rst = conn.query(sql, &[]);
            rst.and_then(|rows| {
                Result::Ok(get_back_json(rows))
            })
        };

        match out_rst {
            Ok(json) => {
                Result::Ok(json)
            },
            Err(err) => {
                println!("{}", err);
                Result::Err(-1)
            },
        }
    }
    
    fn stream<F>(&self, sql:&str, f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
	    Result::Ok(1)	
	}

}

pub fn stream<F>(conn:Connection, sql:&str, mut f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
    let rst = conn.query("BEGIN", &[]);

    //begin
    let rst = rst.and_then(|rows| {
        let json = get_back_json(rows);
        println!("{}", json);
        Result::Ok(1)
    }).or_else(|err|{
        println!("{}", err);
        Result::Err(-1)
    });

    //cursor
    let rst = rst.and_then(|_| {
		let cursor_sql = format!("DECLARE myportal CURSOR FOR {}", sql);
		println!("{}", cursor_sql);
		let rst = conn.query(&cursor_sql, &[]);
		rst.and_then(|rows|{
            let json = get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    let rst = rst.and_then(|_| {
        let fetch_sql = "FETCH NEXT in myportal";
        println!("{}", fetch_sql);

        let mut flag = 0;
        loop {
            let rst = conn.query(&fetch_sql, &[]);
            let _ = rst.and_then(|rows|{
                let json = get_back_json(rows);
                let rows = json_i64!(&json; "rows");
                if rows < 1 {
                    flag = -2;
                } else {
                    let f_back = f(json);
                    if !f_back {
                        flag = -2;
                    }
                }
                Result::Ok(flag)
            }).or_else(|err|{
                println!("{}", err);
                flag = -1;
                Result::Err(flag)
            });
            if flag < 0 {
                break;
            }
        }
        match flag {
            -1 => {
                Result::Err(-1)
            },
            _ => {
                Result::Ok(1)
            },
        }
    });

    //close the portal
    let rst = rst.and_then(|_|{
  		let close_sql = "CLOSE myportal";
        println!("{}", close_sql);
        let rst = conn.query(&close_sql, &[]);
        rst.and_then(|rows|{
            let json = get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    //end the cursor
    let rst = rst.and_then(|_|{
    	let end_sql = "END";
        println!("{}", end_sql);
        let rst = conn.query(&end_sql, &[]);
        rst.and_then(|rows|{
            let json = get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })		
   	});
    rst
}


pub struct DataBase<T> {
    pub name:String,
    pub table_list:BTreeMap<String, Table<T>>,
    pub dc:Arc<T>,   //data center
}

impl<T:DbPool> DataBase<T> {

    pub fn get_connection(&self) -> Result<Connection, i32> {
        self.dc.get_connection()
    }

    fn get_table_define(name:&str, vec:Vec<Column>, dc:Arc<T>) -> Table<T>
    {
        let mut map = BTreeMap::new();
        for col in vec {
            map.insert(col.name.clone(), col);
        }
        Table::new(name, map, dc)
    }

    pub fn new(name:&str, dc:Arc<T>) -> DataBase<T>
    {
        let mut table_list = BTreeMap::new();
        {   //the user's st
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigint", -1, "unique not null", false),
                Column::new("st", "varchar", 32, "not null default ''", false),
                Column::new("fix_st", "varchar", 32, "not null default ''", false),
                Column::new("role", "integer", -1, "default -1", false),
                Column::new("last_active_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("st", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the customer
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("username", "varchar", 40, "unique not null", false),
                Column::new("nickname", "varchar", 40, "not null default ''", true),
                Column::new("password", "varchar", 40, "not null", false),
                Column::new("reg_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("avatar_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("customer", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {
        		let dc = dc.clone();
        		let vec = vec![
                Column::new("id", "bigserial", -1, "not null primary key", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
                Column::new("title", "varchar", 200, "not null", true),
                Column::new("content", "text", -1, "not null", true),
                Column::new("create_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("document", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("name", "varchar", 80, "not null", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("size", "bigint", -1, "default -1", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("file", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file block table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "varchar", 80, "PRIMARY KEY", false),
                Column::new("file_id", "bigint", -1, "", false),
                Column::new("customer_id", "bigint", -1, "", false),
                Column::new("start", "bigint", -1, "", false),
                Column::new("index", "int", -1, "", false),
                Column::new("size", "int", -1, "", false),
                Column::new("content", "text", -1, "not null", false),
            ];
            let table = DataBase::get_table_define("file_block", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        for (_, table) in table_list.iter() {
            info!("{}", table.to_ddl_string());
        }
        DataBase {
            name:name.to_string(),
            table_list:table_list,
            dc:dc,
        }
    }

    pub fn get_table(&self, name:&str) -> Option<&Table<T>>
    {
        self.table_list.get(name)
    }

    pub fn execute(&self, sql:&str) -> Result<Json, i32> {
        self.dc.execute(&sql)
    }
	
	pub fn stream<F>(&self, sql:&str, f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
		self.dc.stream(sql, f)
	}
}

