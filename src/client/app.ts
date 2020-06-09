import { Foo } from "./components/Foo.ts";

function app() {
  window.customElements.define("foo-component", Foo); 
}

app(); // initialize the app
