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
        <div class="relative flex flex-col min-w-0 rounded break-words border bg-white border-1 border-gray-300 mb-3">
            <div class="py-3 px-6 mb-0 bg-gray-200 border-b-1 border-gray-300 text-gray-900">
                by <i>${author || 'Anonymous'}</i> (${formatDate(created_at)})
            </div>

            <div class="flex-auto p-6" dangerouslySetInnerHTML="${{ __html: renderMarkdown(body) }}"></div>
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
            <div class="relative flex flex-col min-w-0 rounded break-words border bg-gray-200 border-1 border-gray-300 mb-3">
                <div class="px-4 py-3 mb-0 bg-gray-200 border-b-1 border-gray-400 text-gray-900">Leave a comment (anonymously if you prefer)</div>

                <div class="flex-auto px-4 py-6">
                    <div class="mb-3">
                        <input
                            type="text"
                            class="block appearance-none w-full py-1 px-2 mb-1 text-base leading-normal bg-white text-gray-800 border border-gray-200 rounded"
                            value=${this.state.author}
                            onChange=${this.setAuthor}
                            placeholder="Name (optional)"
                        />
                    </div>

                    <div class="mb-3">
                        <textarea
                            class="block appearance-none w-full py-1 px-2 mb-1 text-base leading-normal bg-white text-gray-800 border border-gray-200 rounded"
                            value=${this.state.body}
                            onChange=${this.setBody}
                            placeholder="Comment (markdown is supported)"
                        />
                    </div>

                    <button class="inline-block align-middle text-center select-none border font-normal whitespace-no-wrap rounded py-1 px-3 leading-normal no-underline bg-green-500 text-white hover:green-600" onClick=${this.submit}>Add comment</button>
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
        return html`<div class="container mx-auto sm:px-4 max-w-full">
            <${Form} addComment=${this.addComment} />

            ${this.state.comments.map(comment => html`<${Comment} comment=${comment} />`)}
        </div>`;
    }
}

const root = document.getElementById("root");
render(html`<${App} />`, root);
publishResize();

addEventListener("resize", publishResize)
new ResizeObserver(publishResize).observe(document.querySelector("textarea"))
