// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

// This is a specialised implementation of a System module loader.

// @ts-nocheck
/* eslint-disable */
let System, __instantiateAsync, __instantiate;

(() => {
  const r = new Map();

  System = {
    register(id, d, f) {
      r.set(id, { d, f, exp: {} });
    },
  };

  async function dI(mid, src) {
    let id = mid.replace(/\.\w+$/i, "");
    if (id.includes("./")) {
      const [o, ...ia] = id.split("/").reverse(),
        [, ...sa] = src.split("/").reverse(),
        oa = [o];
      let s = 0,
        i;
      while ((i = ia.shift())) {
        if (i === "..") s++;
        else if (i === ".") break;
        else oa.push(i);
      }
      if (s < sa.length) oa.push(...sa.slice(s));
      id = oa.reverse().join("/");
    }
    return r.has(id) ? gExpA(id) : import(mid);
  }

  function gC(id, main) {
    return {
      id,
      import: (m) => dI(m, id),
      meta: { url: id, main },
    };
  }

  function gE(exp) {
    return (id, v) => {
      v = typeof id === "string" ? { [id]: v } : id;
      for (const [id, value] of Object.entries(v)) {
        Object.defineProperty(exp, id, {
          value,
          writable: true,
          enumerable: true,
        });
      }
    };
  }

  function rF(main) {
    for (const [id, m] of r.entries()) {
      const { f, exp } = m;
      const { execute: e, setters: s } = f(gE(exp), gC(id, id === main));
      delete m.f;
      m.e = e;
      m.s = s;
    }
  }

  async function gExpA(id) {
    if (!r.has(id)) return;
    const m = r.get(id);
    if (m.s) {
      const { d, e, s } = m;
      delete m.s;
      delete m.e;
      for (let i = 0; i < s.length; i++) s[i](await gExpA(d[i]));
      const r = e();
      if (r) await r;
    }
    return m.exp;
  }

  function gExp(id) {
    if (!r.has(id)) return;
    const m = r.get(id);
    if (m.s) {
      const { d, e, s } = m;
      delete m.s;
      delete m.e;
      for (let i = 0; i < s.length; i++) s[i](gExp(d[i]));
      e();
    }
    return m.exp;
  }

  __instantiateAsync = async (m) => {
    System = __instantiateAsync = __instantiate = undefined;
    rF(m);
    return gExpA(m);
  };

  __instantiate = (m) => {
    System = __instantiateAsync = __instantiate = undefined;
    rF(m);
    return gExp(m);
  };
})();

System.register("components/Foo", [], function (exports_1, context_1) {
    "use strict";
    var Foo;
    var __moduleName = context_1 && context_1.id;
    return {
        setters: [],
        execute: function () {
            Foo = class Foo extends HTMLElement {
                constructor() {
                    super();
                    this._name = "World";
                    this.shadowRoot.attachShadow({ mode: "open" });
                    this.render();
                }
                static get observedAttributes() {
                    return ["name"];
                }
                set name(value) {
                    this._name = value;
                }
                get name() {
                    return this._name;
                }
                render() {
                    const t = document.createElement("template");
                    t.innerHTML = `
      <style>
        :host {
          color: green;
        }
      </style>
      <h1>Hello ${this._name}</h1>
    `;
                    this.shadowRoot.appendChild(t.content.cloneNode(true));
                }
                attributeChangedCallback(name, oldValue, newValue) {
                    this._name = newValue;
                    this.render();
                }
            };
            exports_1("Foo", Foo);
        }
    };
});
System.register("app", ["components/Foo"], function (exports_2, context_2) {
    "use strict";
    var Foo_ts_1;
    var __moduleName = context_2 && context_2.id;
    function app() {
        window.customElements.define("foo", Foo_ts_1.Foo);
    }
    exports_2("app", app);
    return {
        setters: [
            function (Foo_ts_1_1) {
                Foo_ts_1 = Foo_ts_1_1;
            }
        ],
        execute: function () {
            app(); // initialize the app
        }
    };
});

const __exp = __instantiate("app");
export const app = __exp["app"];
