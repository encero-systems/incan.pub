'use strict';
// Static content stays readable with JavaScript disabled; this adds local controls only.
for (const button of document.querySelectorAll('[data-open]')) {
  button.addEventListener('click', () => document.getElementById(button.dataset.open).showModal());
}
for (const button of document.querySelectorAll('[data-copy]')) {
  button.addEventListener('click', async () => {
    const dialog = button.closest('dialog');
    const text = document.getElementById(button.dataset.copy).textContent;
    const status = dialog.querySelector('.copy-status');
    try { await navigator.clipboard.writeText(text); status.textContent = 'Copied.'; }
    catch { status.textContent = 'Copy is unavailable. Select the text above to copy it.'; }
  });
}
function updateTableHints() {
  for (const hint of document.querySelectorAll('.scroll-hint')) {
    const region = hint.nextElementSibling;
    hint.hidden = !region || region.scrollWidth <= region.clientWidth;
  }
}
updateTableHints();
window.addEventListener('resize', updateTableHints);
window.addEventListener('load', updateTableHints);

// The shared search disclosure overlays content, keeping header geometry fixed.
const headerSearch = document.querySelector('.header-search');
if (headerSearch) {
  headerSearch.addEventListener('toggle', () => {if (headerSearch.open) headerSearch.querySelector('input').focus();});
  document.addEventListener('keydown', event => {
    if (event.key === 'Escape' && headerSearch.open) {headerSearch.open = false; headerSearch.querySelector('summary').focus();}
  });
  document.addEventListener('click', event => {if (!headerSearch.contains(event.target)) headerSearch.open = false;});
}

for (const select of document.querySelectorAll('[data-version]')) {
  select.addEventListener('change', () => {location.assign(select.value);});
}
