import { Foo } from "./components/Foo.js";
import { TextEditor } from "./components/Editor.js";

function app() {
  window.customElements.define("foo-component", Foo); 
}

app(); // initialize the app
