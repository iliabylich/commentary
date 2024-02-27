const template = document.createElement('template');
template.innerHTML = `
    <header>Leave a comment (anonymously if you prefer)</header>

    <fieldset>
        <input type="hidden" name="postId" />
        <input name="author" type="text" placeholder="Name (optional)" />
        <textarea name="body" placeholder="Comment (markdown is supported)" required></textarea>
        <input type="submit" value="Add comment" />
    </fieldset>
`;

class CommentaryForm extends HTMLElement {
    constructor() {
        super();

        const node = template.content.cloneNode(true);
        node.querySelector('input[name=postId]').value = this.querySelector('#post-id').textContent;

        this.onChangeFn = window[this.dataset.onchange];
        this.url = this.dataset.url;
        this.form = document.createElement('form');
        this.form.append(...node.children);

        this.innerHTML = '';
        this.attachShadow({ mode: 'open' });
        this.shadowRoot.appendChild(this.form);

        const style = document.createElement('style');
        style.textContent = `{% include "_commentary-form.css" %}`;
        this.shadowRoot.appendChild(style.cloneNode(true));
    }

    connectedCallback() {
        this.form.addEventListener('submit', (e) => {
            e.preventDefault();
            this.sendData();
            this.form.reset();
        });

        new ResizeObserver(this.onChangeFn).observe(this.form.querySelector('textarea'))
    }

    async sendData() {
        const payload = Object.fromEntries(new FormData(this.form).entries());

        const response = await fetch('/commentary/leave-comment', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        const html = await response.text();
        const node = htmlToNode(html);
        document.querySelector('main').prepend(node);
        this.onChangeFn();
    }
}

customElements.define('commentary-form', CommentaryForm);

function htmlToNode(html) {
    const template = document.createElement('template');
    template.innerHTML = html;
    return template.content.children[0];
}
