import{i as e,m as l,q as u,E as t,n as a,k as o,r}from"./index.js";import{u as V}from"./useFormUtil-98c17c20.js";import{d as n}from"./dict-2940434e.js";const d={class:"dialog-footer"},s=Vue.defineComponent({__name:"role-manage-dialog",props:{roleData:{type:Object,required:!0},open:{type:Boolean,default:!1},title:{type:String,default:""}},emits:["closeDialog"],setup(s,{emit:m}){const i=s,c=Vue.ref({...i.roleData}),f=Vue.ref(),E=Vue.ref(),{formValidate:p}=V(),{t:h}=VueI18n.useI18n({useScope:"global"}),x=Vue.ref([]),_=Vue.ref(!1),C=Vue.ref(!1),v=Vue.ref(!0),w=Vue.ref({label:"menu_name",children:"children"}),N=e(n.sysNormalDisable),P=Vue.ref({role_name:[{required:!0,message:"角色名称不能为空",trigger:"blur"}],role_key:[{required:!0,message:"角色标志不能为空",trigger:"blur"}],list_order:[{required:!0,message:"角色顺序不能为空",trigger:"blur"}]});(async()=>{const{data:e,execute:l}=o(r.getEnabledTree),{data:t,execute:a}=o(u.getRoleMenus,{role_id:i.roleData.role_id});await Promise.all([l(),a()]),x.value=e.value,t.value.forEach((e=>{Vue.nextTick((()=>{var l;null==(l=E.value)||l.setChecked(e,!0,!1)}))}))})();const g=()=>{m("closeDialog")};return(e,o)=>(Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElDialog),{modelValue:s.open,"onUpdate:modelValue":o[11]||(o[11]=e=>Vue.isRef(open)?open.value=e:null),"before-close":g,title:s.title,"append-to-body":"",width:"600px"},{footer:Vue.withCtx((()=>[Vue.createElementVNode("div",d,[Vue.createVNode(Vue.unref(ElementPlus.ElButton),{type:"primary",onClick:o[10]||(o[10]=e=>(async e=>{var o,r;if(!(await p(e)))return;const V=null==(o=E.value)?void 0:o.getCheckedKeys(!1),n=[...null==(r=E.value)?void 0:r.getHalfCheckedKeys(),...V];if(c.value.role_id){let e={...c.value};e.menu_ids=n;const{data:a,execute:o}=l(u.edit,e);if(await o(),a.value===t)return;ElementPlus.ElMessage.success(`更新 ${c.value.role_name} 成功`)}else{let e={...c.value};e.menu_ids=n;const{data:l,execute:o}=a(u.add,e);if(await o(),l.value===t)return;ElementPlus.ElMessage.success(`新增 ${c.value.role_name} 成功`)}g()})(f.value))},{default:Vue.withCtx((()=>[Vue.createTextVNode(Vue.toDisplayString(Vue.unref(h)("common.submit")),1)])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElButton),{onClick:g},{default:Vue.withCtx((()=>[Vue.createTextVNode(Vue.toDisplayString(Vue.unref(h)("common.cancel")),1)])),_:1})])])),default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElForm),{ref_key:"roleFormRef",ref:f,model:c.value,rules:P.value,class:"base-form","label-width":"100px"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"角色名称",prop:"role_name"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElInput),{modelValue:c.value.role_name,"onUpdate:modelValue":o[0]||(o[0]=e=>c.value.role_name=e),placeholder:"请输入角色名称"},null,8,["modelValue"])])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"角色标志",prop:"role_key"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElInput),{modelValue:c.value.role_key,"onUpdate:modelValue":o[1]||(o[1]=e=>c.value.role_key=e),maxlength:"20",placeholder:"请输入角色标志"},null,8,["modelValue"])])),_:1})])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"角色排序",prop:"list_order"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElInputNumber),{modelValue:c.value.list_order,"onUpdate:modelValue":o[2]||(o[2]=e=>c.value.list_order=e),min:0,"controls-position":"right"},null,8,["modelValue"])])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:12},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"角色状态"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElRadioGroup),{modelValue:c.value.status,"onUpdate:modelValue":o[3]||(o[3]=e=>c.value.status=e)},{default:Vue.withCtx((()=>[(Vue.openBlock(!0),Vue.createElementBlock(Vue.Fragment,null,Vue.renderList(Vue.unref(N)[Vue.unref(n).sysNormalDisable],(e=>(Vue.openBlock(),Vue.createBlock(Vue.unref(ElementPlus.ElRadio),{key:e.value,label:e.value},{default:Vue.withCtx((()=>[Vue.createTextVNode(Vue.toDisplayString(e.label),1)])),_:2},1032,["label"])))),128))])),_:1},8,["modelValue"])])),_:1})])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:24},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"菜单权限"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:24},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:8},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCheckbox),{modelValue:_.value,"onUpdate:modelValue":o[4]||(o[4]=e=>_.value=e),onChange:o[5]||(o[5]=e=>(e=>{const l=x.value;for(let u=0;u<l.length;u++)E.value.store.nodesMap[l[u].id].expanded=e})(_.value))},{default:Vue.withCtx((()=>[Vue.createTextVNode(" 展开/折叠 ")])),_:1},8,["modelValue"])])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:8},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCheckbox),{modelValue:C.value,"onUpdate:modelValue":o[6]||(o[6]=e=>C.value=e),onChange:o[7]||(o[7]=e=>(e=>{const l=x.value;E.value.setCheckedNodes(e?l:[])})(C.value))},{default:Vue.withCtx((()=>[Vue.createTextVNode(" 全选/全不选 ")])),_:1},8,["modelValue"])])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:8},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCheckbox),{modelValue:v.value,"onUpdate:modelValue":o[8]||(o[8]=e=>v.value=e)},{default:Vue.withCtx((()=>[Vue.createTextVNode(" 父子联动 ")])),_:1},8,["modelValue"])])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:24},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElScrollbar),{"max-height":"200px"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElTree),{ref_key:"menuTreeRef",ref:E,data:x.value,"show-checkbox":"","node-key":"id","check-strictly":!v.value,"empty-text":"加载中，请稍候",props:w.value},null,8,["data","check-strictly","props"])])),_:1})])),_:1})])),_:1})])),_:1})])),_:1})])),_:1})])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElRow),null,{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:24},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElFormItem),{label:"备注",prop:"remark"},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElInput),{modelValue:c.value.remark,"onUpdate:modelValue":o[9]||(o[9]=e=>c.value.remark=e),maxlength:"50",placeholder:"请输入邮箱",type:"textarea"},null,8,["modelValue"])])),_:1})])),_:1})])),_:1})])),_:1},8,["model","rules"])])),_:1},8,["modelValue","title"]))}});export{s as _};