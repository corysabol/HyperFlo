export class Foo extends HTMLElement {
  static get observedAttributes() {
    return ["name"];
  }

  _name: string;

  constructor() {
    super();

    this._name = "World";

    this.attachShadow({ mode: "open" }); 
    this.render();
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
          color: red;
        }
      </style>
      <h1>Hola ${this._name}</h1>
    `;
    this.shadowRoot.appendChild(t.content.cloneNode(true));
  }

  attributeChangedCallback(name, oldValue, newValue) {
    this._name = newValue;
    this.render();
  }
}
