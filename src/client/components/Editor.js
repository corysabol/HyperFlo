import CodeMirror from '../lib/codemirror-5.54.0/src/codemirror.js';

export class Editor extends HTMLElement {
  static get observedAttributes() {
    return ["foo"]; 
  }

  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.render();
  }

  render() {
    const t = document.createElement("template");
    t.innerHTML = `
    `;
    this.shadowRoot.appendChild(t.content.cloneNode(true));
  }

}
