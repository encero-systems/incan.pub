// Search, filters and Pull from the oven. Without this script the numbered catalog pages still list every item.
import {PAGE_SIZE, licenseIdentifiers, pullFromOven, selectPackages} from './catalog-model.js';

const shell = document.getElementById('directory');
const root = new URL(shell.dataset.root, location.href);
const draw = document.querySelector('.kiln-random');
const form = document.getElementById('filters');

async function loadPackages() {
  const response = await fetch(shell.dataset.searchIndex);
  if (!response.ok) throw Error('Search index unavailable');
  const packages = await response.json();
  if (!Array.isArray(packages) || packages.some(item => !/^[a-z0-9-]+\/[a-z0-9_.+-]+$/.test(item.name))) throw Error('Invalid search index');
  return packages;
}

const packageUrl = item => new URL('packages/' + item.name + '/', root).href;

function node(tag, text, className) {
  const element = document.createElement(tag);
  if (text !== undefined) element.textContent = text;
  if (className) element.className = className;
  return element;
}

// The same row `catalog_row` in src/site.incn writes.
function row(item) {
  const line = node('li');
  line.dataset.kind = 'Loaf';
  const link = node('a');
  link.href = packageUrl(item);
  const title = node('span');
  title.append(node('strong', item.name.split('/')[1]), node('small', 'Loaf', 'kind'), node('small', item.name));
  link.append(title, node('span', item.description, 'result-desc'), node('code', item.latest));
  line.append(link);
  return line;
}

function startCatalog(packages) {
  const query = document.getElementById('package-filter');
  const controls = ['license', 'category', 'platform', 'sort'].map(id => document.getElementById(id));
  const rows = document.getElementById('catalog-rows');
  const pagination = document.getElementById('pagination');
  const canonical = new URL('catalog/', root);

  function readState() {
    const params = new URLSearchParams(location.search);
    if (!params.has('page')) params.set('page', location.pathname.match(/catalog\/page\/(\d+)\//)?.[1] || '1');
    query.value = params.get('q') || '';
    for (const control of controls) {
      control.value = [...control.options].some(option => option.value === params.get(control.id)) ? params.get(control.id) : control.options[0].value;
    }
    return params;
  }

  function state() {
    const params = new URLSearchParams();
    if (query.value.trim()) params.set('q', query.value.trim());
    for (const control of controls) if (control.value && control.value !== 'name') params.set(control.id, control.value);
    return params;
  }

  function destination(params, page) {
    const next = new URLSearchParams(params);
    if (page > 1) next.set('page', page); else next.delete('page');
    return canonical.pathname + (next.size ? '?' + next : '');
  }

  function pageLink(label, page, params) {
    const link = node('a', label);
    link.href = destination(params, page);
    link.addEventListener('click', event => {
      if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
      event.preventDefault();
      const next = new URLSearchParams(params);
      next.set('page', page);
      render(next, true);
    });
    return link;
  }

  function render(params, push = false) {
    const result = selectPackages(packages, params);
    rows.replaceChildren(...result.items.map(row));
    document.getElementById('empty-state').hidden = result.total !== 0;
    const first = (result.page - 1) * PAGE_SIZE + 1, last = Math.min(result.page * PAGE_SIZE, result.total);
    document.getElementById('matches').textContent = result.total ? `${first}–${last} of ${result.total} ${result.total === 1 ? 'item' : 'items'}` : 'No matching items';
    const links = [];
    if (result.page > 1) links.push(pageLink('← Previous', result.page - 1, params));
    links.push(node('span', `Page ${result.page} of ${result.pages}`));
    if (result.page < result.pages) links.push(pageLink('Next →', result.page + 1, params));
    pagination.replaceChildren(...links);
    draw.disabled = result.total === 0;
    const path = destination(params, result.page) + location.hash;
    if (push) history.pushState(null, '', path); else history.replaceState(null, '', path);
    if (push) scrollTo({top: 0});
  }

  document.querySelector('.catalog-filters').hidden = false;
  render(readState());
  let timer;
  query.addEventListener('input', () => { clearTimeout(timer); timer = setTimeout(() => render(state()), 120); });
  for (const control of controls) control.addEventListener('change', () => render(state(), true));
  form.addEventListener('submit', event => { event.preventDefault(); render(state(), true); });
  form.addEventListener('reset', () => {
    clearTimeout(timer);
    query.value = '';
    for (const control of controls) control.selectedIndex = 0;
    render(new URLSearchParams(), true);
  });
  document.querySelector('.global-search').addEventListener('submit', event => {
    event.preventDefault();
    query.value = document.getElementById('search').value;
    render(state(), true);
    document.querySelector('.header-search').open = false;
  });
  addEventListener('popstate', () => render(readState()));
  draw.hidden = false;
  draw.addEventListener('click', () => {
    const picked = pullFromOven(packages, state(), Math.random());
    if (picked) location.assign(packageUrl(picked));
  });
}

function startHome() {
  // The home page lists few items, so it downloads the index only when asked to draw from it.
  draw.hidden = false;
  draw.addEventListener('click', async () => {
    draw.disabled = true;
    try {
      const picked = pullFromOven(await loadPackages(), new URLSearchParams(), Math.random());
      if (picked) location.assign(packageUrl(picked));
    } catch {
      draw.disabled = false;
    }
  });
}

if ('home' in shell.dataset) {
  startHome();
} else {
  try {
    startCatalog(await loadPackages());
  } catch {
    document.getElementById('matches').textContent += ' · Search is unavailable. Browse the numbered pages.';
    form.hidden = true;
  }
}
