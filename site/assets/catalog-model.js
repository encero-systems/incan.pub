// Pure catalog operations, shared by the browser and the tests.
export const PAGE_SIZE = 50;

export function licenseIdentifiers(expression = '') {
  const tokens = expression.match(/[A-Za-z0-9][A-Za-z0-9.+-]*/g) || [];
  let exception = false;
  const identifiers = [];
  for (const token of tokens) {
    if (token === 'WITH') { exception = true; continue; }
    if (['AND', 'OR'].includes(token)) { exception = false; continue; }
    if (exception) { exception = false; continue; }
    identifiers.push(token);
  }
  return [...new Set(identifiers)];
}

const normalize = value => String(value).toLowerCase().replace(/[-_]/g, ' ');

function licenseMatches(item, license) {
  if (!license) return true;
  if (license.startsWith('id:')) return licenseIdentifiers(item.license).includes(license.slice(3));
  return license.startsWith('expression:') && item.license === license.slice(11);
}

function platformMatches(item, platform) {
  if (!platform) return true;
  if (platform === 'none') return item['build-targets'].length === 0;
  return item['build-targets'].some(target => target.includes(platform));
}

function byParams(params) {
  const sort = params.get('sort');
  const query = normalize(params.get('q') || '').trim();
  const rank = item => {
    const short = normalize(item.name.split('/')[1]);
    return short === query ? 0 : short.startsWith(query) ? 1 : 2;
  };
  return (a, b) => {
    if (sort === 'versions' && a.versions !== b.versions) return b.versions - a.versions;
    const aBuilt = a['build-targets'].length > 0, bBuilt = b['build-targets'].length > 0;
    if (sort === 'builds' && aBuilt !== bBuilt) return Number(bBuilt) - Number(aBuilt);
    if ((!sort || sort === 'name') && query && rank(a) !== rank(b)) return rank(a) - rank(b);
    return a.name < b.name ? -1 : a.name > b.name ? 1 : 0;
  };
}

// Every item the parameters select, in the order they list it.
export function matchingPackages(packages, params) {
  const words = normalize(params.get('q') || '').trim().split(/\s+/).filter(Boolean);
  const category = params.get('category') || '';
  const selected = packages.filter(item => {
    const haystack = normalize([item.name, item.description, ...(item.keywords || [])].join(' '));
    return words.every(word => haystack.includes(word))
      && licenseMatches(item, params.get('license') || '')
      && (!category || (item.categories || []).includes(category))
      && platformMatches(item, params.get('platform') || '');
  });
  return selected.sort(byParams(params));
}

// One page of that selection.
export function selectPackages(packages, params) {
  const selected = matchingPackages(packages, params);
  const pages = Math.max(1, Math.ceil(selected.length / PAGE_SIZE));
  const requested = Number(params.get('page') || 1);
  const page = Number.isSafeInteger(requested) ? Math.min(pages, Math.max(1, requested)) : 1;
  return {total: selected.length, pages, page, items: selected.slice((page - 1) * PAGE_SIZE, page * PAGE_SIZE)};
}

// An equal chance for each item the parameters select, or nothing when none is. `random` is in [0, 1).
export function pullFromOven(packages, params, random) {
  const selected = matchingPackages(packages, params);
  return selected.length ? selected[Math.floor(random * selected.length)] : null;
}
