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

  render(): void {
    const t: HTMLTemplateElement = document.createElement("template");
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

  attributeChangedCallback(name, oldValue: string, newValue: string) {
    this._name = newValue;
    this.render();
  }
}
