import { h, render, Component } from 'https://unpkg.com/preact@10.15.1/dist/preact.module.js?module';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);
const renderMarkdown = window.downa.render;
const postId = new URLSearchParams(document.location.search).get('post_id')

const formatDate = (date) => {
    const d = new Date(date);
    return `${d.toLocaleDateString()} ${d.toLocaleTimeString()}`;
}

const loadComments = async () => {
    const res = await fetch(`/commentary/comments?postId=${postId}`, { headers: { 'Content-Type': 'application/json' } });
    return await res.json();
};

const publishResize = () => {
    window.top.postMessage({ type: 'resize', height: document.body.scrollHeight }, '*');
};

const createComment = async (comment) => {
    const res = await fetch('/commentary/comment', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ...comment, postId })
    });
    return await res.json();
};

const Comment = (props) => {
    const { author, body, created_at } = props.comment;

    return html`
        <div class="card mb-3">
            <div class="card-header">
                by <i>${author || 'Anonymous'}</i> (${formatDate(created_at)})
            </div>

            <div class="card-body" dangerouslySetInnerHTML="${{ __html: renderMarkdown(body) }}"></div>
        </div>`;
};

class Form extends Component {
    constructor(props) {
        super(props);
        this.state = { author: '', body: '' };
    }

    setAuthor = (e) => this.setState({ author: e.target.value });
    setBody = (e) => this.setState({ body: e.target.value });

    submit = (e) => {
        if (this.state.body.trim() === '') return;
        this.props.addComment(this.state);
        this.setState({ author: '', body: '' });
    }

    componentDidUpdate() {
        publishResize();
    }

    render() {
        return html`
            <div class="card bg-info-subtle mb-3">
                <div class="card-header">Leave a comment (anonymously if you prefer)</div>

                <div class="card-body">
                    <div class="mb-3">
                        <input
                            type="text"
                            class="form-control"
                            value=${this.state.author}
                            onChange=${this.setAuthor}
                            placeholder="Name (optional)"
                        />
                    </div>

                    <div class="mb-3">
                        <textarea
                            class="form-control"
                            value=${this.state.body}
                            onChange=${this.setBody}
                            placeholder="Comment (markdown is supported)"
                        />
                    </div>

                    <button class="btn btn-success" onClick=${this.submit}>Confirm identity</button>
                </div>
            </div>`;
    }
};

class App extends Component {
    constructor(props) {
        super(props);
        this.state = { comments: [] };
    }

    componentDidMount() {
        loadComments().then(comments => this.setState({ comments }));
    }

    addComment = (comment) => {
        createComment(comment).then(comment => this.setState({ comments: [comment, ...this.state.comments] }));
    }

    render() {
        return html`<div class="container-fluid">
            <${Form} addComment=${this.addComment} />

            ${this.state.comments.map(comment => html`<${Comment} comment=${comment} />`)}
        </div>`;
    }
}

render(html`<${App} />`, document.body);
publishResize();
