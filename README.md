# Anole
Anole is an Interface automation testing client in Rust.

* Sequence HTTP, MySQL task. 
* Capture response values and use for tasks. Support for headers, json, xml, database query.
* Expect assert for task.
* Custom reporter.



## Install

```toml
# Cargo.toml
[dependencies]
anole = "0.0.1"
```



## Usage

See the `examples/` folder.

### Capture syntax

* use `.` to split

  To capture JSON value `a` from `{"code": 0, "data": {"a": "a"}]} `, type `data.a`.

* use `|` to read as array

  To capture JSON value `b` from `{"code": 0, "data": ["a", "b", "c"]}`,type `data|1`.

* use `#` to read as attribute

  To capture XML attribute `hover` from `<a hover="true"></a>`,type `a#hover`.

* use `:` to replace with store value

  To use captured value, type`:store_key`.

### HTTP 

```rust
use std::{sync, time::Instant};

use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method, Deserializer}, capture, report::{ReportItem, StdReporter}, value::Value};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");

    let cost = Instant::now();

    let (sender, recv) = sync::mpsc::channel::<ReportItem>();

    let reporter_printer = tokio::spawn(async {
        for r in recv {
            info!("{}", r);
        }
    });

    Engine::new()
        .with_reporter(Box::new(StdReporter::new(sender)))
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tags")
            .method(Method::Get)
            .capture(vec![
              //capture as json,take data[1] and save as tag
                capture::json("data|1", "tag"),
              //capture header["content-length"] and save as cl
                capture::header("content-length", "cl"),
                capture::json("code", "code")
            ])
            .expect(("code", Value::I32(0)))
            .build())
        .with_http(HttpTaskBuilder::new()
          //":tag" take value from capture store which key is tag 
            .url("https://tvapi.dykkan.com/v1/tag/:tag")
            .method(Method::Get)
            .verbose(false)
            .build())
        .run().await.unwrap();

        reporter_printer.await.unwrap();

        info!("execute completed! cost_time:{:?}", cost.elapsed());
    
}
```

### MySQL

```rust

use std::{sync, time::Instant};

use anole::{engine::Engine, task::db::{mysql::{MysqlTask, DBTask}, DBClientOption}, capture, value, report::{ReportItem, self}};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");

    let cost = Instant::now();

    let (sender, recv) = sync::mpsc::channel::<ReportItem>();

    let reporter_printer = tokio::spawn(async {
        for r in recv {
            info!("{}", r);
        }
    });

    Engine::new()
        .with_reporter(Box::new(report::StdReporter::new(sender)))
        .with_mysql(MysqlTask::default()
            .options(DBClientOption::builder().url("mysql://username:password@host/database").max_connections(5))
            .with_task(DBTask::new("SELECT count(*) as count FROM tbl_order")
                .capture(vec![
                    capture::column("count", "order_count", capture::CapValueType::Size)
                ]).expect(("order_count", value::Value::I64(1))))
            .with_task(DBTask::new("SELECT name, create_time FROM tbl_order LIMIT :order_count")
            .param("order_count")
            .capture(vec![
                capture::column("name", "order_name", capture::CapValueType::Str),
                capture::column("create_time", "order_date", capture::CapValueType::Date),
            ]))
        )
        .run().await.unwrap();

    reporter_printer.await.unwrap();
    
    info!("execute completed! cost_time:{:?}", cost.elapsed());
}
```

You can also run with mysql and http task.

## TODO

* support postgres, mssql, redis, mongodb
* support spawn task

## License

This project is licensed under either of

* Apache License, Version 2.0
* MIT License