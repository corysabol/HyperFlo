import { Foo } from "./components/Foo.ts";

export function bar() {
  console.log("Exported funciton bar()");
}

function app() {
  window.customElements.define("foo-component", Foo); 
}

app(); // initialize the app
