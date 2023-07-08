import { h, render, Component } from 'https://unpkg.com/preact@10.15.1/dist/preact.module.js?module';
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

class App extends Component {
    constructor(props) {
        super(props);
        this.state = { comments: [] };
    }

    componentDidMount() {
        fetch('/commentary/comments', { headers: { 'Content-Type': 'application/json' } })
            .then(res => res.json())
            .then(comments => this.setState({ comments }));
    }

    render() {
        return html`<div class="container-fluid">
            ${this.state.comments.map(comment => html`<${Comment} comment=${comment} />`)}
        </h1>`;
    }
}

render(html`<${App} />`, document.body);
