
use std::{vec, time::Duration};

use sqlx::{mysql::{self, *}, Pool, Row, types::time, ConnectOptions};

use crate::{context::Context, task, capture::Capture, value::Value, faker, report::ReportItem};

use super::DBClientOption;


#[derive(Debug)]
pub struct MysqlTask<'a> {
    //options
    options: Option<DBClientOption<'a>>,
    tasks: Vec<DBTask<'a>>,
}

impl<'a> MysqlTask<'a> {

    pub fn default() -> Self {
        MysqlTask { options: None, tasks: vec![] }
    }
    
    pub fn options(mut self, options: task::db::DBClientOption<'a>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_task(mut self, t: DBTask<'a>) -> Self {
        self.tasks.push(t);
        self
    }

    pub async fn execute(&self, ctx: &mut Context) -> crate::Result<()> {
        let options = match &self.options {
            Some(o) => o,
            None => return Err(crate::error::create_client("DBClientOptions Empty".into()))
        };
        let mut client = MysqlClient::default();
        match client.create(options).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        for tt in &self.tasks {
            match client.execute(tt, ctx).await {
                Ok(r) => {
                    ctx.report(r);
                    continue;
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DBTask<'a> {
    pub sql: &'a str,
    params: Option<Vec<&'a str>>,
    capture: Option<Vec<Capture<'a>>>,
    expect: Option<(&'a str, Value)>,
    pub task_id: String,
}

impl <'a> DBTask<'a> {
    
    pub fn new(sql: &'a str) -> Self {
        DBTask {
            sql,
            params: None,
            capture: None,
            task_id: faker::uuid_v4(),
            expect: None,
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

    pub fn expect(mut self, tup: (&'a str, Value)) -> Self {
        self.expect = Some(tup);
        self
    }

    pub(crate) fn handle_rows(&self, rows: &[MySqlRow], ctx: &mut Context) -> crate::Result<ReportItem> {
        if let Some(ref _caps) = self.capture {
            for (idx, r) in rows.iter().enumerate() {
                for _cap in _caps {
                    if let Capture::Column(ref _c) = _cap {
                        let mut _save_key = _c.save_key.to_owned();
                        if rows.len() > 1 {
                            _save_key = _save_key + "|" + &idx.to_string();
                            ctx.store.set(_c.save_key.to_owned(), Value::U32(idx as u32));
                        }
                        if _c.is_usize() {
                            if let Ok(vv) = r.try_get::<i64, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::I64(vv));
                            }
                        } else if _c.is_i32() {
                            if let Ok(vv) = r.try_get::<i32, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::I32(vv));
                            }
                        } else if _c.is_u32() {
                            if let Ok(vv) = r.try_get::<u32, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::U32(vv));
                            }
                        } else if _c.is_i64() {
                            if let Ok(vv) = r.try_get::<i64, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::I64(vv));
                            }
                        } else if _c.is_u64() {
                            if let Ok(vv) = r.try_get::<u64, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::U64(vv));
                            }
                        } else if _c.is_bool() {
                            if let Ok(vv) = r.try_get::<bool, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::Bool(vv));
                            }
                        } else if _c.is_str() {
                            if let Ok(vv) = r.try_get::<&str, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::Str(vv.to_string()));
                            }
                        } else if _c.is_date() {
                            if let Ok(vv) = r.try_get::<time::Date, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::Date(vv));
                            }
                        } else if _c.is_time() {
                            if let Ok(vv) = r.try_get::<time::Time, &str>(_c.key) {
                                ctx.store.set(_save_key, Value::Time(vv));
                            }
                        } else {
                            return Err(crate::error::unimplement("unsupport type"));
                        }
                    }
                }
            }
        }

        let task_id = self.task_id.to_owned();
        let status_code: i32 = 0;
        if let Some(_expect) = &self.expect {
            if let Some(_value) = ctx.store.get(_expect.0.to_string()) {
                if _value == &_expect.1 {
                    return Ok(ReportItem::success(&task_id, status_code, format!("{} expect {:?} pass", _expect.0, _expect.1)))
                } else {
                    return Ok(ReportItem::failed(&task_id, status_code, format!("{} expect {:?} but {:?}", _expect.0, _expect.1, _value)))
                }
            } else {
                return Ok(ReportItem::failed(&task_id, status_code, format!("{} expect {:?} but not found", _expect.0, _expect.1)))
            }
        }

        Ok(ReportItem::success(&task_id, status_code, "database execute succeed".to_string()))
    }

}

#[derive(Debug, Default)]
struct MysqlClient {
    pool: Option<Pool<MySql>>
}

impl MysqlClient {

    async fn create(&mut self, options: &DBClientOption<'_>) -> crate::Result<()> {
        let mut opts = match options.url.parse::<mysql::MySqlConnectOptions>() {
            Ok(o) => o,
            Err(e) => return Err(crate::error::create_client(e.into()))
        };
        opts.disable_statement_logging();
        let pool = match MySqlPoolOptions::new()
            .connect_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(20))
            .max_connections(options.max_connections)
            .connect_with(opts).await {
                Ok(p) => p,
                Err(e) => return Err(crate::error::create_client(e.into()))
            };

        self.pool = Some(pool);
        
        Ok(())
    }

    async fn execute(&self, t: &DBTask<'_>, ctx: &mut Context) -> crate::Result<ReportItem> {
        let pool = &self.pool.as_ref().unwrap();

        let mut sql = t.sql.to_owned();

        if let Some(_params) = &t.params {
            for _k in _params {
                if let Some(v) = ctx.store.get(_k.to_string()) {
                    sql = sql.replace(format!("#{}#", _k).as_str(), v.as_str().as_str());
                }
            }
        }
        let rows = match sqlx::query(&sql).fetch_all(*pool).await {
            Ok(r) => r,
            Err(e) => return Err(crate::error::request(e.into()))
        };
        t.handle_rows(&rows, ctx)
    }

}
