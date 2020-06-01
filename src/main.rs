//use deno_core::CoreIsolate;
//use deno_core::Op;
//use deno_core::ResourceTable;
//use deno_core::Script;
//use deno_core::StartupData;
//use deno_core::ZeroCopyBuf;
//use futures::future::poll_fn;
//use futures::prelude::*;
//use futures::task::Context;
//use futures::task::Poll;
//use std::io::{self, Write};
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::pin::Pin;
//use tokio::io::AsyncRead;
//use tokio::io::AsyncWrite;
//use tokio::net::TcpListener;
//use tokio::net::TcpStream;
use web_view::*;

fn main() {
    // create a web view
    let html_content = format!(
        include_str!("./client/index.html"),
        css = include_str!("./client/global.css"),
        app = include_str!("./client/app.bundle.js")
    );

    web_view::builder()
        .title("HyperFlo - HTTP dataflow programming tool.")
        .content(Content::Html(html_content))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match arg {
                "foo" => {
                    println!("Wow so cool!");
                    Ok(())
                }
                "bar" => {
                    // pass data back into the webview context
                    webview.eval(&format!("setData({})", 42))
                }
                _ => unimplemented!(),
            }
        })
        .run()
        .unwrap();
    /*let mut isolate = deno_core::CoreIsolate::new(
        deno_core::StartupData::Script(Script{
            source: "",
            filename: "",
        }),
        false,
    );
    deno_core::js_check(isolate.execute("<anon>", r#"
        Deno.core.print('\n\n');
        for (let prop in Deno.core) {
            Deno.core.print(`${prop}\n`);
        }
        Deno.core.print('\n\n');
        "#
    ));*/
}
