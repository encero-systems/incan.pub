(() => {
  const anchor = document.querySelector('.incus-secret');
  if (!anchor) return;
  const button = anchor.querySelector('button');
  const header = document.querySelector('.header-inner');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  const gaze = createGaze(anchor,button,reduced);
  const sleeping = document.createElement('img');
  sleeping.src = '/arcade/loaf-run/cameos/doze.png';
  sleeping.alt = '';
  sleeping.className = 'incus-dozing';
  sleeping.hidden = true;
  if (header) header.append(sleeping);
  const key = 'loaves.incus.doze.v1';
  let shown = false, idleTimer, exitTimer, pose = '';
  try { shown = sessionStorage.getItem(key) === '1' } catch {}
  function clearPose() {
    gaze.stop();
    clearTimeout(exitTimer);
    pose = '';
    delete anchor.dataset.pose;
    sleeping.hidden = true;
    button.setAttribute('aria-pressed','false');
    button.setAttribute('aria-label','Knock on the kiln seal');
  }
  function remember() {
    shown = true;
    try { sessionStorage.setItem(key,'1') } catch {}
  }
  function busy() {
    return document.hidden || !document.hasFocus() || Boolean(document.querySelector('dialog[open],.header-search[open]')) ||
      document.activeElement?.matches('input,textarea,select,[contenteditable="true"]');
  }
  function doze() {
    if (shown || reduced.matches || pose || busy()) return;
    const nav = header?.querySelector('nav');
    const launch = header?.querySelector('.run-launch');
    if (!nav || !launch) return;
    const available = nav.getBoundingClientRect().left - launch.getBoundingClientRect().right;
    if (available >= 90 && header.getBoundingClientRect().bottom > 0) {
      sleeping.style.left = (nav.getBoundingClientRect().left - header.getBoundingClientRect().left - 65) + 'px';
      sleeping.hidden = false;
    } else {
      const box = anchor.getBoundingClientRect();
      if (box.top < header.getBoundingClientRect().bottom || box.bottom > innerHeight) return;
      anchor.dataset.pose = 'doze';
    }
    pose = 'doze'; remember(); exitTimer = setTimeout(clearPose,9000);
  }
  function rearm() {
    clearTimeout(idleTimer);
    if (pose === 'doze') clearPose();
    if (!shown && !pose && !reduced.matches && !busy()) idleTimer=setTimeout(doze,45000);
  }
  button.addEventListener('click',()=>{
    const wasPeek=pose==='peek';
    clearPose(); clearTimeout(idleTimer);
    if (wasPeek){rearm();return}
    pose='peek';anchor.dataset.pose='peek';gaze.start();
    button.setAttribute('aria-pressed','true');button.setAttribute('aria-label','Send Incus back behind the seal');
    exitTimer=setTimeout(()=>{clearPose();rearm()},7000);
  });
  document.addEventListener('keydown',event=>{
    if(event.key==='Escape' && pose){clearPose()}
    rearm();
  });
  for(const event of ['pointermove','pointerdown','wheel','scroll','focusin','focusout'])
    document.addEventListener(event,rearm,{capture:true,passive:true});
  window.addEventListener('resize',rearm);
  document.addEventListener('visibilitychange',()=>{clearPose();rearm()});
  window.addEventListener('blur',()=>{clearPose();clearTimeout(idleTimer)});
  window.addEventListener('focus',rearm);
  reduced.addEventListener('change',()=>{clearPose();rearm()});
  rearm();
// A two-pose pixel-art turn. Only the head moves; the source armor and paws
// remain registered to the original 90 × 96 artwork.
function createGaze(anchor,button,reduced) {
  const artwork=button.querySelector('img.peek');
  const fine=matchMedia('(any-hover: hover) and (any-pointer: fine)');
  const view=document.createElement('canvas');
  const context=view.getContext('2d');
  if(!context)return {start(){},stop(){}};
  view.className='cameo-pose peek gaze';
  view.setAttribute('aria-hidden','true');
  view.width=90;view.height=104;
  let head,body,active=false,raf=0,previous=0;
  let lookX=0,lookY=0,targetX=0,targetY=0,facing=1;
  const pivotX=47,pivotY=54;
  function layer(){
    const c=document.createElement('canvas');c.width=90;c.height=96;
    return c;
  }
  function prepare(){
    if(head||!artwork.complete||!artwork.naturalWidth)return;
    head=layer();body=layer();
    const h=head.getContext('2d'),b=body.getContext('2d');
    // Trace the underside of the muzzle, above the first row of shoulder armor.
    const jaw=new Path2D();
    jaw.moveTo(0,0);jaw.lineTo(90,0);jaw.lineTo(90,53);jaw.lineTo(66,53);
    jaw.lineTo(59,55);jaw.lineTo(48,56);jaw.lineTo(37,53);jaw.lineTo(27,52);
    jaw.lineTo(0,52);jaw.closePath();
    h.clip(jaw);h.drawImage(artwork,0,0);
    b.drawImage(artwork,0,0);b.globalCompositeOperation='destination-out';b.fill(jaw);
    b.globalCompositeOperation='destination-over';
    b.drawImage(artwork,39,56,19,7,35,48,27,10);
    button.append(view);
  }
  function paint(){
    context.setTransform(1,0,0,1,0,8);context.clearRect(0,-8,90,104);context.imageSmoothingEnabled=false;
    if(!lookX&&!lookY){context.drawImage(artwork,0,0);return}
    context.drawImage(body,0,0);
    // Hysteresis keeps the two profile poses steady near the pointer's center.
    if(lookX<-.24)facing=-1;
    if(lookX>-.08)facing=1;
    context.save();
    context.translate(pivotX+Math.round(lookX*2),pivotY+Math.round(lookY*1.5));
    context.rotate(lookX*.065+facing*lookY*.12);
    context.scale(facing,1);context.drawImage(head,-pivotX,-pivotY);context.restore();
  }
  function animate(time){
    raf=0;if(!active)return;
    const dt=previous?Math.min(50,time-previous):16;previous=time;
    const mix=1-Math.exp(-dt/70);
    lookX+=(targetX-lookX)*mix;lookY+=(targetY-lookY)*mix;
    if(Math.abs(lookX-targetX)<.012)lookX=targetX;
    if(Math.abs(lookY-targetY)<.012)lookY=targetY;
    paint();
    if(lookX!==targetX||lookY!==targetY)raf=requestAnimationFrame(animate);
    else previous=0;
  }
  function aim(x,y){
    targetX=x;targetY=y;if(!raf)raf=requestAnimationFrame(animate);
  }
  function neutral(){aim(0,0)}
  function move(e){
    if(e.pointerType!=='mouse'){neutral();return}
    const box=artwork.getBoundingClientRect();
    const dx=e.clientX-box.left-box.width*.52,dy=e.clientY-box.top-box.height*.35;
    if(Math.hypot(dx,dy)>360){neutral();return}
    const clamp=n=>Math.max(-1,Math.min(1,n));
    aim(clamp(dx/85),clamp(dy/100));
  }
  function stop(){
    active=false;cancelAnimationFrame(raf);raf=0;previous=0;
    lookX=lookY=targetX=targetY=0;facing=1;delete anchor.dataset.gaze;
    document.removeEventListener('pointermove',move);
    document.removeEventListener('pointerleave',neutral);
    window.removeEventListener('scroll',neutral,true);
  }
  function start(){
    if(!fine.matches||reduced.matches)return;
    prepare();if(!head)return;
    active=true;anchor.dataset.gaze='on';paint();
    document.addEventListener('pointermove',move,{passive:true});
    document.addEventListener('pointerleave',neutral);
    window.addEventListener('scroll',neutral,{capture:true,passive:true});
  }
  artwork.addEventListener('load',()=>{if(anchor.dataset.pose==='peek')start()},{once:true});
  fine.addEventListener('change',()=>{stop();if(anchor.dataset.pose==='peek')start()});
  return {start,stop};
}

})();
