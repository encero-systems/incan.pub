/* In-page navigation and install action */
const rail = document.querySelector('.section-rail');
const links = Array.from(rail.querySelectorAll('a[href^="#"]'));
const sections = links.map(link => document.getElementById(link.hash.slice(1)));
const marker = document.createElement('i');
marker.className = 'rail-marker';
marker.setAttribute('aria-hidden', 'true');
rail.append(marker);

const install = document.querySelector('.rail-install');
const topbar = document.querySelector('.site-header');
let active = null;
let queued = false;
function updateLayout() {
    queued = false;
    const headerBottom = topbar.getBoundingClientRect().bottom;
    const readingLine = headerBottom + 96;
    const desktop = getComputedStyle(install).position === 'sticky';
    const docked = desktop && Math.abs(install.getBoundingClientRect().top - (headerBottom + 10)) < 2;
    install.classList.toggle('is-docked', docked);

    let current = links[0];
    for (const [index, section] of sections.entries()) {
        if (!section) continue;
        if (section.id === 'metadata' && desktop) continue;
        if (section.getBoundingClientRect().top <= readingLine) current = links[index];
    }
    if (current !== active) {
        for (const link of links) {
            link.classList.toggle('active', link === current);
            if (link === current) link.setAttribute('aria-current','location');
            else link.removeAttribute('aria-current');
        }
        active = current;
    }
    const y = current.offsetTop + current.offsetHeight / 2 - 8;
    marker.style.transform = 'translateY(' + y + 'px)';
}
function schedule() {
    if (!queued) { queued = true; requestAnimationFrame(updateLayout); }
}
window.addEventListener('scroll', schedule, { passive: true });
window.addEventListener('resize', schedule);
new ResizeObserver(schedule).observe(document.querySelector('.document-layout'));
document.fonts.ready.then(schedule);
updateLayout();

/* Use the recorded data tables and the already rendered SVG points as one pair. */
function enableInspection(card, table, registry) {
    const rows = Array.from(table.querySelectorAll('tbody tr'), row => ({
        day: row.cells[0].textContent.trim(),
        value: row.cells[1].textContent.trim()
    }));
    const svg = card.querySelector('svg');
    const points = svg?.querySelector('polyline')?.points;
    if (!points || points.numberOfItems !== rows.length || !rows.length) return;

    const shell = document.createElement('div');
    shell.className = 'mini-chart inspect-chart';
    const link = document.createElement('a');
    link.className = 'chart-link'; link.href = card.href;
    link.append(card.querySelector('span'), card.querySelector('strong'));
    const plot = document.createElement('div');
    plot.className = 'plot';
    const slider = document.createElement('input');
    slider.type = 'range'; slider.min = '0'; slider.max = String(rows.length - 1);
    slider.step = '1'; slider.value = slider.max;
    slider.setAttribute('aria-label', registry ? 'Inspect daily registry pulls' : 'Inspect daily crates.io downloads');
    const dot = document.createElementNS('http://www.w3.org/2000/svg','circle');
    dot.setAttribute('fill','currentColor'); dot.setAttribute('r','2.7');
    svg.append(dot);
    const readout = document.createElement('div');
    readout.className = 'daily-readout';
    const recorded = card.querySelector('small');
    plot.append(svg, slider);
    shell.append(link, plot, readout, recorded);
    card.replaceWith(shell);

    function show(index) {
        const sample = rows[index], point = points.getItem(index);
        const partial = registry && index === rows.length-1 && table.closest('details').nextElementSibling?.textContent.includes('partial UTC day');
        const label = sample.day + ' · ' + sample.value + (registry ? ' pulls' : ' downloads') + (partial ? ' · partial' : '');
        slider.value = String(index);
        slider.setAttribute('aria-valuetext', label);
        readout.textContent = label;
        dot.setAttribute('cx', point.x); dot.setAttribute('cy', point.y);
    }
    slider.addEventListener('input', () => show(Number(slider.value)));
    function inspectAt(event) {
        const rect = svg.getBoundingClientRect();
        const x = (event.clientX-rect.left)/rect.width*240;
        show(Math.max(0,Math.min(rows.length-1, Math.round((x-2)/236*(rows.length-1)))));
    }
    slider.addEventListener('pointermove', event => {
        if (event.pointerType === 'mouse' && !event.buttons) inspectAt(event);
    });
    slider.addEventListener('pointerup', inspectAt);
    show(rows.length-1);
}
const tables = document.querySelectorAll('#popularity details table');
document.querySelectorAll('.mini-chart').forEach((card,index) => {
    if (tables[index]) enableInspection(card,tables[index],index===1);
});
