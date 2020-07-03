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
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;
    use std::process::Command;
    use std::ffi::OsStr;
    use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};
    use notify::event::*;
    use notify::EventKind::*;
    use notify::event::AccessKind::*;

    extern crate base64;

    fn exec_client_build() {
        let output = Command::new("just")
            .arg("build-client")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            print!("{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            print!("{}", s);
        }
    }

    pub fn client_dev_mode(path: &str, wv: WebView<usize>) -> Result<()> {
        // Automatically select the best implementation for your platform.
        let wv_handle = wv.handle();
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res: Result<Event>| {
            match res {
                Ok(event) => {
                    // match on the event
                    if let Access(Close(mode)) = &event.kind {
                        match mode {
                            Write => {
                                println!("{:?}", event.kind);
                                // unwrap all the things!
                                let mut paths = event.paths;
                                let path_buf = paths.pop().unwrap();
                                let filename_osstr = path_buf.file_name();
                                if filename_osstr == Some(OsStr::new("app.bundle.js")) {
                                    // read in the bundle
                                    let mut file = match File::open(path_buf) {
                                        Err(e) => panic!("couldn't open file: {}", e),
                                        Ok(file) => file,
                                    };
                                    let mut s = String::new();
                                    match file.read_to_string(&mut s) {
                                        Err(e) => panic!("couldn't read file: {}", e),
                                        Ok(_) => (),
                                    };
                                    wv_handle.dispatch(|wv| {
                                        wv.eval(&format!(r#"
                                        var scr = document.querySelector('#app');
                                        scr.innerHTML = ""; // clear the inner contents
                                        scr.src = `data:text/javascript;base64,{}`;
                                        var e = document.querySelector('html');
                                        window.location = `data:text/html,${{encodeURIComponent(e.innerHTML)}}`;
                                        "#, base64::encode(s)))
                                    });
                                } else {
                                    exec_client_build() 
                                }
                            },
                            _ => exec_client_build(),
                        }
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        })?;


        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive);
        wv.run().unwrap();
        loop {};
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
        dev::client_dev_mode("./src/client", webview)?; 
    } else {
        webview.run().unwrap();
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
