import { h, render } from 'https://unpkg.com/preact@10.15.1/dist/preact.module.js?module';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);
const renderMarkdown = window.downa.render;

const formatDate = (date) => {
    const d = new Date(date);
    return `${d.toLocaleDateString()} ${d.toLocaleTimeString()}`;
}

function Comment(props) {
    const { author, body, created_at } = props.comment;

    return html`<div class="card mb-2">
        <div class="card-header">
            ${author}
            (${formatDate(created_at)})
        </div>

        <div class="card-body" dangerouslySetInnerHTML="${{ __html: renderMarkdown(body) }}"></div>
    </div>`;
}

function App(props) {
    return html`<div class="container-fluid">
        ${props.comments.map(comment => html`<${Comment} comment=${comment} />`)}
    </h1>`;
}

fetch('/commentary/comments', { headers: { 'Content-Type': 'application/json' } })
    .then(res => res.json())
    .then(comments => {
        render(html`<${App} comments=${comments} />`, document.body);
    });
