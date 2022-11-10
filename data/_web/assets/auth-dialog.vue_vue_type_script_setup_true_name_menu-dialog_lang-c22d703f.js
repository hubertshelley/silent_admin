import{i as e}from"./index-174ad176.js";import{i as t,n as l,B as u,m as a,r as o,E as V}from"./index.js";import{d as n}from"./dict-2940434e.js";const d=Vue.createElementVNode("div",null,[Vue.createTextVNode(" 日志记录方式: "),Vue.createElementVNode("br"),Vue.createTextVNode("分为四种方式: "),Vue.createElementVNode("br"),Vue.createTextVNode("不记录 "),Vue.createElementVNode("br"),Vue.createTextVNode("文件记录 "),Vue.createElementVNode("br"),Vue.createTextVNode("数据库记录 "),Vue.createElementVNode("br"),Vue.createTextVNode("文件+数据库记录 ")],-1),r=Vue.createElementVNode("div",null,[Vue.createTextVNode(" 日志记录方式: "),Vue.createElementVNode("br"),Vue.createTextVNode("分为三种方式: "),Vue.createElementVNode("br"),Vue.createTextVNode("不缓存:每次都从数据库获取，用于经常更新的数据 "),Vue.createElementVNode("br"),Vue.createTextVNode("按访问人:用于缓存每个人访问数据都不同的数据 "),Vue.createElementVNode("br"),Vue.createTextVNode("公共缓存:用于数据更新较少，不区分个人的数据 ")],-1),c={class:"dialog-footer"},m=Vue.defineComponent({name:"menu-dialog"}),i=Vue.defineComponent({...m,props:{formData:{type:Object,required:!0},open:{type:Boolean,default:!1},title:{type:String,default:""}},emits:["closeDialog"],setup(m,{emit:i}){const s=m,f=Vue.ref(s.formData),p=t(n.sysNormalDisable,n.sysApiMethod,n.apiCacheMethod,n.apiLogMethod,n.sysShowHide,n.db),E=async()=>{const e={api_id:f.value.id,dbs:f.value.dbs},t={id:f.value.id,data_cache_method:f.value.data_cache_method,log_method:f.value.log_method},{data:n,execute:d}=l(u.edit,e),{data:r,execute:c}=a(o.updateLogCache,t);await Promise.all([d(),c()]),n.value!==V&&r.value!==V&&(ElementPlus.ElMessage.success("数据关联更新成功"),h())},h=()=>{i("closeDialog")};return(t,l)=>{const u=Vue.resolveComponent("el-checkbox"),a=Vue.resolveComponent("el-checkbox-group");return Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElDialog),{modelValue:m.open,"onUpdate:modelValue":l[3]||(l[3]=e=>Vue.isRef(open)?open.value=e:null),title:m.title,width:"680px","before-close":h,"append-to-body":""},{footer:Vue.withCtx((()=>[Vue.createElementVNode("div",c,[Vue.createVNode(Vue.unref(ElementPlus.ElButton),{type:"primary",onClick:E},{default:Vue.withCtx((()=>[Vue.createTextVNode(" 确 定 ")])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElButton),{onClick:h},{default:Vue.withCtx((()=>[Vue.createTextVNode(" 取 消 ")])),_:1})])])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElForm),{ref:"menuRef","label-width":"100px",class:"base-form"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:24},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"关联数据表"},{default:Vue.withCtx((()=>[Vue.createVNode(a,{modelValue:f.value.dbs,"onUpdate:modelValue":l[0]||(l[0]=e=>f.value.dbs=e)},{default:Vue.withCtx((()=>[(Vue.openBlock(!0),Vue.createElementBlock(Vue.Fragment,null,Vue.renderList(Vue.unref(p)[Vue.unref(n).db],(e=>(Vue.openBlock(),Vue.createBlock(u,{key:e.value,style:{width:"150px"},label:e.value},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElTooltip),{content:e.label+" : "+e.value,placement:"top"},{default:Vue.withCtx((()=>[Vue.createTextVNode(Vue.toDisplayString(e.value),1)])),_:2},1032,["content"])])),_:2},1032,["label"])))),128))])),_:1},8,["modelValue"])])),_:1})])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),null,{label:Vue.withCtx((()=>[Vue.createElementVNode("span",null,[Vue.createVNode(Vue.unref(ElementPlus.ElTooltip),{placement:"top"},{content:Vue.withCtx((()=>[d])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElIcon),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(e))])),_:1})])),_:1}),Vue.createTextVNode(" 日志记录 ")])])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElSelect),{modelValue:f.value.log_method,"onUpdate:modelValue":l[1]||(l[1]=e=>f.value.log_method=e),placeholder:"日志记录方式"},{default:Vue.withCtx((()=>[(Vue.openBlock(!0),Vue.createElementBlock(Vue.Fragment,null,Vue.renderList(Vue.unref(p)[Vue.unref(n).apiLogMethod],(e=>(Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElOption),{key:e.value,label:e.label,value:e.value},null,8,["label","value"])))),128))])),_:1},8,["modelValue"])])),_:1})])),_:1}),"GET"===f.value.method?(Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElCol),{key:0,span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),null,{label:Vue.withCtx((()=>[Vue.createElementVNode("span",null,[Vue.createVNode(Vue.unref(ElementPlus.ElTooltip),{placement:"top"},{content:Vue.withCtx((()=>[r])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElIcon),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(e))])),_:1})])),_:1}),Vue.createTextVNode(" 缓存方式 ")])])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElSelect),{modelValue:m.formData.data_cache_method,"onUpdate:modelValue":l[2]||(l[2]=e=>m.formData.data_cache_method=e),placeholder:"缓存方式"},{default:Vue.withCtx((()=>[(Vue.openBlock(!0),Vue.createElementBlock(Vue.Fragment,null,Vue.renderList(Vue.unref(p)[Vue.unref(n).apiCacheMethod],(e=>(Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElOption),{key:e.value,label:e.label,value:e.value},null,8,["label","value"])))),128))])),_:1},8,["modelValue"])])),_:1})])),_:1})):Vue.createCommentVNode("",!0)])),_:1})])),_:1},512)])),_:1},8,["modelValue","title"])}}});export{i as _};