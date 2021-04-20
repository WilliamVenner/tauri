import{_ as t,a as e}from"./tauri-e581da3f.js";import{i}from"./tauri-684c7ba7.js";var r;function o(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"readTextFile",path:r,options:o}})]}))}))}function n(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"readBinaryFile",path:r,options:o}})]}))}))}function u(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return"object"==typeof o&&Object.freeze(o),"object"==typeof r&&Object.freeze(r),[2,i({__tauriModule:"Fs",message:{cmd:"writeFile",path:r.path,contents:r.contents,options:o}})]}))}))}!function(t){t[t.Audio=1]="Audio",t[t.Cache=2]="Cache",t[t.Config=3]="Config",t[t.Data=4]="Data",t[t.LocalData=5]="LocalData",t[t.Desktop=6]="Desktop",t[t.Document=7]="Document",t[t.Download=8]="Download",t[t.Executable=9]="Executable",t[t.Font=10]="Font",t[t.Home=11]="Home",t[t.Picture=12]="Picture",t[t.Public=13]="Public",t[t.Runtime=14]="Runtime",t[t.Template=15]="Template",t[t.Video=16]="Video",t[t.Resource=17]="Resource",t[t.App=18]="App",t[t.Current=19]="Current"}(r||(r={}));function a(t){var e=function(t){if(t.length<65536)return String.fromCharCode.apply(null,Array.from(t));for(var e="",i=t.length,r=0;r<i;r++){var o=t.subarray(65536*r,65536*(r+1));e+=String.fromCharCode.apply(null,Array.from(o))}return e}(new Uint8Array(t));return btoa(e)}function s(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return"object"==typeof o&&Object.freeze(o),"object"==typeof r&&Object.freeze(r),[2,i({__tauriModule:"Fs",message:{cmd:"writeBinaryFile",path:r.path,contents:a(r.contents),options:o}})]}))}))}function c(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"readDir",path:r,options:o}})]}))}))}function d(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"createDir",path:r,options:o}})]}))}))}function f(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"removeDir",path:r,options:o}})]}))}))}function l(r,o,n){return void 0===n&&(n={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"copyFile",source:r,destination:o,options:n}})]}))}))}function m(r,o){return void 0===o&&(o={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"removeFile",path:r,options:o}})]}))}))}function p(r,o,n){return void 0===n&&(n={}),t(this,void 0,void 0,(function(){return e(this,(function(t){return[2,i({__tauriModule:"Fs",message:{cmd:"renameFile",oldPath:r,newPath:o,options:n}})]}))}))}var h=Object.freeze({__proto__:null,get BaseDirectory(){return r},get Dir(){return r},readTextFile:o,readBinaryFile:n,writeFile:u,writeBinaryFile:s,readDir:c,createDir:d,removeDir:f,copyFile:l,removeFile:m,renameFile:p});export{r as B,n as a,s as b,c,d,f as e,h as f,l as g,m as h,p as i,o as r,u as w};
