"use strict";var t,e=require("./tauri-1e18b462.js"),n=require("./tauri-d62e0985.js");exports.ResponseType=void 0,(t=exports.ResponseType||(exports.ResponseType={}))[t.JSON=1]="JSON",t[t.Text=2]="Text",t[t.Binary=3]="Binary";var r=function(){function t(t,e){this.type=t,this.payload=e}return t.form=function(e){return new t("Form",e)},t.json=function(e){return new t("Json",e)},t.text=function(e){return new t("Text",e)},t.bytes=function(e){return new t("Bytes",e)},t}(),i=function(){function t(t){this.id=t}return t.prototype.drop=function(){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(t){return[2,n.invokeTauriCommand({__tauriModule:"Http",message:{cmd:"dropClient",client:this.id}})]}))}))},t.prototype.request=function(t){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(e){return[2,n.invokeTauriCommand({__tauriModule:"Http",message:{cmd:"httpRequest",client:this.id,options:t}})]}))}))},t.prototype.get=function(t,n){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(r){return[2,this.request(e._assign({method:"GET",url:t},n))]}))}))},t.prototype.post=function(t,n,r){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(i){return[2,this.request(e._assign({method:"POST",url:t,body:n},r))]}))}))},t.prototype.put=function(t,n,r){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(i){return[2,this.request(e._assign({method:"PUT",url:t,body:n},r))]}))}))},t.prototype.patch=function(t,n){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(r){return[2,this.request(e._assign({method:"PATCH",url:t},n))]}))}))},t.prototype.delete=function(t,n){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(r){return[2,this.request(e._assign({method:"DELETE",url:t},n))]}))}))},t}();function o(t){return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(e){return[2,n.invokeTauriCommand({__tauriModule:"Http",message:{cmd:"createClient",options:t}}).then((function(t){return new i(t)}))]}))}))}var u=null;function s(t,n){var r;return e.__awaiter(this,void 0,void 0,(function(){return e.__generator(this,(function(i){switch(i.label){case 0:return null!==u?[3,2]:[4,o()];case 1:u=i.sent(),i.label=2;case 2:return[2,u.request(e._assign({url:t,method:null!==(r=null==n?void 0:n.method)&&void 0!==r?r:"GET"},n))]}}))}))}var a=Object.freeze({__proto__:null,get ResponseType(){return exports.ResponseType},Body:r,Client:i,getClient:o,fetch:s});exports.Body=r,exports.Client=i,exports.fetch=s,exports.getClient=o,exports.http=a;
