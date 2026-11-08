'use strict';
const canvas=document.querySelector('canvas'), ctx=canvas.getContext('2d');
const W=480,H=184,ground=156,world=876,oven=810;
const sprite=new Image();sprite.src='incus-run-v2.png';
const parcels=[
 {x:84,y:91,name:'serde',slug:'serde'}, {x:255,y:42,name:'regex',slug:'regex'},
 {x:415,y:88,name:'memchr',slug:'memchr'}, {x:584,y:37,name:'tokio',slug:'tokio'},
 {x:718,y:94,name:'clap',slug:'clap'}
];
const ledges=[{x:169,w:57,y:131},{x:236,w:77,y:108},{x:479,w:62,y:130},{x:551,w:83,y:101}];
const vents=[{x:323,phase:0},{x:656,phase:1.65}];
const score=document.querySelector('#score'),statusLine=document.querySelector('#state'),startButton=document.querySelector('#start'),pauseButton=document.querySelector('#pause'),clock=document.querySelector('#clock'),receipt=document.querySelector('#receipt');
const reduced=matchMedia('(prefers-reduced-motion:reduce)');
let mode='ready',x,y,vy,camera,face,t,loaves,keys=new Set(),tick=0,previous=0,jumpQueue=false;
let onFloor=true,checkpoint=30,recoverUntil=0;
const knockoutDuration=0.95;
let hitAt=null;
function reset(){
 hitAt=null;
 x=30;y=ground;vy=0;face=1;t=0;camera=0;keys.clear();jumpQueue=false;onFloor=true;checkpoint=30;recoverUntil=0;
 loaves=parcels.map(p=>({...p,hit:false,at:-10}));score.textContent='0 / 5 loaves';clock.textContent='0.0 s';receipt.hidden=true;
}
function message(s){if(statusLine.textContent!==s)statusLine.textContent=s}
function begin(){reset();mode='playing';message('Pack three loaves and reach the kiln.');startButton.textContent='Restart';pauseButton.disabled=false;pauseButton.textContent='Pause';canvas.focus();wake()}
function pause(){if(mode==='playing'){mode='paused';keys.clear();jumpQueue=false;message('Paused');pauseButton.textContent='Resume'}else if(mode==='paused'){mode='playing';message(hitAt===null?'Back on the kiln road.':'Too hot!');pauseButton.textContent='Pause';canvas.focus();wake()}}
function wake(){if(!tick){previous=0;tick=requestAnimationFrame(frame)}}
function ventPhase(v){return (t+v.phase)%3.6}
function finish(){
 mode='won';keys.clear();const packed=loaves.filter(l=>l.hit);const perfect=packed.length===5;
 pauseButton.disabled=true;startButton.textContent='Run again';message(perfect?'Perfect batch!':'Batch baked!');
 const key='loaf-run.v2.best.'+packed.length;let best=t;
 try{const prior=Number(localStorage.getItem(key));if(prior>0)best=Math.min(t,prior);localStorage.setItem(key,String(best))}catch{}
 receipt.hidden=false;document.querySelector('#finish-time').textContent=packed.length+' loaves in '+t.toFixed(1)+' s · Best for this batch size: '+best.toFixed(1)+' s';
 const list=document.querySelector('#packed-items');list.replaceChildren();
 for(const p of packed){const item=document.createElement('li'),a=document.createElement('a');a.href='/packages/crates-io/'+p.slug+'/';a.textContent=p.name;item.append(a);list.append(item)}
}
function advance(dt){
 t+=dt;clock.textContent=t.toFixed(1)+' s';
 if(hitAt!==null){
  if(t-hitAt>=knockoutDuration){
   hitAt=null;x=checkpoint;y=ground;vy=0;onFloor=true;recoverUntil=t+1.4;
   camera=Math.max(0,Math.min(world-W,x-W*.38));
   message('Back at the checkpoint with your loaves.');
  }
  return;
 }
 const dir=Number(keys.has('right'))-Number(keys.has('left'));if(dir)face=dir;
 x=Math.min(world-36,Math.max(0,x+dir*115*dt));
 if(jumpQueue&&onFloor){vy=-254;onFloor=false}jumpQueue=false;
 const lastFoot=y,headHeight=38;vy+=490*dt;y+=vy*dt;onFloor=false;
 if(vy>=0){
   let floor=ground;
   for(const p of ledges)if(x+16>=p.x&&x+16<=p.x+p.w&&lastFoot<=p.y+.15&&y>=p.y)floor=Math.min(floor,p.y);
   if(y>=floor){y=floor;vy=0;onFloor=true}
 }
 for(const l of loaves){
   const bottom=l.y+12;
   if(!l.hit&&vy<0&&lastFoot-headHeight>=bottom&&y-headHeight<=bottom&&Math.abs(x+16-l.x)<18){
     l.hit=true;l.at=t;vy=35;y=bottom+headHeight;
     const count=loaves.filter(p=>p.hit).length;score.textContent=count+' / 5 loaves';
     message(l.name+' packed. '+(count===3?'The kiln is open!':count===5?'Perfect batch. Head to the kiln!':''));
   }
 }
 if(x>=390&&checkpoint===30){checkpoint=387;message('Checkpoint reached. Your batch is safe.')}
 for(const vent of vents){
   if(ventPhase(vent)>2.65&&Math.abs(x+16-vent.x)<13&&y>ground-31&&t>=recoverUntil){
     hitAt=t;vy=0;keys.clear();jumpQueue=false;message('Too hot!');return;
   }
 }
 camera=Math.max(0,Math.min(world-W,x-W*.38));
 if(x+16>=oven){
  if(loaves.filter(l=>l.hit).length>=3)finish();else message('Bring three loaves to light the kiln.');
 }
}
function rect(x,y,w,h,c){ctx.fillStyle=c;ctx.fillRect(Math.round(x),Math.round(y),w,h)}
function poly(pts,c){ctx.fillStyle=c;ctx.beginPath();pts.forEach(([x,y],i)=>i?ctx.lineTo(Math.round(x),Math.round(y)):ctx.moveTo(Math.round(x),Math.round(y)));ctx.closePath();ctx.fill()}
function scene(){
 ctx.imageSmoothingEnabled=false;
 for(let j=0;j<8;j++)rect(0,j*23,W,23,['#171c2b','#1d2334','#272c40','#323347','#413c4d','#55444b','#69514b','#765a50'][j]);
 for(let i=0;i<43;i++){let sx=((i*83+13-camera*.09)%W+W)%W;rect(sx,8+i*19%61,1,i%6===0?2:1,i%5===0?'#e7c89b':'#8d8290')}
 const moonX=360-camera*.12;
 rect(moonX,17,17,2,'#ba9c86');rect(moonX-3,19,23,6,'#d4b194');rect(moonX-4,25,25,7,'#e4c6a2');rect(moonX-2,32,21,4,'#c1a18e');rect(moonX+1,36,15,2,'#aa8990');
 for(let i=-1;i<10;i++){let sx=i*88-Math.round(camera*.22);let peak=57+(i+4)*17%39;poly([[sx-30,143],[sx+36,peak],[sx+117,143]],'#333747');poly([[sx+36,peak],[sx+55,peak+43],[sx+117,143]],'#414457')}
 for(let i=-1;i<15;i++){const sx=i*66-Math.round(camera*.47),roof=107+(i+4)*11%16;
 rect(sx,roof,52,155-roof,'#252d39');rect(sx+7,roof-4,37,4,'#343846');rect(sx+34,roof-19,7,17,'#292e3a');rect(sx+33,roof-20,9,3,'#4c4851');
 for(let w=0;w<3;w++){rect(sx+5+w*12,roof+8,6,8,'#9c7355');rect(sx+7+w*12,roof+8,2,8,'#2e323c')}}
 rect(0,ground,W,H-ground,'#24262f');rect(0,ground,W,2,'#d2a976');rect(0,ground+2,W,4,'#6d594a');
 for(let i=-1;i<23;i++){let sx=i*30-Math.round(camera%30);rect(sx,ground+6,2,7,'#171e26');rect(sx+14,ground+14,2,7,'#171e26');rect(sx+4,ground+11,21,1,'#50454a')}
 for(const p of ledges){
  const sx=p.x-camera;rect(sx,p.y,p.w,3,'#d8bb88');rect(sx,p.y+3,p.w,5,'#745747');
  for(let dx=3;dx<p.w;dx+=14)rect(sx+dx,p.y+6,3,6,'#473c42');
 }
 const flag=392-camera;rect(flag,ground-35,2,35,'#aa9783');poly([[flag+2,ground-35],[flag+18,ground-30],[flag+2,ground-25]],checkpoint>30?'#81dfca':'#8b8586');
 for(const vent of vents){
  const sx=vent.x-camera, phase=ventPhase(vent);
  rect(sx-13,ground-2,26,5,'#494952');for(let i=0;i<5;i++)rect(sx-10+i*5,ground-2,2,3,'#ffbb70');
  if(phase>2.65){
    for(let j=0;j<3;j++){const bx=sx-9+j*7,height=22+(j===1?12:0);poly([[bx-3,ground-2],[bx+1,ground-height],[bx+5,ground-3]],'#df7c49');rect(bx,ground-12,3,9,'#ffd184')}
  }else if(phase>1.85){rect(sx-10,ground-6,20,2,'#ffca7a');rect(sx-4,ground-14,8,2,'#e4bc85')}
 }
 const o=oven-camera;
 rect(o-19,ground-43,41,43,'#474151');rect(o-23,ground-42,48,4,'#a28260');rect(o-15,ground-49,32,6,'#6a5d5b');rect(o+10,ground-63,8,14,'#575465');rect(o-10,ground-26,22,26,'#c38f53');rect(o-7,ground-22,16,22,'#332e37');rect(o-4,ground-12,10,10,'#ea9952');rect(o-1,ground-8,5,6,'#ffce76');
 for(const l of loaves)drawLoaf(l);
 if(hitAt!==null){drawKnockout();return}
 const idx=!onFloor?3:keys.has('left')||keys.has('right')?[1,0,2,0][Math.floor(t*10)%4]:0;
 if(sprite.complete&&sprite.naturalWidth){
  ctx.save();ctx.translate(Math.round(x-camera+16),Math.round(y));ctx.scale(face,1);
  ctx.drawImage(sprite,idx*64,0,64,64,-22,-44,44,44);ctx.restore();
 }
}
function drawKnockout(){
 const progress=Math.min(1,(t-hitAt)/knockoutDuration);
 const cx=Math.round(x+16-camera),foot=Math.round(y);
 if(!sprite.complete||!sprite.naturalWidth)return;
 ctx.save();
 if(reduced.matches){
  ctx.translate(cx,foot);ctx.scale(face,1);
  ctx.drawImage(sprite,192,0,64,64,-22,-44,44,44);
 }else{
  // Brief held impact, a backward tumble, then a puff of cinders.
  const arc=Math.min(1,Math.max(0,(progress-.12)/.76));
  const rise=42*4*arc*(1-arc);
  ctx.translate(cx-face*Math.round(20*arc),foot-22-Math.round(rise));
  ctx.rotate(-face*arc*Math.PI*.95);ctx.scale(face,1);
  ctx.globalAlpha=Math.min(1,(1-progress)*5);
  ctx.drawImage(sprite,192,0,64,64,-22,-22,44,44);
 }
 ctx.restore();
 if(reduced.matches)return;
 for(let i=0;i<7;i++){
  const phase=progress*Math.PI;
  const spread=(i-3)*(2+progress*5);
  const height=Math.sin(phase)*(14+(i%3)*7)+progress*14;
  const size=progress>.7?2:3;
  rect(cx+spread,foot-5-height,size,size,i%2?'#ffd48d':'#e78955');
 }
 if(progress>.5){
  const smoke=(progress-.5)*2;
  for(let i=0;i<3;i++){
   rect(cx-face*18+(i-1)*(4+smoke*7),foot-20-smoke*(12+i*5),4,3,'#978c93');
  }
 }
}
function drawLoaf(l){
 let sx=l.x-camera;if(sx< -20||sx>W+20)return;
 if(l.hit){let age=t-l.at;if(age>.44||reduced.matches)return;for(let i=0;i<5;i++)rect(sx+(i-2)*age*25,l.y-8-age*42+age*age*60,2,2,'#f6d798');return}
 let h=l.y;
 rect(sx-8,h+2,17,9,'#513c35');rect(sx-6,h,13,2,'#d6a25b');rect(sx-9,h+3,19,5,'#e4b769');rect(sx-7,h+8,15,3,'#aa703f');
 for(let i=0;i<3;i++){rect(sx-5+i*5,h+2,2,3,'#9c653e');rect(sx-4+i*5,h+1,2,2,'#fff0b1')}
 ctx.fillStyle='#eee3ca';ctx.font='8px monospace';ctx.textAlign='center';ctx.fillText(l.name,Math.round(sx),h-3);ctx.textAlign='left';
 rect(sx-7,h+14,15,1,'#7f9c9a');
}
function frame(now){tick=0;const dt=previous?Math.min(.04,(now-previous)/1000):0;previous=now;
 if(mode==='playing'){advance(dt/2);if(mode==='playing')advance(dt/2)}
 scene();if(mode==='playing')tick=requestAnimationFrame(frame)}
function action(k,down){if(mode!=='playing'||hitAt!==null)return;if(k==='jump'){if(down)jumpQueue=true}else{if(down)keys.add(k);else keys.delete(k)}}
const keymap={ArrowLeft:'left',KeyA:'left',ArrowRight:'right',KeyD:'right',Space:'jump',ArrowUp:'jump',KeyW:'jump'};
window.addEventListener('keydown',e=>{if(mode!=='playing'||e.ctrlKey||e.metaKey||e.altKey||e.target!==canvas)return;const k=keymap[e.code];if(!k)return;e.preventDefault();if(!e.repeat)action(k,true)});
window.addEventListener('keyup',e=>{if(keymap[e.code])action(keymap[e.code],false)});
for(const b of document.querySelectorAll('[data-move]')){
 const k=b.dataset.move;
 b.addEventListener('pointerdown',e=>{e.preventDefault();b.setPointerCapture(e.pointerId);action(k,true)});
 for(const event of ['pointerup','pointercancel','lostpointercapture'])b.addEventListener(event,()=>action(k,false));
}
startButton.addEventListener('click',begin);pauseButton.addEventListener('click',pause);
window.addEventListener('blur',()=>{if(mode==='playing')pause()});
document.addEventListener('visibilitychange',()=>{if(document.hidden&&mode==='playing')pause()});
reset();scene();sprite.onload=scene;
