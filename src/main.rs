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
use std::thread;
use web_view::*;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

mod dev {
    use web_view::*;
    use std::sync::Arc;
    use std::thread;
    use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};
    pub fn client_dev_mode(path: &str, wv: WebView<usize>) -> Result<()> {
        // Automatically select the best implementation for your platform.
        let wv_handle = wv.handle();
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
            match res {
                Ok(event) => {
                    println!("event: {:?}", event);
                    //wv.eval(&format!("alert('this was triggered by a fs event!')"));
                    wv_handle.dispatch(move |wv| {
                        wv.eval(&format!("console.log('this was triggered by a fs event: {:?}')", event))
                    });
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        })?;


        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive);
        wv.run().unwrap();
        loop {};

        Ok(())
    }
}

fn main() -> Result<()> {
    // if dev mode watch the client dir and rebuild the client app on change
    // create a web view
    let html_content = format!(
        include_str!("./client/index.html"),
        css = include_str!("./client/global.css"),
        app = include_str!("./client/app.bundle.js")
    );

    let webview = web_view::builder()
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
        .build()
        .unwrap();
    let dev = true;
    if dev {
        // this will block though :(
        dev::client_dev_mode("./src/client", webview)?; 
    } else {
        webview
            .run()
            .unwrap();
    }

    Ok(())
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
