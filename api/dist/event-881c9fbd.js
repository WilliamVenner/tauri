import{_ as n,a as t,t as i}from"./tauri-e581da3f.js";import{i as r}from"./tauri-684c7ba7.js";function e(e,o){return n(this,void 0,void 0,(function(){var s=this;return t(this,(function(c){return[2,r({__tauriModule:"Event",message:{cmd:"listen",event:e,handler:i(o)}}).then((function(i){return function(){return n(s,void 0,void 0,(function(){return t(this,(function(n){return[2,u(i)]}))}))}}))]}))}))}function u(i){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,r({__tauriModule:"Event",message:{cmd:"unlisten",eventId:i}})]}))}))}function o(i,r){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,e(i,r)]}))}))}function s(i,r){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,e(i,(function(n){r(n),u(n.id).catch((function(){}))}))]}))}))}function c(i,e,u){return n(this,void 0,void 0,(function(){return t(this,(function(n){switch(n.label){case 0:return[4,r({__tauriModule:"Event",message:{cmd:"emit",event:i,windowLabel:e,payload:u}})];case 1:return n.sent(),[2]}}))}))}export{c as e,o as l,s as o};
