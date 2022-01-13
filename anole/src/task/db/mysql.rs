
use std::{collections::HashMap, vec};

use log::debug;
use sqlx::{mysql::*, ConnectOptions, Pool, Executor, Row, Column, types::time};

use crate::{context::Context, task, capture::{Capture, Cap}, value::Value};

use super::DBClientOption;


#[derive(Debug)]
pub struct MysqlTask<'a> {
    //options
    options: Option<DBClientOption<'a>>,
    client: Option<MysqlClient>,
    tasks: Vec<DBTask<'a>>,
}

impl <'a> MysqlTask<'a> {
    
    pub fn new() -> Self {
        MysqlTask { options: None, client: None, tasks: vec![] }
    }

    pub fn options(mut self, options: task::db::DBClientOption<'a>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_task(mut self, t: DBTask<'a>) -> Self {
        self.tasks.push(t);
        self
    }

    pub async fn execute(self, ctx: &mut Context) -> crate::Result<()> {
        let options = match self.options {
            Some(o) => o,
            None => return Err(crate::error::create_client("DBClientOptions Empty".into()))
        };
        let mut client = MysqlClient::default();
        match client.create(options).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        for tt in self.tasks {
            match client.execute(tt, ctx).await {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }


}

#[derive(Debug)]
pub struct DBTask<'a> {
    // client: Vec<dyn DBClient>
    pub sql: &'a str,
    params: Option<Vec<&'a str>>,
    capture: Option<Vec<Capture<'a>>>
}

impl <'a> DBTask<'a> {
    
    pub fn new(sql: &'a str) -> Self {
        DBTask {
            sql,
            params: None,
            capture: None
        }
    }

    pub fn param(mut self, p: &'a str) -> Self {
        let params =  self.params.get_or_insert(vec![]);
        params.push(p);
        self
    }

    pub fn capture(mut self, caps: Vec<Capture<'a>>) -> Self {
        self.capture = Some(caps);
        self
    }

    pub(crate) fn handle_rows(&self, rows: Vec<MySqlRow>, ctx: &mut Context) -> crate::Result<()> {
        if self.capture.is_none() {
            return Ok(())
        }
        if rows.is_empty() {
            return Ok(())
        }
        if let Some(ref _caps) = self.capture {
            for (idx, r) in rows.into_iter().enumerate() {
                for _cap in _caps {
                    if let Capture::Column(ref _c) = _cap {
                        ctx.store.set(_c.save_key.to_owned(), Value::U32(idx as u32));
                        if _c.is_usize() {
                            if let Ok(vv) = r.try_get::<i64, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::U32(vv as u32));
                            }
                        } else if _c.is_i32() || _c.is_i64() {
                            if let Ok(vv) = r.try_get::<i32, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::I32(vv));
                            }
                        } else if _c.is_u32() || _c.is_u64() {
                            if let Ok(vv) = r.try_get::<u32, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::U32(vv));
                            }
                        } else if _c.is_bool() {
                            if let Ok(vv) = r.try_get::<bool, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::Bool(vv));
                            }
                        } else if _c.is_str() {
                            if let Ok(vv) = r.try_get::<&str, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::Str(vv.to_string()));
                            }
                        } else if _c.is_date() {
                            if let Ok(vv) = r.try_get::<time::Date, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::Date(vv));
                            }
                        } else if _c.is_time() {
                            if let Ok(vv) = r.try_get::<time::Time, &str>(_c.key) {
                                ctx.store.set(_c.save_key.to_owned() + "|" + &idx.to_string(), Value::Time(vv));
                            }
                        } else {
                            return Err(crate::error::unimplement("unsupport type"));
                        }
                    }
                }
                // debug!("column_name:{:?}", r.get::<&str, usize>(0));
                // let date: time::Date = r.get(1);
                // debug!("column_date:{}", date);
            }
        }
        Ok(())
    }

}

// trait DBClient {
//     fn create(&mut self, options: DBClientOption);
//     fn execute<R>(&mut self, sql: &str) -> R;
// }

#[derive(Debug, Default)]
struct MysqlClient {
    pool: Option<Pool<MySql>>
}

impl MysqlClient {

    async fn create(&mut self, options: DBClientOption<'_>) -> crate::Result<()> {
        let pool = match MySqlPoolOptions::new()
            .max_connections(options.max_connections)
            .connect(options.url).await {
                Ok(p) => p,
                Err(e) => return Err(crate::error::create_client(e.into()))
            };

        self.pool = Some(pool);
        
        Ok(())
    }

    async fn execute(&self, t: DBTask<'_>, ctx: &mut Context) -> crate::Result<()> {
        let pool = &self.pool.as_ref().unwrap();

        let mut sql = t.sql.to_owned();

        if let Some(_params) = &t.params {
            // let _params = _params.to_owned();
            for _k in _params {
                if let Some(v) = ctx.store.get(_k.to_string()) {
                    sql = sql.replace(format!("#{}#", _k).as_str(), v.as_str().as_str());
                }
            }
        }
        debug!("execute_sql:{}", sql);

        let rows = match sqlx::query(&sql).fetch_all(*pool).await {
            Ok(r) => r,
            Err(e) => return Err(crate::error::request(e.into()))
        };
        t.handle_rows(rows, ctx)
        // for r in rows {
        //     debug!("column_name:{:?}", r.get::<&str, usize>(0));
        //     let date: time::Date = r.get(1);
        //     debug!("column_date:{}", date);
        // }
        // Ok(())
    }

}
