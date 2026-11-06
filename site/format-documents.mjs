// The formatting boundary: Markdown and code highlighting. What a page says, and where, stays in src/site.incn.
//
//   node format-documents.mjs <checkout of index> <destination>
//
// Writes one fragment per document and format (<hex>.<format>.html) and per adopted manifest
// (manifests/<scope>/<name>/<version>.html). Nothing here runs or interprets a package's content, and every document's
// bytes are checked against the digest the index names before they are read.
import {readFile, writeFile, mkdir} from 'node:fs/promises';
import {dirname, join, resolve} from 'node:path';
import {createHash} from 'node:crypto';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js/lib/core';
import c from 'highlight.js/lib/languages/c';
import cpp from 'highlight.js/lib/languages/cpp';
import ini from 'highlight.js/lib/languages/ini';
import python from 'highlight.js/lib/languages/python';
import rust from 'highlight.js/lib/languages/rust';

const tokens = JSON.parse(await readFile(new URL('./incan-tokens.json', import.meta.url), 'utf8'));
hljs.registerLanguage('rust', rust);
hljs.registerLanguage('toml', ini);
hljs.registerLanguage('c', c);
hljs.registerLanguage('cpp', cpp);
hljs.registerLanguage('incan', language => ({
  ...python(language),
  name: 'Incan',
  keywords: {keyword: tokens.keywords, type: tokens.types, built_in: tokens.functions},
}));
const LANGUAGE_ALIASES = {rs: 'rust', incn: 'incan'};

const escape = text => text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', '&quot;');
const decode = text => text.replaceAll('&lt;', '<').replaceAll('&gt;', '>').replaceAll('&quot;', '"').replaceAll('&#39;', "'").replaceAll('&amp;', '&');

// A fence's language is the first word of its info string: rustdoc writes `rust,no_run`. Anything else stays plain.
export function highlightBlock(encoded, label = '') {
  const first = label.split(',')[0].toLowerCase();
  const language = LANGUAGE_ALIASES[first] || first;
  if (!hljs.getLanguage(language)) return `<pre><code>${encoded}</code></pre>`;
  const {value} = hljs.highlight(decode(encoded), {language, ignoreIllegals: true});
  return `<pre><code class="syntax-highlight language-${language}">${value}</code></pre>`;
}

const highlightBlocks = html => html.replace(
  /<pre><code(?: class="language-([^"]*)")?>([\s\S]*?)<\/code><\/pre>/g,
  (_, label, encoded) => highlightBlock(encoded, label),
);

// Author HTML stays inert: Markdown's own `html: false` escapes it, prose comments are dropped, and a remote image
// becomes its alternate text so reading a page never requests anything from elsewhere. Headings sit below the page's.
const markdown = new MarkdownIt({html: false, linkify: false});
export function renderMarkdown(source) {
  const html = markdown.render(source)
    .split(/(<code\b[^>]*>[\s\S]*?<\/code>)/g)
    .map((part, index) => index % 2 ? part : part.replace(/&lt;!--[\s\S]*?--&gt;/g, '').replace(/<img\b[^>]*alt="([^"]*)"[^>]*>/g, '$1'))
    .join('')
    .replace(/<(\/?)h([1-6])\b/g, (_, close, level) => `<${close}h${Math.min(6, Number(level) + 2)}`);
  return highlightBlocks(html);
}

export const renderDocument = (source, format) => format === 'markdown' ? renderMarkdown(source) : `<pre><code>${escape(source)}</code></pre>`;

async function readDocument(root, document) {
  const {digest, content, format, path} = document;
  if (!/^sha256:[0-9a-f]{64}$/.test(digest) || content !== `content/sha256/${digest.slice(7)}`) throw Error(`Invalid document identity: ${path}`);
  if (!['text', 'markdown', 'restructuredtext'].includes(format)) throw Error(`Unsupported document format: ${format}`);
  const bytes = await readFile(join(root, content));
  if (bytes.length !== document.bytes || `sha256:${createHash('sha256').update(bytes).digest('hex')}` !== digest) throw Error(`Document integrity failure: ${path}`);
  return new TextDecoder('utf-8', {fatal: true}).decode(bytes);
}

async function write(path, text) {
  await mkdir(dirname(path), {recursive: true});
  await writeFile(path, text);
}

async function formatIndex(root, destination) {
  const catalog = JSON.parse(await readFile(join(root, 'catalog/index.json'), 'utf8'));
  const done = new Set();
  let manifests = 0;
  for (const summary of catalog.packages) {
    const page = JSON.parse(await readFile(join(root, summary.page), 'utf8'));
    for (const version of page.versions) {
      for (const document of version.documents || []) {
        const name = `${document.digest.slice(7)}.${document.format}.html`;
        if (done.has(name)) continue;
        done.add(name);
        await write(join(destination, name), renderDocument(await readDocument(root, document), document.format));
      }
      if (version.manifest.split('/').some(part => part === '.' || part === '..')) throw Error(`Unsafe manifest path: ${version.manifest}`);
      const manifest = await readFile(join(root, version.manifest), 'utf8');
      await write(join(destination, 'manifests', page.name, `${version.version}.html`), highlightBlock(escape(manifest), 'toml'));
      manifests += 1;
    }
  }
  console.log(`Verified and formatted ${done.size} documents and ${manifests} manifests`);
}

if (process.argv[1] && resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)) {
  const [root, destination] = process.argv.slice(2);
  if (!root || !destination) throw Error('usage: node format-documents.mjs <checkout of index> <destination>');
  await formatIndex(resolve(root), resolve(destination));
}
