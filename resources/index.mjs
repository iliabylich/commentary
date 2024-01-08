const postId = new URLSearchParams(document.location.search).get('post_id');
const commentTemplate = document.getElementById('commentary-comment-template').content;

const publishResize = () => {
    window.top.postMessage({ type: 'resize', height: document.body.scrollHeight }, '*');
};

/**
 * @typedef Comment
 * @prop {string} author
 * @prop {string} created_at
 * @prop {string} body
 */

customElements.define('commentary-form', class extends HTMLFormElement {
    connectedCallback() {
        this.addEventListener('submit', (e) => {
            e.preventDefault();
            this.sendData();
            this.reset();
        });

        new ResizeObserver(publishResize).observe(this.querySelector('textarea'))
    }

    async sendData() {
        const payload = { postId, ...Object.fromEntries(new FormData(this).entries()) };

        const response = await fetch('/commentary/leave-comment', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        const newComment = await response.json();
        document.querySelector('commentary-comments').addNewComment(newComment);
    }
}, { extends: 'form' });

customElements.define('commentary-comments', class extends HTMLElement {
    connectedCallback() {
        this.initialRender();
    }

    addNewComment(comment) {
        const commentElement = document.createElement('commentary-comment');
        commentElement.comment = comment;
        this.prepend(commentElement);
        publishResize();
    }

    async initialRender() {
        const response = await fetch(
            `/commentary/comments.json?postId=${postId}`,
            { headers: { 'Content-Type': 'application/json' } }
        );
        const comments = await response.json();
        for (const comment of comments) {
            const commentElement = document.createElement('commentary-comment');
            commentElement.comment = comment;
            this.appendChild(commentElement);
        }
        publishResize();
    }
});

customElements.define('commentary-comment', class extends HTMLElement {
    /**
     * @param {Comment} comment
     */
    set comment(comment) {
        const node = commentTemplate.cloneNode(true);
        node.querySelector('header').textContent = this.renderHeader(comment);
        node.querySelector('p').innerHTML = this.renderMarkdown(comment);
        this.appendChild(node);
    }

    /**
     * @param {Comment} comment
     */
    renderHeader(comment) {
        return `
            by ${comment.author || 'Anonymous'}
            (${new Date(comment.created_at).toLocaleString()})
        `;
    }

    /**
     * @param {Comment} comment
     */
    renderMarkdown(comment) {
        return window.downa.render(comment.body);
    }
});
