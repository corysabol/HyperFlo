#[macro_use]
extern crate derive_deref;

use deno_core::CoreIsolate;
//use deno_core::Op;
use deno_core::ResourceTable;
use deno_core::Script;
use deno_core::StartupData;
use deno_core::ZeroCopyBuf;
use futures::future::poll_fn;
use futures::prelude::*;
use futures::task::Context;
use futures::task::Poll;
use std::io::{self, Write};
use std::cell::RefCell;
use std::rc::Rc;
use std::pin::Pin;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

struct Isolate {
    core_isolate: Box<CoreIsolate>,
    state: State
}

#[derive(Clone, Default, Deref)]
struct State(Rc<RefCell<StateInner>>);

#[derive(Default)]
struct StateInner {
    resource_table: ResourceTable,
}

impl Isolate {
    pub fn new() -> Self {
        let startup_data = StartupData::Script(Script {
            source: include_str!("blah.js"),
            filename: "blah.js",
        });

        let mut isolate = Self {
            core_isolate: CoreIsolate::new(startup_data, false),
            state: Default::default(),
        };

        /*
        isolate.register_sync_op("listen", op_listen);
        isolate.register_op("accept", op_accept);
        isolate.register_op("read", op_read);
        isolate.register_op("write", op_write);
        isolate.register_op("close", op_close);
        */

        isolate
    }
}

impl Future for Isolate {
    type Output = <CoreIsolate as Future>::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.core_isolate.poll_unpin(cx)
    }
}

fn main() {
    io::stdout().flush().unwrap();
    let mut isolate = deno_core::CoreIsolate::new(
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
    ));
}
