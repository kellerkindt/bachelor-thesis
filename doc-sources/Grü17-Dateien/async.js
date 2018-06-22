(function(lib,name,path,cdnpath,clickpath,wwwpath,prefix,version,document,window){var inited,ready;lib=lib||{};if(lib.init instanceof Function){throw new Error('will not redefine '+lib.name);}
if(!lib.publickey){throw new Error('missing publickey');}
lib.status=0;lib.type='async';lib.name=lib.name||name;lib.publickey=lib.publickey||'';lib.path=lib.path||path;lib.cdnpath=lib.cdnpath||cdnpath;lib.clickpath=lib.clickpath||clickpath;lib.wwwpath=lib.wwwpath||wwwpath;lib.prefix=lib.prefix||prefix;lib.sandbox=lib.debug||lib.sandbox;lib.dataMode=lib.dataMode||'data-widget';lib.dataModeGroup=lib.dataModeGroup||'data-plista-placement';lib.context=lib.context||{};lib.context.widgets={};lib.context.script=function(){};lib.readyState=0;lib.version=version;if(lib.isTouch===void 0){lib.isTouch='ontouchstart'in document.documentElement;}
if(lib.useDocumentReady===void 0){lib.useDocumentReady=true;}
if(lib.isFrame===void 0){try{lib.isFrame=window.top!==window;}catch(error){lib.isFrame=true;}}
function replaceHttps(){var http=/^http:\/\//,https='https://';lib.path=lib.path.replace(http,https);lib.cdnpath=lib.cdnpath.replace(http,https);lib.clickpath=lib.clickpath.replace(http,https);lib.wwwpath=lib.wwwpath.replace(http,https);}
if(window.location&&window.location.protocol==='https:'){replaceHttps();}
function timeout(){if(lib.status<1){(new Image()).src=lib.path+'errorreport.php?publickey='+lib.publickey+'&msgkey=not_initialized&error='+encodeURIComponent(lib.name+' bad status '+lib.status+' after 30 seconds');}}
setTimeout(timeout,3e4);function onload(e){if(document.addEventListener||document.readyState==='interactive'||document.readyState==='complete'||e.type==='load'){detach();init();}}
function detach(){if(document.addEventListener){document.removeEventListener('DOMContentLoaded',onload,false);window.removeEventListener('load',onload,false);}else{document.detachEvent('onreadystatechange',onload);window.detachEvent('onload',onload);}}
function load(){if(!lib.useDocumentReady){init();}else if(document.attachEvent?document.readyState==='complete':document.readyState!=='loading'){setTimeout(init);}else if(document.addEventListener){document.addEventListener('DOMContentLoaded',onload,false);window.addEventListener('load',onload,false);}else{document.attachEvent('onreadystatechange',onload);window.attachEvent('onload',onload);if(!lib.isFrame&&document.documentElement&&document.documentElement.doScroll){(function scroll(){if(!ready){try{document.documentElement.doScroll('left');}catch(e){setTimeout(scroll,50);return;}
detach();init();}})();}}}
function init(){ready=true;var modules=lib.require||[];if(window.JSON&&JSON.stringify&&JSON.stringify([])==='[]'&&JSON.stringify({x:void 0})==='{}'){lib.JSON=window.JSON;}else{modules.push('json');}
if(document.querySelectorAll){lib.querySelector=function(selector){return document.querySelector(selector);};lib.querySelectorAll=function(selector){return document.querySelectorAll(selector);};}else{modules.push('sizzle');}
lib.loadModules(modules,lib.init);lib.status+=0.01;}
lib.init=function(){if(inited){return;}
inited=true;lib.readyState=1;lib.status+=0.1;lib.dispatchEvent(lib.event.INIT);};lib.reset=function(){if(!inited){return;}
inited=false;lib.status-=0.1;lib.readyState=0;lib.dispatchEvent(lib.event.UNLOAD);};try{(function(){var TRACE=true,log='';lib.debug=!!(lib.debug||String(window.location.hash).indexOf(lib.name+'.debug')>-1);lib.log=function(){var date,stack,args=lib.array(arguments);try{if(TRACE){stack=new Error().stack.match(/((https?:)?\/\/.+?):(\d+:\d+)/g);stack.splice(0,1);args.unshift('\n\n\n');args.push('\n\n\n');args.push(stack.join('\n'));}}catch(error){}
date=new Date();if(date.toLocaleTimeString&&date.getMilliseconds){args.unshift(date.toLocaleTimeString()+'.'+date.getMilliseconds());}
log+=args.join(' \n');if(lib.debug&&window.console){try{console.log.apply(console,args);}catch(error){console.log(args.toString());}}};lib.getLog=function(){return log;};}());(function(){lib.error=function(type,message,error,rand){lib.log('ERROR',type,message,error);rand=rand>=0?rand:0.95;error=error||{};var e={};e.version=lib.version;e.message=type+':'+message;e.description=error.name&&error.name+':'+error.message;e.file=error.fileName&&error.fileName+':'+error.lineNumber;e.stack=error.stack&&error.stack.toString().substr(0,1000);if(Math.random()>=rand){(new Image()).src=lib.path+'errorreport.php?publickey='+lib.publickey+'&msgkey=remotedata&error='+encodeURIComponent(lib.JSON.stringify(e));}};}());(function(){lib.bind=function(f,t){return Function.prototype.bind?f.bind(t):function(){f.apply(t,arguments);};};lib.array=function(a){for(var i=0,r=[],l=a.length;i<l;r[i]=a[i],i+=1);return r;};lib.trim=function(s){return String.prototype.trim?String.prototype.trim.call(String(s)):String(s).replace(/^\s+|\s+$/g,'');};lib.formatString=function(){var i,s=arguments[0];for(i=1;i<arguments.length;i+=1){s=s.replace(new RegExp('\\{'+(i-1)+'\\}','gm'),arguments[i]);}
return s;};lib.evalScript=function(s,t){var r,f;if(s){try{lib.currentTarget=t;f=new Function('lib',s);r=f.apply(t,[lib]);lib.currentTarget=void 0;}catch(error){lib.log('EVAL_SCRIPT',error);}}
return r;};lib.evalScriptTags=function(d,t){var i,p,n,e,s=d.getElementsByTagName?lib.array(d.getElementsByTagName('script')):[],l=s.length;for(i=0;i<l;i+=1){e=s[i];p=e.getAttribute('type');if(!p||p==='text/javascript'){if(e.src){e.parentNode.removeChild(e);n=document.createElement('script');n.src=e.src;d.appendChild(n);}else if(e.innerHTML){lib.evalScript(e.innerHTML,t);}}}};}());(function(){lib.unique=function(a,b){var x,y,r=[];if(b===void 0){b=r;}else{a=lib.unique(a.concat(b));}
label:for(x=0;x<a.length;x+=1){for(y=0;y<b.length;y+=1){if(b[y]===a[x]){continue label;}}
r.push(a[x]);}
return r;};}());(function(){var NAME='EVENT',ERROR_DISPATCH='error dispatching ',ERROR_FUNCTION=' listener is no function';lib.event={CLICK:'click',COMPLETE:'complete',INIT:'init',INSIGHT:'insight',LOAD:'load',EMPTY:'empty',MESSAGE:'message',MOUSEDOWN:'mousedown',MOUSEOVER:'mouseover',MOUSEOUT:'mouseout',ORIENTATION_CHANGE:'orientationchange',RESIZE:'resize',SCROLL:'scroll',UNLOAD:'unload',ADD_ITEM:'additem',COUNT_ENGAGEMENT:'countengagement',IS_ARTICLE:'isarticle',RECOMMENDATION_SEEN:'recommendationseen',SEND_ITEM:'senditem',WIDGET_LOAD:'widgetload',WIDGET_VIEW:'widgetview',WIDGET_INIT:'widgetinit',WIDGET_REQUEST:'widgetrequest'};function EventDispatcher(){var listeners={};function callListener(listener,type,data){try{listener.call(this,data);}catch(error){if(lib.debug){throw error;}
lib.error(NAME,ERROR_DISPATCH+type,error,0);}}
this.addEventListener=function(type,listener,element){if(typeof listener!=='function'){throw new Error(type+ERROR_FUNCTION);}
if(element===void 0){listeners[type]=listeners[type]||[];listeners[type].push(listener);}else if(element.addEventListener){element.addEventListener(type,listener,false);}else if(element.attachEvent){element.attachEvent('on'+type,listener);}};this.removeEventListener=function(type,listener,element){var i,ls;if(element===void 0){if(listener===void 0){listeners[type]=[];}else if(listeners[type]){ls=listeners[type];for(i=0;i<ls.length;i+=1){if(ls[i]===listener){ls[i]=void 0;}}}}else if(element.removeEventListener){element.removeEventListener(type,listener,false);}else if(element.detachEvent){element.detachEvent('on'+type,listener);}};this.dispatchEvent=function(type,data){var ls,i;lib.log(NAME,type,data);if(typeof this['on'+type]==='function'){callListener.call(this,this['on'+type],type,data);}
if(listeners[type]){ls=listeners[type];for(i=0;i<ls.length;i+=1){if(ls[i]){callListener.call(this,ls[i],type,data);}}
for(i=ls.length-1;i>=0;i-=1){if(ls[i]===void 0){ls.splice(i,1);}}}};}
lib.EventDispatcher=EventDispatcher;lib.EventDispatcher.call(lib);}());(function(){function AMP(widget){if(window.context&&context.requestResize){this.widget=widget;this.queueResize=lib.bind(this.queueResize,this);this.resize=lib.bind(this.resize,this);this.remove=lib.bind(this.remove,this);this.addEventListeners();}}
AMP.prototype.addEventListeners=function(){var i,images=this.widget.container.getElementsByTagName('img');for(i=0;i<images.length;i+=1){lib.addEventListener(lib.event.LOAD,this.queueResize,images[i]);}
lib.addEventListener(lib.event.RESIZE,this.queueResize,window);this.widget.addEventListener(lib.event.UNLOAD,this.remove);this.interval=setInterval(this.resize,3e3);};AMP.prototype.queueResize=function(){clearTimeout(this.timeout);this.timeout=setTimeout(this.resize,800);};AMP.prototype.resize=function(){var r=lib.getRect(this.widget.container);context.requestResize(void 0,r.height+10);};AMP.prototype.remove=function(){context.requestResize(0,0);lib.removeEventListener(lib.event.RESIZE,this.queueResize,window);this.widget.removeEventListener(lib.event.UNLOAD,this.remove);clearInterval(this.interval);};AMP.init=function(widget){new AMP(widget);};lib.addEventListener(lib.event.WIDGET_INIT,AMP.init);}());(function(lib,document){var NAME='WIDGET',LOG_NOT_FOUND='could not find widget {0}',ATTR_CLICKS='data-clicks',ATTR_REPLACE='data-replace-url',ATTR_REDIRECT='data-redirect',ATTR_MOUSEDOWN='this.href=this.getAttribute(\''+ATTR_REDIRECT+'\');';function Widget(widget){var c,t;lib.EventDispatcher.call(this);this.name=widget.name;this.id=widget.id;this.uid=widget.uid||widget.id;this.bv=widget.bv;this.items=widget.items;this.campaigns=widget.campaigns;this.language=widget.language;this.optout=widget.optout;this.isVisible=widget.visible;this.events=widget.events;this.completeEvents={};this.viewtrackers=widget.viewtrackers;this.impressionHTML=widget.ihtml;this.ast=widget.ast;this.group=widget.group;this.oba=lib.context.WIDGET_OBA!==void 0?lib.context.WIDGET_OBA:widget.oba;this.view=Widget.getViewSettings(widget.view);this.addCSS(widget.css);this.additionalScript();this.container=this.getContainer();if(!this.container){lib.log(NAME,lib.formatString(LOG_NOT_FOUND,this.id));return;}
c=lib.prefix+widget.name;t=this.container.className;this.className=t;this.container.id=this.uid;this.container.setAttribute(lib.dataMode,'');this.container.className=t&&t!==c?c+' '+t:c;this.container.innerHTML=widget.html||'';if(widget.onsite===false&&lib.hasAdBlock()===1){this.container.style.display='none';}
this.remove=lib.bind(this.remove,this);lib.placements[this.uid]=this;Widget.replaceUrl.call(this);lib.addEventListener(lib.event.UNLOAD,this.remove);lib.dispatchEvent(lib.event.WIDGET_INIT,this);this.addImpressionHTML();lib.evalScript(widget.script,this);}
Widget.assign=function(prototype){for(var i in Widget.prototype){if(Widget.prototype.hasOwnProperty(i)){prototype[i]=Widget.prototype[i];}}};Widget.replaceUrl=function(){var i,t=lib.querySelectorAll('#'+this.uid+' .'+lib.prefix+this.name+'_item');for(i=0;i<t.length;i+=1){Widget.addClickListener(t[i]);Widget.addReplaceUrlListener(t[i]);}};Widget.addReplaceUrlListener=function(item){if(item.getAttribute(ATTR_REPLACE)){item.setAttribute(ATTR_REDIRECT,item.href);item.href=item.getAttribute(ATTR_REPLACE);item.setAttribute('onmousedown',ATTR_MOUSEDOWN);item.removeAttribute(ATTR_REPLACE);}};Widget.addClickListener=function(item){item.clickItem=lib.bind(Widget.clickItem,item);lib.addEventListener(lib.event.MOUSEDOWN,item.clickItem,item);lib.addEventListener(lib.event.CLICK,item.clickItem,item);};Widget.clickItem=function(){var i,c=this.getAttribute(ATTR_CLICKS);if(c){this.removeAttribute(ATTR_CLICKS);lib.removeEventListener(lib.event.MOUSEDOWN,this.clickItem,this);lib.removeEventListener(lib.event.CLICK,this.clickItem,this);c=lib.JSON.parse(c);for(i=0;c&&c.length&&i<c.length;i+=1){(new Image()).src=c[i];}}};Widget.getViewSettings=function(data){data=data||{};data.percent=data.percent>=0?data.percent:0;data.time=data.time>=0?data.time:3e3;data.last=data.last>=0?data.last:0;return data;};Widget.prototype.addTrackingEvent=function(event){this.events=this.events||[];this.events.push(event);this.runCompleteTrackingEvents();};Widget.prototype.runCompleteTrackingEvents=function(){for(var i in this.completeEvents){if(this.completeEvents.hasOwnProperty(i)){this.trackingEvent(i);}}};Widget.prototype.trackingEvent=function(type){for(var e,i=0;this.events&&i<this.events.length;i+=1){e=this.events[i];if(e&&e[type]){Widget.tracking(e[type]);e[type]=void 0;}}
this.completeEvents[type]=true;};Widget.tracking=function(urls){var i,frame,url;for(i=0;i<urls.length;i+=1){url=urls[i];if(String(url).indexOf('iframe:')===0){frame=document.createElement('iframe');frame.style.cssText='position:absolute;top:1px;left:-1px;width:1px;height:1px;visibility:hidden !important;';document.body.insertBefore(frame,document.body.firstChild);frame.src=url.substr(7);}else{(new Image()).src=url;}}};Widget.prototype.viewTracking=function(){this.dispatchEvent(lib.event.WIDGET_VIEW);if(this.events){this.trackingEvent('visible');}else if(this.viewtrackers&&this.viewtrackers.length){for(var i=0;i<this.viewtrackers.length;i+=1){(new Image()).src=this.viewtrackers[i];}}};Widget.prototype.onvisible=function(time){if(this.view.impression>=0&&time>=this.view.impression){this.view.impression=void 0;this.isVisible=true;this.addImpressionHTML();}};Widget.prototype.addImpressionHTML=function(){if(!this.isVisible){return;}
this.trackingEvent('impression');this.dispatchEvent(lib.event.RECOMMENDATION_SEEN);if(this.impressionHTML){var t=document.createElement('div');t.style.display='none';t.innerHTML=this.impressionHTML;this.container.appendChild(t);if(lib.evalScriptTags){lib.evalScriptTags(t,this);}}};Widget.prototype.additionalScript=function(){var w=lib.context.widgets&&lib.context.widgets[this.name];if(w&&w.script){try{w.script.call(this);}catch(e){lib.error(NAME,this.name,e);}}};Widget.prototype.addCSS=function(css){var w=lib.context.widgets&&lib.context.widgets[this.name];css=css||w&&w.css;lib.addCSS(css,this.name);};Widget.prototype.getContainer=function(){if(this.group){return lib.querySelector('['+lib.dataModeGroup+'="'+this.group+'"]').appendChild(document.createElement('div'));}
return document.getElementById(this.id)||lib.querySelector('['+lib.dataMode+'="'+this.id+'"]');};Widget.prototype.remove=function(){this.dispatchEvent(lib.event.UNLOAD);lib.removeEventListener(lib.event.UNLOAD,this.remove);lib.placements[this.uid]=void 0;if(this.group){this.container.parentNode.locked=false;this.container.parentNode.removeChild(this.container);}else{this.container.setAttribute(lib.dataMode,this.id);this.container.id='';this.container.className=this.className;this.container.innerHTML='';}
this.completeEvents={};};lib.Widget=Widget;lib.placements={};}(lib,document));(function(){var ATTR_NAME='data-pub';function init(widgets){var i,w,n;for(i=0;i<widgets.length;i+=1){w=widgets[i];if(typeof w==='string'){continue;}
w.pub=w.pub||{};if(w.pre){w.pub.click=w.pre;w.pre=void 0;}
if(w.group){lib.context.tracking['group_'+w.group]=w.pub;}else{n=w.name;lib.context.tracking[n]=lib.context.tracking[n]||[];lib.context.tracking[n].push(w.pub);}
w.pub=void 0;}}
function add(widget){var i,a,t,p,w;if(widget.group){w=lib.context.tracking['group_'+widget.group];p=w;}else{w=lib.context.tracking[widget.name];p=w&&w.shift?w.shift():{};}
widget.pub=p||{};if(!widget.items||!widget.items.length){return;}
if(widget.pub.impression){widget.addEventListener(lib.event.RECOMMENDATION_SEEN,impression);}
if(widget.pub.click){t=lib.querySelectorAll('#'+widget.uid+' .'+lib.prefix+widget.name+'_item');for(i=0;i<t.length;i+=1){a=t[i];a.setAttribute(ATTR_NAME,widget.pub.click);lib.addEventListener(lib.event.MOUSEDOWN,click,a);lib.addEventListener(lib.event.CLICK,click,a);}}}
function impression(){(new Image()).src=this.pub.impression;}
function click(){if(this.getAttribute(ATTR_NAME)){(new Image()).src=this.getAttribute(ATTR_NAME);this.removeAttribute(ATTR_NAME);}}
lib.context.tracking=lib.context.tracking||{};lib.addEventListener(lib.event.WIDGET_REQUEST,init);lib.addEventListener(lib.event.WIDGET_INIT,add);}());(function(){lib.getRect=function(element){var r=element.getBoundingClientRect();if(r.width===void 0){return{top:r.top,bottom:r.bottom,left:r.left,right:r.right,width:r.right-r.left,height:r.bottom-r.top};}
return r;};}());(function(){var hasWebp,webp=new Image();webp.onload=webp.onerror=function(){hasWebp=webp.height===2;};webp.src='data:image/webp;base64,UklGRjoAAABXRUJQVlA4IC4AAACyAgCdASoCAAIALmk0mk0iIiIiIgBoSygABc6WWgAA/veff/0PP8bA//LwYAAA';lib.hasWebp=function(){return hasWebp;};}());(function(){var video=document.createElement('video'),hasVideo=video&&video.canPlayType?true:void 0;lib.hasVideo=function(){return hasVideo;};}());(function(){var plugin,version=0;try{plugin=navigator.plugins&&navigator.plugins['Shockwave Flash'];if(plugin){version=parseFloat(plugin.description.replace(/^.*?(\d+\.\d+).*$/,'$1'));}else if(ActiveXObject){plugin=new ActiveXObject('ShockwaveFlash.ShockwaveFlash');version=parseFloat(plugin.GetVariable('$version').replace(/^.*?(\d+),(\d+).*$/,'$1.$2'));}}catch(error){}
lib.hasFlash=function(){return version>=10||void 0;};}());(function(){var hasAdBlock;function isInvisible(d){var s=window.getComputedStyle(d,null);if(s.MozBinding&&s.MozBinding.match(/url\("about:(abp|sab)/)){return d.clientHeight===0;}
return s.display==='none';}
function append(d){if(window.top!==window){try{window.parent.document.body.appendChild(d);}catch(error){document.body.appendChild(d);}}else{document.body.appendChild(d);}}
lib.hasAdBlock=function(){return hasAdBlock;};lib.detectAdBlock=function(complete){var a=document.createElement('a'),b=document.createElement('div'),w=document.createElement('div');try{w.id='plwldplwldplwld';w.style.display='block';w.style.position='absolute';w.style.left='-999em';w.style.width='1px';w.style.height='1px';w.appendChild(document.createTextNode('&nbsp;'));append(w);b.style.position='absolute';b.className='plistaList';a.className='itemLinkPET';a.href='ht'+'tp://click.plista.com/pets/abp';a.style.display='block';a.style.position='absolute';a.style.left='-999em';a.style.width='1px';a.style.height='1px';a.appendChild(document.createTextNode('&nbsp;'));b.appendChild(a);append(b);hasAdBlock=isInvisible(w)?2:isInvisible(a)?1:0;}catch(e){}
if(w.parentNode){w.parentNode.removeChild(w);}
if(b.parentNode){b.parentNode.removeChild(b);}
if(complete){complete();}};lib.detectAdBlock();}());(function(){var NAME='LOAD SCRIPT',ERROR_TIMEOUT='timeout({0}) src {1}',head=document.getElementsByTagName('head')[0]||document.documentElement;lib.loadScript=function(src,complete,timeout,ontimeout){ontimeout=ontimeout||defaultTimeout;var t,loaded,s=document.createElement('script');s.async='async';s.type='text/javascript';s.charset='utf-8';s.onload=s.onreadystatechange=function(){if(!loaded&&(!this.readyState||this.readyState==='loaded'||this.readyState==='complete')){loaded=true;s.onload=s.onreadystatechange=null;clearTimeout(t);if(complete){complete(s);}
s.parentNode.removeChild(s);}};timeout=timeout||20000;t=setTimeout(function(){ontimeout(timeout,src);},timeout);s.src=src;head.insertBefore(s,head.firstChild);return s;};function defaultTimeout(timeout,src){lib.error(NAME,lib.formatString(ERROR_TIMEOUT,timeout,src&&src.substr(0,1000)));}}());(function(){var loadedModules=[],additionalModules=[];lib.addModules=function(modules){additionalModules=additionalModules.concat(modules);};lib.loadModules=function(modules,complete){if(additionalModules.length){modules=modules.concat(additionalModules);additionalModules=[];}
modules=lib.unique(modules,loadedModules);if(modules.length){loadedModules=loadedModules.concat(modules);lib.loadScript(lib.cdnpath+'async/module/'+modules.join(',')+'/'+encodeURIComponent(lib.name)+'.js',complete);return true;}
if(complete){complete();}
return false;};}());(function(){var json={name:lib.name,sb:lib.debug||lib.sandbox||void 0};lib.call=function(name,data){if(name){lib.queueCall(name,data);}
json.publickey=lib.publickey;json.objectid=lib.getObjectId();json.clientid=lib.clientid;json.nt=lib.noTracking;json.rev=lib.context&&lib.context.rev||0;json.cmpvc=lib.cmpvc||void 0;json.ctype=lib.ctype||void 0;json.ssp=lib.ssp;json.geo=lib.geo;json.uid=lib.uid;json.app=lib.app||lib.isApp;if(json.req&&json.req.length){lib.callRequest(json);json.req=void 0;}};lib.queueCall=function(name,data){var req=[name];if(data){req.push(data);}
json.req=json.req||[];json.req.push(req);};lib.callRequest=function(data){lib.loadScript(lib.path+'async_lib.js?json='+encodeURIComponent(lib.JSON.stringify(data))+'&x='+(new Date()).getTime());};lib.bvd={};}());(function(){lib.getCSS=function(e,p,d){d=d||'';return e.style[p]?e.style[p]:window.getComputedStyle?window.getComputedStyle(e)[p]:e.currentStyle?e.currentStyle[p]:d;};}());(function(){var style,head,addedCSS;lib.addCSS=function(css,key){if(!style||!style.parentNode){addedCSS={};style=document.createElement('style');style.type='text/css';head=document.getElementsByTagName('head')[0]||document.documentElement;head.insertBefore(style,head.firstChild);}
if(!css||key in addedCSS){return;}
if(style.textContent!==void 0){style.textContent+=css;}else{style.styleSheet.cssText+=css;}
if(key){addedCSS[key]=true;}};}());(function(lib){function init(){if(!lib.getObjectId()){return;}
var d;if(lib.adaas){d={};}
else if(lib.updateItem){cleanItem();d=lib.item;}else if(lib.item.updated_at){cleanDate('updated_at');d={updated_at:lib.item.updated_at};}
lib.addEventListener(lib.event.IS_ARTICLE,isArticle);lib.queueCall(lib.event.IS_ARTICLE,d);}
function isArticle(e){lib.removeEventListener(lib.event.IS_ARTICLE,isArticle);if(!lib.sandbox&&e.action==='sendItemData'){lib.sendItem();}
lib.status+=10;}
function convertHTMLEntities(s){var t=document.createElement('textarea');t.innerHTML='_'+s.replace(/(<([^>]+)>)/ig,"")+'_';s=t.value;return s.substr(1,s.length-2);}
function cleanItem(){var i,v,item=lib.item;for(i in item){if(item.hasOwnProperty(i)&&item[i]){v=lib.trim(item[i]);if(v.match(/^https?:\/\//)){v=v.substr(0,1024);}else{v=convertHTMLEntities(v).substr(0,255);}
item[i]=v;}}
cleanDate('published_at');cleanDate('created_at');cleanDate('updated_at');}
function cleanDate(key){if(lib.item[key]&&Number(lib.item[key])>1e11){lib.item[key]=Math.floor(Number(lib.item[key])/1e3);}}
lib.item=lib.item||{};lib.updateItem=lib.updateItem||false;lib.adaas=lib.adaas||false;lib.sendItem=function(){lib.dispatchEvent(lib.event.SEND_ITEM);if(lib.getObjectId()){cleanItem();lib.call(lib.event.ADD_ITEM,lib.item);}};lib.getObjectId=function(){return lib.item&&lib.item.objectid?lib.trim(lib.item.objectid):void 0;};lib.addEventListener(lib.event.LOAD,init);lib.addEventListener(lib.event.EMPTY,function empty(){lib.call();});}(lib));(function(){lib.getCookie=function(name){try{var match=String(document.cookie).match(new RegExp(' ?'+name+'=(.*?)(;|$)'));return match&&match[1]?match[1]:null;}catch(e){return null;}};lib.setCookie=function(name,value,expires,path){path=path||'/';expires=Number(expires)||0;try{document.cookie=name+'='+value+(expires>=0?'; expires='+new Date(expires).toUTCString():'')+'; path='+path;}catch(e){}};lib.deleteCookie=function(name){lib.setCookie(name,'',0);};}());(function(){var CNAME='plista_referral',pref,dref;lib.getReferrerHost=function(){var ref=dref+'/';ref=ref.match(/^.*?\/\/.*?\//);return ref?ref[0]:'';};lib.getReferrer=function(){try{return String(document.referrer);}catch(e){return'';}};lib.isOnsiteReferral=function(){return pref?pref===dref:false;};function load(){pref=lib.getCookie(CNAME);dref=lib.getReferrer();}
function onclick(){lib.setCookie(CNAME,document.location.href,new Date().getTime()+10000);}
function init(widget){var i,items=lib.querySelectorAll('.'+lib.prefix+widget.name+'_item');for(i=0;i<items.length;i+=1){lib.addEventListener(lib.event.MOUSEDOWN,onclick,items[i]);lib.addEventListener(lib.event.CLICK,onclick,items[i]);}}
function complete(){lib.deleteCookie(CNAME);}
lib.addEventListener(lib.event.LOAD,load);lib.addEventListener(lib.event.WIDGET_INIT,init);lib.addEventListener(lib.event.COMPLETE,complete);}());(function(){var ready,loading,pm,cmpWindow,cmpTimeout,cmpId,CMP_TIMEOUT=5000,VENDOR_IDS=[32,91,98,177];function init(){loading=true;if(lib.ssp||lib.noCache){cmpinit();}else{lib.loadScript((lib.apipath||lib.cdnpath)+'async/'+lib.publickey+'/'+encodeURIComponent(lib.name)+'.js',cmpinit);}}
function cmpinit(){if(!window.postMessage||!document.addEventListener){uidload();}else if(typeof window.__cmp==='function'||cmpwindow()){cmpTimeout=setTimeout(cmpload,CMP_TIMEOUT);try{if(lib.isFrame){cmppost();}else{window.__cmp('ping',null,cmpping);}}catch(e){cmpload();}}else{uidload();}}
function cmpwindow(){if(cmpWindow!==void 0){return cmpWindow;}
cmpWindow=null;var f=window;while(!cmpWindow){try{if(f.frames["__cmpLocator"]||f.__cmp){cmpWindow=f;return cmpWindow;}}catch(e){return;}
if(f===window.top){return;}
f=f.parent;}
return cmpWindow;}
function cmppost(){lib.addEventListener('message',cmpmessage,window);cmpId=':'+new Date().getTime()+':'+Math.random();cmpWindow.postMessage({__cmpCall:{command:'getVendorConsents',parameter:VENDOR_IDS,callId:cmpId,}},'*');lib.log('CMP','post',cmpWindow);}
function cmpmessage(e){var d=typeof e.data==="string"?lib.JSON.parse(e.data):e.data;if(!d.__cmpReturn||d.__cmpReturn.callId!==cmpId){return;}
lib.log('CMP','iframe getVendorConsents',d);cmpload(d.__cmpReturn.returnValue);}
function cmpping(result){lib.log('CMP','ping',result);window.__cmp('getVendorConsents',VENDOR_IDS,cmpload);}
function cmpload(result){if(result){lib.ctype='Iab';lib.cmpvc=result;lib.log('CMP','getVendorConsents',result);}
if(cmpTimeout===null){return;}
lib.removeEventListener('message',cmpmessage,window);clearTimeout(cmpTimeout);cmpTimeout=null;uidload();}
function uidload(){if(lib.loadUid&&!window.PLISTA_UID){lib.loadScript(lib.path+'getuid.php',preload);}else{preload();}}
function preload(){if(window.PLISTA_UID){lib.uid=window.PLISTA_UID;}
loading=false;if(ready){load();}}
function load(){if(loading){ready=true;return;}
lib.readyState=2;lib.status+=1;if(lib.modules&&lib.modules.length){lib.addModules(lib.modules);}
if(lib.context.css){lib.addCSS(lib.context.css,lib.publickey);}
if(lib.context.script){lib.context.script();}
lib.dispatchEvent(lib.event.LOAD);lib.detectAdBlock(lib.loadWidgets);}
function complete(widgets){var i;for(i in widgets){if(widgets.hasOwnProperty(i)&&widgets[i]){lib.addWidget(widgets[i]);}}
lib.readyState=4;lib.status+=100;lib.dispatchEvent(lib.event.COMPLETE);}
function widgetload(e){lib.removeEventListener(lib.event.WIDGET_LOAD,widgetload);if(e.success){e.useragent=e.useragent||{};lib.language=e.useragent.language||'en';lib.browser=e.useragent.browser||'unknown';lib.system=e.useragent.system||'unknown';lib.loadModules(e.modules,function(){complete(e.widgets);});}}
function getOutstreamWidth(){try{var q=lib.prefix+'outstream',o=lib.querySelector('[id^="'+q+'"],['+lib.dataMode+'^="'+q+'"]');if(o){return lib.getRect(o).width;}}catch(error){}
return 0;}
function supportsPostMessage(){if(pm!==void 0){return pm;}
if(!window.postMessage||!document.addEventListener){pm=false;return pm;}
var onlyStrings=false;try{window.postMessage({toString:function(){onlyStrings=true;}},"*");}catch(e){}
if(onlyStrings){pm=false;return pm;}
pm=true;return pm;}
lib.activeWidgets=[];lib.enabled=lib.enabled||[];lib.disabled=lib.disabled||[];lib.loadUid=lib.loadUid||false;lib.addWidget=function(widget){try{new lib[widget.base](widget);}catch(error){lib.error('WIDGET','failed to construct '+widget&&widget.base,error,0);}};lib.loadWidgets=function(widgets){var i,ii,w,rw;if(lib.ssp||lib.noCache){lib.activeWidgets=lib.widgets||[];}
widgets=widgets||lib.widgets||[];if(widgets.length){rw=true;}else{if(lib.activeWidgets.length){for(i=0;i<lib.activeWidgets.length;i+=1){w=lib.activeWidgets[i];for(ii=0;ii<lib.querySelectorAll('#'+lib.prefix+w+', ['+lib.dataMode+'="'+lib.prefix+w+'"]').length;ii+=1){widgets.push(w);}}}
var g,groups=lib.querySelectorAll('['+lib.dataModeGroup+']');for(i=0;i<groups.length;i+=1){g=groups[i];if(!g.locked){g.locked=true;widgets.push({group:g.getAttribute(lib.dataModeGroup)});}}}
if(!rw&&lib.enabled){widgets=widgets.concat(lib.unique(lib.enabled));}
if(!widgets.length){lib.loadModules([]);lib.dispatchEvent(lib.event.EMPTY);return false;}
lib.addEventListener(lib.event.WIDGET_LOAD,widgetload);lib.dispatchEvent(lib.event.WIDGET_REQUEST,widgets);lib.call(lib.event.WIDGET_LOAD,lib.getSettings({widgets:widgets,rw:rw}));return true;};lib.getAppNexusPreviewId=function(){var previewId,href=window.location?window.location.href:null;previewId=href?String(href).match(/plista_override_ast=(\d+)/i):null;return previewId&&previewId[1]||void 0;};lib.getSettings=function(data){data=data||{};var uc=[];if(lib.getCookie('slide.removed')){uc.push('slide');}
if(lib.getCookie('sticky.removed')){uc.push('sticky');}
if(uc.length){data.uc=uc;}
if(lib.disabled&&lib.disabled.length){data.ds=lib.unique(lib.disabled);}
data.c=lib.categories;data.ab=lib.ab||lib.hasAdBlock();data.fl=lib.hasFlash();data.vo=lib.hasVideo();data.pm=supportsPostMessage();data.sc=window.screen&&screen.width+'x'+screen.height;data.px=window.devicePixelRatio||1;data.xc=lib.exclude||void 0;data.fr=lib.isFrame||void 0;data.nc=lib.noCache||void 0;data.tc=lib.isTouch||void 0;data.fc=lib.fc!==void 0?lib.fc:void 0;data.kw=lib.keywords;data.or=lib.isOnsiteReferral()||void 0;data.rf=lib.getReferrerHost()||void 0;data.fv=lib.fv;data.ap=lib.getAppNexusPreviewId();data.wb=lib.hasWebp();data.vp=!!lib.querySelector('meta[name="viewport"]')||void 0;data.ww=lib.getRect(document.documentElement).width||0;data.ow=getOutstreamWidth()||void 0;return data;};init();lib.addEventListener(lib.event.INIT,load);}());(function(){var t,widgets=[];function init(){if(lib.widgets&&lib.widgets.length){lib.widgets.push=function(){Array.prototype.push.apply(widgets,arguments);Array.prototype.push.apply(lib.widgets,arguments);if(lib.readyState>2){loadWidgets();}};}}
function loadWidgets(){clearTimeout(t);t=setTimeout(loadWidgetsDelayed,50);}
function loadWidgetsDelayed(){if(widgets.length){lib.loadWidgets(widgets);widgets=[];}}
lib.addEventListener(lib.event.INIT,init);lib.addEventListener(lib.event.COMPLETE,loadWidgets);}());(function(){var y,v=[],MIN=10,TICK=250,AMP=false,lastChanges;function Visibility(widget){this.widget=widget;this.reset();this.isVisible();}
Visibility.prototype.calculate=function(){var x,b,w,r,y=true;if(!AMP){r=lib.getRect(this.widget.container);w=getWindowRect();}else if(!this.lastChange){return false;}else{r=this.lastChange.boundingClientRect;w=this.lastChange.rootBounds;}
if(r.width<=MIN||r.height<=MIN){return false;}
if(this.widget.view.last){b=r.bottom<=w.height&&r.bottom>=0;}else{x=r.height*this.widget.view.percent;b=r.bottom>=x&&r.top<=w.height-x;}
if(!AMP){if(lib.isFrame){y=w.scrollHeight>=(r.height+100);}
return b&&r.right>=0&&r.left<=(w.width-100)&&y;}
return b;};Visibility.prototype.isVisible=function(){if(this.calculate()){var t=(new Date()).getTime()-this.t;this.widget.onvisible(t);if(t>=this.widget.view.time){this.widget.viewTracking();return true;}}else{this.reset();}
return false;};Visibility.prototype.reset=function(){this.t=(new Date()).getTime();};function init(widget){if(lib.hasAdBlock()===1||lib.sandbox){return;}
v.push(new Visibility(widget));y=y||setInterval(tick,TICK);if(lastChanges){observeIntersection(lastChanges);lastChanges=null;}}
function tick(){for(var i=0;i<v.length;i+=1){if(v[i]&&v[i].isVisible()){v[i]=void 0;}}}
function unload(){clearInterval(y);y=void 0;v=[];}
function observeIntersection(changes){if(!v.length){lastChanges=changes;return;}
for(var z=0;z<changes.length;z+=1){for(var i=0;i<v.length;i+=1){if(v[i]){v[i].lastChange=changes[z];if(v[i].isVisible()){v[i]=void 0;}}}}}
function getWindowRect(){var db=document.body||document.documentElement,de=document.documentElement;return{width:window.innerWidth||de.clientWidth||db.clientWidth,height:window.innerHeight||de.clientHeight||db.clientHeight,scrollHeight:Math.max(db.scrollHeight,db.offsetHeight,de.clientHeight,de.scrollHeight,de.offsetHeight)};}
if(window.context&&window.context.observeIntersection){AMP=true;window.context.observeIntersection(observeIntersection);}
lib.addEventListener(lib.event.WIDGET_INIT,init);lib.addEventListener(lib.event.UNLOAD,unload);}());(function(){var PREFIX=lib.prefix+'optout_popup',EVENT='optoutrender';function OptoutPopup(target,widget){this.target=target;this.widget=widget;this.close=lib.bind(this.close,this);this.onmessage=lib.bind(this.onmessage,this);this.target.onclick=lib.bind(this.open,this);}
OptoutPopup.prototype.open=function(e){e=e||window.event;e.cancelBubble=true;if(e.stopPropagation){e.stopPropagation();}
if(this.active){return false;}
this.active=true;if(this.html){this.render();}else{lib.addEventListener(EVENT,this.onmessage);lib.call(EVENT,{optout:this.getOptout(),language:this.widget.language});}
return false;};OptoutPopup.prototype.getOptout=function(){var i,n,item,y=0,data={},items=lib.querySelectorAll('#'+this.widget.uid+' .'+lib.prefix+this.widget.name+'_item');for(i=0;i<items.length;i+=1){item=items[i];if(item.getAttribute('data-type')==='default'){continue;}
n=item.getAttribute('data-adv-optout');if(n){data[y]={name:n,url:item.getAttribute('data-adv-optout')};}
y+=1;}
return data;};OptoutPopup.prototype.onmessage=function(data){lib.removeEventListener(EVENT,this.onmessage);this.html=data.html;this.render();};OptoutPopup.prototype.render=function(){this.container=document.createElement('div');this.container.className=PREFIX;this.container.innerHTML=this.html;this.addCloseButton();var position=lib.getCSS(this.widget.container,'position');if(position==='static'){this.widget.container.style.position='relative';}
this.widget.container.appendChild(this.container);};OptoutPopup.prototype.addCloseButton=function(){var close=new Image();close.src=lib.cdnpath+'images/x_black.png';close.className=PREFIX+'_close';close.onclick=this.close;this.container.insertBefore(close,this.container.firstChild);};OptoutPopup.prototype.close=function(){this.container.parentNode.removeChild(this.container);this.widget.container.style.position='';this.active=false;};lib.OptoutPopup=OptoutPopup;lib.addCSS('.'+PREFIX+'{position:absolute;text-align: left;z-index: 4;max-width:90%;width:320px;background:#fff;bottom:0;right:0;font-family:Arial, Helvetica, sans-serif;font-size:12px;box-shadow: 0 5px 40px rgba(0,0,0,0.3);padding:12px 12px 32px;line-height: 17px;}'+'.'+PREFIX+' img{width:auto!important;height:10px!important; }'+'.'+PREFIX+' img:nth-child(2){width:auto!important;height:28px!important; }'+'.'+PREFIX+' .'+PREFIX+'_close{position: absolute;width:18px!important;height:18px!important;cursor:pointer;background-size:18px 18px;top:15px;right:15px;}'+'.'+PREFIX+' a{color: #69b4de;text-decoration:none}',PREFIX);}());(function(){var height=14,width=14,mwidth=112,pxr=window.devicePixelRatio>1?2:1,cs=lib.prefix+'oba';function init(){lib.addCSS(''+'.'+cs+'{display:block;cursor:pointer;margin-left:1px;width:'+width+'px;height:'+(height-1)+'px;background-repeat:no-repeat}'+'.'+cs+':hover{width:'+mwidth+'px}'+'.'+cs+'_hint{width:'+mwidth+'px!important}'+'.'+cs+'_right{float:right;background-position:right top}'+'.'+cs+'_right:hover{background-position:right -'+height+'px;}'+'.'+cs+'_left{margin-left:0}'+'.'+cs+'_left:hover{background-position:left -'+height+'px;}'+'span[data-oba="text"]:hover{color:transparent!important;white-space:nowrap;}'+'span[data-oba="text"] .'+cs+'{margin-top:-1px;}'+'span[data-oba="text"] .'+cs+'_left{float:left;}','oba');}
function background(p,a,t,l){return'url('+lib.cdnpath+'image/'+(p.getAttribute('data-pcd')?'pcd':'ad')+'choices'+(t?'2':'')+'/'+l+'/'+((height-1)*pxr)+'/'+color(p)+'/'+a+'/1.png'+')';}
function color(p){var c=lib.getCSS(p,'color');if(c){c=c.replace(/ /g,'').match(/([0-9]+),([0-9]+),([0-9]+)/);c=c&&c[0]?c[0]:null;}
return c||'939393';}
function position(p){var l,n=document.createElement('span');n.className='plista-oba-image';p.parentNode.insertBefore(n,p);l=lib.getCSS(n,'position','static')==='absolute';p.parentNode.removeChild(n);return l?'left':'right';}
function optout(p){var o,i=0;while(p.parentNode&&i<10){o=p.getAttribute('data-adv-optout');if(o){return o;}
p=p.parentNode;i+=1;}
return'';}
function paint(p,widget){if(p.hasOBA){return;}
p.hasOBA=true;var t,l=widget.language,s=document.createElement('span'),r=p.getAttribute('data-popup')!==null;s.setAttribute('data-optout',optout(p)||widget.optout);s.className=cs;if(pxr===2){s.style.backgroundSize='auto '+(height*pxr)+'px';}
if((widget.oba===1||r)){if(widget.items&&widget.items.length>1){new lib.OptoutPopup(s,widget);}else{s.onclick=click;}
p.style.textAlign='right';setbg(p,s,0,l);append(p,s,r);}else{s.onclick=click;t=p.getAttribute('data-oba').match(/^2;/);if(position(p)==='left'){p.setAttribute('data-position','left');}
setbg(p,s,t,l);append(p,s,true);}}
function append(p,s,r){if(p.tagName.toLowerCase()==='span'&&p.className.indexOf('plistaPetImg')>-1){p.setAttribute('data-oba','text');p.appendChild(s);}else if(r){p.parentNode.insertBefore(s,p);p.parentNode.removeChild(p);if(p.tagName.toLowerCase()==='img'){s.className+=' '+cs+'_hint';}
s.className='plista-oba '+s.className;}else{p.insertBefore(s,p.firstChild);}}
function setbg(p,s,t,l){if(p.getAttribute('data-position')==='left'){s.style.backgroundImage=background(p,2,t,l);s.className+=' '+cs+'_left';}else{s.style.backgroundImage=background(p,0,t,l);s.className+=' '+cs+'_right';}}
function click(e){window.open(this.getAttribute('data-optout'),'_newtab');e=e||window.event;e.cancelBubble=true;if(e.stopPropagation){e.stopPropagation();}
return false;}
function render(widget){var i,q,w;if(widget.container&&widget.container.id){if(widget.oba===1){q='#'+widget.container.id+' .plista-powered, #'+widget.container.id+' [title="powered by plista"]';}else if(widget.oba===2){q='#'+widget.container.id+' span[data-oba], #'+widget.container.id+' img[data-oba]';}
w=lib.querySelectorAll(q);for(i=0;i<w.length;i+=1){paint(w[i],widget);}
if(widget.oba!==1){return;}
q='#'+widget.container.id+' img[alt="plista Blume"], #'+widget.container.id+' img[alt="powered by plista"]';w=lib.querySelectorAll(q);for(i=0;i<w.length;i+=1){paint(w[i].parentNode,widget);}
q='#'+widget.container.id+' img[src*="logo_plista_"]';try{w=lib.querySelectorAll(q);}catch(error){w=[];}
for(i=0;i<w.length;i+=1){paint(w[i].parentNode,widget);}}}
lib.createObaButtons=render;lib.renderOBA=paint;lib.addEventListener(lib.event.WIDGET_INIT,render);lib.addEventListener(lib.event.LOAD,init);}());load();lib.status+=0.001;}catch(error){(new Image()).src=lib.path+'errorreport.php?publickey='+lib.publickey+'&msgkey=remotedata&error='+encodeURIComponent(error.toString()+(error.stack&&':'+error.stack.toString().substr(0,1000)));}}(window.PLISTA,'PLISTA','https://farm.plista.com/','https://static.plista.com/','https://click.plista.com/','https://www.plista.com/','plista_widget_','',document,window));