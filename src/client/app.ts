import { Foo } from "./components/Foo.ts";

export function app() {
  window.customElements.define("foo", Foo); 
}

app(); // initialize the app
