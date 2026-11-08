// The site's browser model and formatting boundary, and with SITE_OUT set a rendered site.
import {test} from 'node:test';
import assert from 'node:assert/strict';
import {existsSync, readdirSync, readFileSync} from 'node:fs';
import {join} from 'node:path';
import {licenseIdentifiers, matchingPackages, pullFromOven, selectPackages} from '../assets/catalog-model.js';
import {highlightBlock, renderDocument, renderMarkdown} from '../format-documents.mjs';

const item = (name, extra = {}) => ({name: `crates-io/${name}`, description: '', license: 'MIT', versions: 1, 'build-targets': [], ...extra});
const params = text => new URLSearchParams(text);

test('a license expression names its identifiers, without its exceptions', () => {
  assert.deepEqual(licenseIdentifiers('(Apache-2.0 WITH LLVM-exception OR MIT) AND BSD-3-Clause'), ['Apache-2.0', 'MIT', 'BSD-3-Clause']);
  assert.deepEqual(licenseIdentifiers(''), []);
});

test('a search ranks the exact name first and folds dashes and underscores', () => {
  const packages = [item('serde-json'), item('aserde'), item('serde'), item('other', {description: 'for serde users'})];
  assert.deepEqual(matchingPackages(packages, params('q=serde')).map(p => p.name.split('/')[1]), ['serde', 'serde-json', 'aserde', 'other']);
  assert.deepEqual(matchingPackages(packages, params('q=serde_json')).map(p => p.name.split('/')[1]), ['serde-json']);
});

test('filters narrow by license, category and indexed platform', () => {
  const packages = [
    item('a', {license: 'MIT OR Apache-2.0', categories: ['web'], 'build-targets': ['aarch64-apple-darwin']}),
    item('b', {license: 'Apache-2.0'}),
  ];
  assert.equal(matchingPackages(packages, params('license=id:MIT')).length, 1);
  assert.equal(matchingPackages(packages, params('license=expression:Apache-2.0'))[0].name, 'crates-io/b');
  assert.equal(matchingPackages(packages, params('category=web'))[0].name, 'crates-io/a');
  assert.equal(matchingPackages(packages, params('platform=apple'))[0].name, 'crates-io/a');
  assert.equal(matchingPackages(packages, params('platform=none'))[0].name, 'crates-io/b');
});

test('a page out of range lands on the nearest page', () => {
  const packages = Array.from({length: 120}, (_, index) => item(`p${String(index).padStart(3, '0')}`));
  const last = selectPackages(packages, params('page=99'));
  assert.deepEqual([last.page, last.pages, last.items.length, last.total], [3, 3, 20, 120]);
  assert.equal(selectPackages(packages, params('page=nonsense')).page, 1);
});

test('the oven draws from every match, not one page, and from nothing when none match', () => {
  const packages = Array.from({length: 120}, (_, index) => item(`p${String(index).padStart(3, '0')}`));
  assert.equal(pullFromOven(packages, params(''), 0).name, 'crates-io/p000');
  assert.equal(pullFromOven(packages, params(''), 0.999).name, 'crates-io/p119');
  assert.equal(pullFromOven(packages, params('q=p05'), 0).name, 'crates-io/p050');
  assert.equal(pullFromOven(packages, params('q=absent'), 0), null);
});

test('author HTML stays inert and a remote image becomes its alternate text', () => {
  const html = renderMarkdown('<script>alert(1)</script>\n\n![badge](https://example.test/b.png)\n\n<!-- a hidden note -->\n\n[x](javascript:alert(1))\n');
  assert.ok(!html.includes('<script>'));
  assert.ok(!html.includes('<img'));
  assert.ok(html.includes('badge'));
  assert.ok(!html.includes('hidden note'));
  assert.ok(!html.includes('href="javascript:'));
});

test('headings sit below the page and a comment in code stays', () => {
  const html = renderMarkdown('# Title\n\n`<!-- kept -->`\n');
  assert.ok(html.includes('<h3>Title</h3>'));
  assert.ok(html.includes('&lt;!-- kept --&gt;'));
});

test('known fences are highlighted at build time and others stay plain', () => {
  assert.ok(renderMarkdown('```rust,no_run\nfn main() {}\n```\n').includes('class="syntax-highlight language-rust"'));
  assert.ok(renderMarkdown('```rs\nfn main() {}\n```\n').includes('language-rust'));
  assert.ok(renderMarkdown('```toml\n[a]\nb = 1\n```\n').includes('hljs-section'));
  assert.ok(renderMarkdown('```incan\ndef f() -> None:\n    pass\n```\n').includes('<span class="hljs-keyword">def</span>'));
  assert.equal(highlightBlock('a &lt; b', 'made-up'), '<pre><code>a &lt; b</code></pre>');
});

test('a plain document is escaped as literal text', () => {
  assert.equal(renderDocument('<b>&', 'restructuredtext'), '<pre><code>&lt;b&gt;&amp;</code></pre>');
});

const files = directory => readdirSync(directory, {recursive: true, withFileTypes: true})
  .filter(entry => entry.isFile() && /\.(html|css|js)$/.test(entry.name))
  .map(entry => join(entry.parentPath, entry.name));

test('the design carries no third-party request and no preview text', () => {
  const root = new URL('..', import.meta.url).pathname;
  for (const file of [...files(join(root, 'templates')), ...files(join(root, 'assets')), ...files(join(root, 'arcade'))]) {
    const text = readFileSync(file, 'utf8');
    for (const banned of ['googleapis', 'gstatic', 'playground', 'serde-assets', 'Incan.pub', 'ACME', 'acme-assets']) {
      assert.ok(!text.includes(banned), `${file} says ${banned}`);
    }
  }
});

test('a rendered site has a page for every package and version', {skip: !process.env.SITE_OUT}, () => {
  const out = process.env.SITE_OUT;
  const catalog = JSON.parse(readFileSync(join(out, 'catalog/index.json'), 'utf8'));
  assert.equal(JSON.parse(readFileSync(join(out, 'assets/search.json'), 'utf8')).length, catalog.packages.length);
  for (const summary of catalog.packages) {
    assert.ok(existsSync(join(out, 'packages', summary.name, 'index.html')), summary.name);
    const page = JSON.parse(readFileSync(join(out, summary.page), 'utf8'));
    for (const version of page.versions) {
      assert.ok(existsSync(join(out, 'packages', summary.name, 'versions', version.version, 'index.html')), `${summary.name} ${version.version}`);
    }
  }
  for (const relative of ['index.html', 'catalog/index.html', '404.html', 'arcade/loaf-run/index.html', 'assets/site.css', 'catalog/policy.json']) {
    assert.ok(existsSync(join(out, relative)), relative);
  }
});
