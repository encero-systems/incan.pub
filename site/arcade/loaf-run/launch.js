document.addEventListener('DOMContentLoaded',()=>{
 if(typeof HTMLDialogElement==='undefined')return;
 const links=[...document.querySelectorAll('a[href="/arcade/loaf-run/"]')];
 if(!links.length)return;
 let pending=false;
 function openArcade(link){
  pending=false;
  const dialog=document.createElement('dialog');
  dialog.className='loaf-modal';
  dialog.setAttribute('aria-label','Incus Loaf Run');
  const bar=document.createElement('div');
  bar.className='loaf-modal-bar';
  const title=document.createElement('strong');
  title.textContent='Incus Loaf Run';
  const close=document.createElement('button');
  close.type='button';close.textContent='Close ×';close.autofocus=true;
  const frame=document.createElement('iframe');
  frame.title='Incus Loaf Run game';frame.src='/arcade/loaf-run/?embedded=1';
  const originScroll={left:window.scrollX,top:window.scrollY,behavior:'instant'};
  close.addEventListener('click',()=>dialog.close());
  dialog.addEventListener('click',event=>{if(event.target===dialog)dialog.close()});
  dialog.addEventListener('close',()=>{
   dialog.remove();link.focus({preventScroll:true});window.scrollTo(originScroll);
  },{once:true});
  bar.append(title,close);dialog.append(bar,frame);document.body.append(dialog);
  dialog.showModal();close.focus({preventScroll:true});
 }
 for(const link of links){
  link.setAttribute('aria-haspopup','dialog');
  link.addEventListener('click',event=>{
   if(event.defaultPrevented||event.metaKey||event.ctrlKey||event.altKey||event.shiftKey||event.button!==0)return;
   event.preventDefault();
   if(pending||document.querySelector('dialog.loaf-modal[open]'))return;
   const header=link.parentElement;
   const nav=header.querySelector('nav');
   const left=link.offsetLeft+4;
   const distance=(nav?.offsetLeft??header.clientWidth)-left-38;
   if(!link.matches('.run-launch')||distance<45||matchMedia('(prefers-reduced-motion: reduce)').matches){
    openArcade(link);return;
   }
   pending=true;
  const runner=document.createElement('span');runner.className='run-sprite';runner.style.left=left+'px'; header.append(runner);link.querySelector('img').style.opacity='0';
  const start=performance.now(), duration=Math.min(1150,distance*3.8);
  function step(t){
   const elapsed=t-start, progress=Math.min(1,elapsed/duration);
   runner.style.transform='translateX('+Math.round(distance*progress)+'px)';
   runner.style.backgroundPositionX=(-32*[1,0,2,0][Math.floor(elapsed/105)%4])+'px';
   if(progress<1){requestAnimationFrame(step);return}
   runner.remove();link.querySelector('img').style.opacity='';openArcade(link);
  }
  requestAnimationFrame(step);
  });
 }
});
