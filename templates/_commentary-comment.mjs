const template = document.createElement('template');
template.innerHTML = `
    <article>
      <header>
        by <author></author> (<time></time>)
      </header>

      <p>
      </p>
    </article>
`;

class CommentaryComment extends HTMLElement {
    constructor() {
        super();

        const node = template.content.cloneNode(true);
        node.querySelector('author').textContent = this.querySelector('#author').textContent;
        node.querySelector('time').textContent = this.querySelector('#created_at').textContent;
        node.querySelector('p').textContent = this.querySelector('#body').textContent;

        const style = document.createElement('style');
        style.textContent = `{% include "_commentary-comment.css" %}`;

        this.innerHTML = '';
        this.attachShadow({ mode: 'open' });

        this.shadowRoot.appendChild(node);
        this.shadowRoot.appendChild(style.cloneNode(true));
    }
}

customElements.define('commentary-comment', CommentaryComment);
