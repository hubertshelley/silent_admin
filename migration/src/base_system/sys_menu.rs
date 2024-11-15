use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_menu;
// create table sys_menu (
//   menu_id           bigint(20)      not null auto_increment    comment "菜单ID",
//   menu_name         varchar(50)     not null                   comment "菜单名称",
//   parent_id         bigint(20)      default 0                  comment "父菜单ID",
//   order_num         int(4)          default 0                  comment "显示顺序",
//   path              varchar(200)    default ""                 comment "路由地址",
//   component         varchar(255)    default null               comment "组件路径",
//   query             varchar(255)    default null               comment "路由参数",
//   route_name        varchar(50)     default ""                 comment "路由名称",
//   is_frame          int(1)          default 1                  comment "是否为外链（0是 1否）",
//   is_cache          int(1)          default 0                  comment "是否缓存（0缓存 1不缓存）",
//   menu_type         char(1)         default ""                 comment "菜单类型（M目录 C菜单 F按钮）",
//   visible           char(1)         default 0                  comment "菜单状态（0显示 1隐藏）",
//   status            char(1)         default 0                  comment "菜单状态（0正常 1停用）",
//   perms             varchar(100)    default null               comment "权限标识",
//   icon              varchar(100)    default "#"                comment "菜单图标",
//   create_by         varchar(64)     default ""                 comment "创建者",
//   create_time       datetime                                   comment "创建时间",
//   update_by         varchar(64)     default ""                 comment "更新者",
//   update_time       datetime                                   comment "更新时间",
//   remark            varchar(500)    default ""                 comment "备注",
//   primary key (menu_id)
// ) engine=innodb auto_increment=2000 comment = "菜单权限表";

#[derive(DeriveIden)]
enum SysMenu {
    Table,
    Name,
    ParentId,
    OrderNum,
    Path,
    Component,
    Query,
    RouteName,
    IsFrame,
    IsCache,
    MenuType,
    Visible,
    Status,
    Perms,
    Icon,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_menu");
    manager
        .create_table(
            Table::create()
                .table(SysMenu::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysMenu::Name).string_len(50).not_null().comment("菜单名称"))
                .col(ColumnDef::new(SysMenu::ParentId).string_len(36).default(0).comment("父菜单ID"))
                .col(ColumnDef::new(SysMenu::OrderNum).integer().default(0).comment("显示顺序"))
                .col(ColumnDef::new(SysMenu::Path).string_len(200).default("").comment("路由地址"))
                .col(ColumnDef::new(SysMenu::Component).string_len(255).default(Value::String(None)).comment("组件路径"))
                .col(ColumnDef::new(SysMenu::Query).string_len(255).default(Value::String(None)).comment("路由参数"))
                .col(ColumnDef::new(SysMenu::RouteName).string_len(50).default("").comment("路由名称"))
                .col(ColumnDef::new(SysMenu::IsFrame).integer().default(1).comment("是否为外链（0是 1否）"))
                .col(ColumnDef::new(SysMenu::IsCache).integer().default(0).comment("是否缓存（0缓存 1不缓存）"))
                .col(ColumnDef::new(SysMenu::MenuType).string_len(1).default("").comment("菜单类型（M目录 C菜单 F按钮）"))
                .col(ColumnDef::new(SysMenu::Visible).string_len(1).default("0").comment("菜单状态（0显示 1隐藏）"))
                .col(ColumnDef::new(SysMenu::Status).string_len(1).default("0").comment("菜单状态（0正常 1停用）"))
                .col(ColumnDef::new(SysMenu::Perms).string_len(100).default(Value::String(None)).comment("权限标识"))
                .col(ColumnDef::new(SysMenu::Icon).string_len(100).default("#").comment("菜单图标"))
                .col(ColumnDef::new(SysMenu::Remark).string_len(500).default("").comment("备注"))
                .comment("菜单权限表")
                .to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_menu");
    manager.drop_table(Table::drop().table(SysMenu::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_menu");
    // -- ----------------------------
    // -- 初始化-菜单信息表数据
    // -- ----------------------------

    let insert = Query::insert()
        .into_table(SysMenu::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysMenu::Name.into_iden(),
                SysMenu::ParentId.into_iden(),
                SysMenu::OrderNum.into_iden(),
                SysMenu::Path.into_iden(),
                SysMenu::Component.into_iden(),
                SysMenu::Query.into_iden(),
                SysMenu::RouteName.into_iden(),
                SysMenu::IsFrame.into_iden(),
                SysMenu::IsCache.into_iden(),
                SysMenu::MenuType.into_iden(),
                SysMenu::Visible.into_iden(),
                SysMenu::Status.into_iden(),
                SysMenu::Perms.into_iden(),
                SysMenu::Icon.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysMenu::Remark.into_iden(),
            ])
        // -- 一级菜单
        // insert into sys_menu values("1".into(), "系统管理".into(), "0".into(), "1".into(), "system".into(),           null.into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "system".into(),   "admin".into(), sysdate().into(), "".into(), null.into(), "系统管理目录");
        // insert into sys_menu values("2".into(), "系统监控".into(), "0".into(), "2".into(), "monitor".into(),          null.into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "monitor".into(),  "admin".into(), sysdate().into(), "".into(), null.into(), "系统监控目录");
        // insert into sys_menu values("3".into(), "系统工具".into(), "0".into(), "3".into(), "tool".into(),             null.into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "tool".into(),     "admin".into(), sysdate().into(), "".into(), null.into(), "系统工具目录");
        // insert into sys_menu values("4".into(), "若依官网".into(), "0".into(), "4".into(), "http://ruoyi.vip".into(), null.into(), "".into(), "".into(), 0.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "guide".into(),    "admin".into(), sysdate().into(), "".into(), null.into(), "若依官网地址");
        .values_panic(["1".into(), "系统管理".into(), "0".into(), "1".into(), "system".into(), Value::String(None).into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "system".into(), "admin".into(), "系统管理目录".into()])
        .values_panic(["2".into(), "系统监控".into(), "0".into(), "2".into(), "monitor".into(), Value::String(None).into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "monitor".into(), "admin".into(), "系统监控目录".into()])
        .values_panic(["3".into(), "系统工具".into(), "0".into(), "3".into(), "tool".into(), Value::String(None).into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "tool".into(), "admin".into(), "系统工具目录".into()])
        .values_panic(["4".into(), "若依官网".into(), "0".into(), "4".into(), "http://ruoyi.vip".into(), Value::String(None).into(), "".into(), "".into(), 0.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "guide".into(), "admin".into(), "若依官网地址".into()])
        // -- 二级菜单
        // insert into sys_menu values("100".into(),  "用户管理".into(), "1".into(),   "1".into(), "user".into(),       "system/user/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:user:list".into(),        "user".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "用户管理菜单");
        // insert into sys_menu values("101".into(),  "角色管理".into(), "1".into(),   "2".into(), "role".into(),       "system/role/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:role:list".into(),        "peoples".into(),       "admin".into(), sysdate().into(), "".into(), null.into(), "角色管理菜单");
        // insert into sys_menu values("102".into(),  "菜单管理".into(), "1".into(),   "3".into(), "menu".into(),       "system/menu/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:menu:list".into(),        "tree-table".into(),    "admin".into(), sysdate().into(), "".into(), null.into(), "菜单管理菜单");
        // insert into sys_menu values("103".into(),  "部门管理".into(), "1".into(),   "4".into(), "dept".into(),       "system/dept/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:dept:list".into(),        "tree".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "部门管理菜单");
        // insert into sys_menu values("104".into(),  "岗位管理".into(), "1".into(),   "5".into(), "post".into(),       "system/post/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:post:list".into(),        "post".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "岗位管理菜单");
        // insert into sys_menu values("105".into(),  "字典管理".into(), "1".into(),   "6".into(), "dict".into(),       "system/dict/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:dict:list".into(),        "dict".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "字典管理菜单");
        // insert into sys_menu values("106".into(),  "参数设置".into(), "1".into(),   "7".into(), "config".into(),     "system/config/index".into(),      "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:config:list".into(),      "edit".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "参数设置菜单");
        // insert into sys_menu values("107".into(),  "通知公告".into(), "1".into(),   "8".into(), "notice".into(),     "system/notice/index".into(),      "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:notice:list".into(),      "message".into(),       "admin".into(), sysdate().into(), "".into(), null.into(), "通知公告菜单");
        // insert into sys_menu values("108".into(),  "日志管理".into(), "1".into(),   "9".into(), "log".into(),        "".into(),                         "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(),                        "log".into(),           "admin".into(), sysdate().into(), "".into(), null.into(), "日志管理菜单");
        // insert into sys_menu values("109".into(),  "在线用户".into(), "2".into(),   "1".into(), "online".into(),     "monitor/online/index".into(),     "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:online:list".into(),     "online".into(),        "admin".into(), sysdate().into(), "".into(), null.into(), "在线用户菜单");
        // insert into sys_menu values("110".into(),  "定时任务".into(), "2".into(),   "2".into(), "job".into(),        "monitor/job/index".into(),        "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:job:list".into(),        "job".into(),           "admin".into(), sysdate().into(), "".into(), null.into(), "定时任务菜单");
        // insert into sys_menu values("111".into(),  "数据监控".into(), "2".into(),   "3".into(), "druid".into(),      "monitor/druid/index".into(),      "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:druid:list".into(),      "druid".into(),         "admin".into(), sysdate().into(), "".into(), null.into(), "数据监控菜单");
        // insert into sys_menu values("112".into(),  "服务监控".into(), "2".into(),   "4".into(), "server".into(),     "monitor/server/index".into(),     "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:server:list".into(),     "server".into(),        "admin".into(), sysdate().into(), "".into(), null.into(), "服务监控菜单");
        // insert into sys_menu values("113".into(),  "缓存监控".into(), "2".into(),   "5".into(), "cache".into(),      "monitor/cache/index".into(),      "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:cache:list".into(),      "redis".into(),         "admin".into(), sysdate().into(), "".into(), null.into(), "缓存监控菜单");
        // insert into sys_menu values("114".into(),  "缓存列表".into(), "2".into(),   "6".into(), "cacheList".into(),  "monitor/cache/list".into(),       "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:cache:list".into(),      "redis-list".into(),    "admin".into(), sysdate().into(), "".into(), null.into(), "缓存列表菜单");
        // insert into sys_menu values("115".into(),  "表单构建".into(), "3".into(),   "1".into(), "build".into(),      "tool/build/index".into(),         "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:build:list".into(),         "build".into(),         "admin".into(), sysdate().into(), "".into(), null.into(), "表单构建菜单");
        // insert into sys_menu values("116".into(),  "代码生成".into(), "3".into(),   "2".into(), "gen".into(),        "tool/gen/index".into(),           "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:gen:list".into(),           "code".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "代码生成菜单");
        // insert into sys_menu values("117".into(),  "系统接口".into(), "3".into(),   "3".into(), "swagger".into(),    "tool/swagger/index".into(),       "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:swagger:list".into(),       "swagger".into(),       "admin".into(), sysdate().into(), "".into(), null.into(), "系统接口菜单");
        .values_panic(["100".into(), "用户管理".into(), "1".into(), "1".into(), "user".into(), "system/user/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:user:list".into(), "user".into(), "admin".into(), "用户管理菜单".into()])
        .values_panic(["101".into(), "角色管理".into(), "1".into(), "2".into(), "role".into(), "system/role/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:role:list".into(), "peoples".into(), "admin".into(), "角色管理菜单".into()])
        .values_panic(["102".into(), "菜单管理".into(), "1".into(), "3".into(), "menu".into(), "system/menu/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:menu:list".into(), "tree-table".into(), "admin".into(), "菜单管理菜单".into()])
        .values_panic(["103".into(), "部门管理".into(), "1".into(), "4".into(), "dept".into(), "system/dept/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:dept:list".into(), "tree".into(), "admin".into(), "部门管理菜单".into()])
        .values_panic(["104".into(), "岗位管理".into(), "1".into(), "5".into(), "post".into(), "system/post/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:post:list".into(), "post".into(), "admin".into(), "岗位管理菜单".into()])
        .values_panic(["105".into(), "字典管理".into(), "1".into(), "6".into(), "dict".into(), "system/dict/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:dict:list".into(), "dict".into(), "admin".into(), "字典管理菜单".into()])
        .values_panic(["106".into(), "参数设置".into(), "1".into(), "7".into(), "config".into(), "system/config/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:config:list".into(), "edit".into(), "admin".into(), "参数设置菜单".into()])
        .values_panic(["107".into(), "通知公告".into(), "1".into(), "8".into(), "notice".into(), "system/notice/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "system:notice:list".into(), "message".into(), "admin".into(), "通知公告菜单".into()])
        .values_panic(["108".into(), "日志管理".into(), "1".into(), "9".into(), "log".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "M".into(), "0".into(), "0".into(), "".into(), "log".into(), "admin".into(), "日志管理菜单".into()])
        .values_panic(["109".into(), "在线用户".into(), "2".into(), "1".into(), "online".into(), "monitor/online/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:online:list".into(), "online".into(), "admin".into(), "在线用户菜单".into()])
        .values_panic(["110".into(), "定时任务".into(), "2".into(), "2".into(), "job".into(), "monitor/job/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:job:list".into(), "job".into(), "admin".into(), "定时任务菜单".into()])
        .values_panic(["111".into(), "数据监控".into(), "2".into(), "3".into(), "druid".into(), "monitor/druid/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:druid:list".into(), "druid".into(), "admin".into(), "数据监控菜单".into()])
        .values_panic(["112".into(), "服务监控".into(), "2".into(), "4".into(), "server".into(), "monitor/server/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:server:list".into(), "server".into(), "admin".into(), "服务监控菜单".into()])
        .values_panic(["113".into(), "缓存监控".into(), "2".into(), "5".into(), "cache".into(), "monitor/cache/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:cache:list".into(), "redis".into(), "admin".into(), "缓存监控菜单".into()])
        .values_panic(["114".into(), "缓存列表".into(), "2".into(), "6".into(), "cacheList".into(), "monitor/cache/list".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:cache:list".into(), "redis-list".into(), "admin".into(), "缓存列表菜单".into()])
        .values_panic(["115".into(), "表单构建".into(), "3".into(), "1".into(), "build".into(), "tool/build/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:build:list".into(), "build".into(), "admin".into(), "表单构建菜单".into()])
        .values_panic(["116".into(), "代码生成".into(), "3".into(), "2".into(), "gen".into(), "tool/gen/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:gen:list".into(), "code".into(), "admin".into(), "代码生成菜单".into()])
        .values_panic(["117".into(), "系统接口".into(), "3".into(), "3".into(), "swagger".into(), "tool/swagger/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "tool:swagger:list".into(), "swagger".into(), "admin".into(), "系统接口菜单".into()])
        // -- 三级菜单
        // insert into sys_menu values("500".into(),  "操作日志".into(), "108".into(), "1".into(), "operlog".into(),    "monitor/operlog/index".into(),    "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:operlog:list".into(),    "form".into(),          "admin".into(), sysdate().into(), "".into(), null.into(), "操作日志菜单");
        // insert into sys_menu values("501".into(),  "登录日志".into(), "108".into(), "2".into(), "logininfor".into(), "monitor/logininfor/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:logininfor:list".into(), "logininfor".into(),    "admin".into(), sysdate().into(), "".into(), null.into(), "登录日志菜单");
        .values_panic(["500".into(), "操作日志".into(), "108".into(), "1".into(), "operlog".into(), "monitor/operlog/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:operlog:list".into(), "form".into(), "admin".into(), "操作日志菜单".into()])
        .values_panic(["501".into(), "登录日志".into(), "108".into(), "2".into(), "logininfor".into(), "monitor/logininfor/index".into(), "".into(), "".into(), 1.into(), 0.into(), "C".into(), "0".into(), "0".into(), "monitor:logininfor:list".into(), "logininfor".into(), "admin".into(), "登录日志菜单".into()])
        // -- 用户管理按钮
        // insert into sys_menu values("1000".into(), "用户查询".into(), "100".into(), "1".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1001".into(), "用户新增".into(), "100".into(), "2".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1002".into(), "用户修改".into(), "100".into(), "3".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1003".into(), "用户删除".into(), "100".into(), "4".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1004".into(), "用户导出".into(), "100".into(), "5".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:export".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1005".into(), "用户导入".into(), "100".into(), "6".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:import".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1006".into(), "重置密码".into(), "100".into(), "7".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:resetPwd".into(),       "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1000".into(), "用户查询".into(), "100".into(), "1".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1001".into(), "用户新增".into(), "100".into(), "2".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1002".into(), "用户修改".into(), "100".into(), "3".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1003".into(), "用户删除".into(), "100".into(), "4".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1004".into(), "用户导出".into(), "100".into(), "5".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:export".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1005".into(), "用户导入".into(), "100".into(), "6".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:import".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1006".into(), "重置密码".into(), "100".into(), "7".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:user:resetPwd".into(), "#".into(), "admin".into(), "".into()])
        // -- 角色管理按钮
        // insert into sys_menu values("1007".into(), "角色查询".into(), "101".into(), "1".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1008".into(), "角色新增".into(), "101".into(), "2".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1009".into(), "角色修改".into(), "101".into(), "3".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1010".into(), "角色删除".into(), "101".into(), "4".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1011".into(), "角色导出".into(), "101".into(), "5".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:export".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1007".into(), "角色查询".into(), "101".into(), "1".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1008".into(), "角色新增".into(), "101".into(), "2".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1009".into(), "角色修改".into(), "101".into(), "3".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1010".into(), "角色删除".into(), "101".into(), "4".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1011".into(), "角色导出".into(), "101".into(), "5".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:role:export".into(), "#".into(), "admin".into(), "".into()])
        // -- 菜单管理按钮
        // insert into sys_menu values("1012".into(), "菜单查询".into(), "102".into(), "1".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1013".into(), "菜单新增".into(), "102".into(), "2".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1014".into(), "菜单修改".into(), "102".into(), "3".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1015".into(), "菜单删除".into(), "102".into(), "4".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1012".into(), "菜单查询".into(), "102".into(), "1".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1013".into(), "菜单新增".into(), "102".into(), "2".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1014".into(), "菜单修改".into(), "102".into(), "3".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1015".into(), "菜单删除".into(), "102".into(), "4".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:menu:remove".into(), "#".into(), "admin".into(), "".into()])
        // -- 部门管理按钮
        // insert into sys_menu values("1016".into(), "部门查询".into(), "103".into(), "1".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1017".into(), "部门新增".into(), "103".into(), "2".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1018".into(), "部门修改".into(), "103".into(), "3".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1019".into(), "部门删除".into(), "103".into(), "4".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1016".into(), "部门查询".into(), "103".into(), "1".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1017".into(), "部门新增".into(), "103".into(), "2".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1018".into(), "部门修改".into(), "103".into(), "3".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1019".into(), "部门删除".into(), "103".into(), "4".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dept:remove".into(), "#".into(), "admin".into(), "".into()])
        // -- 岗位管理按钮
        // insert into sys_menu values("1020".into(), "岗位查询".into(), "104".into(), "1".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1021".into(), "岗位新增".into(), "104".into(), "2".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1022".into(), "岗位修改".into(), "104".into(), "3".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1023".into(), "岗位删除".into(), "104".into(), "4".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1024".into(), "岗位导出".into(), "104".into(), "5".into(),  "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:export".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1020".into(), "岗位查询".into(), "104".into(), "1".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1021".into(), "岗位新增".into(), "104".into(), "2".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1022".into(), "岗位修改".into(), "104".into(), "3".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1023".into(), "岗位删除".into(), "104".into(), "4".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1024".into(), "岗位导出".into(), "104".into(), "5".into(), "".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:post:export".into(), "#".into(), "admin".into(), "".into()])
        // -- 字典管理按钮
        // insert into sys_menu values("1025".into(), "字典查询".into(), "105".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1026".into(), "字典新增".into(), "105".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1027".into(), "字典修改".into(), "105".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1028".into(), "字典删除".into(), "105".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1029".into(), "字典导出".into(), "105".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:export".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1025".into(), "字典查询".into(), "105".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1026".into(), "字典新增".into(), "105".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1027".into(), "字典修改".into(), "105".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1028".into(), "字典删除".into(), "105".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1029".into(), "字典导出".into(), "105".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:dict:export".into(), "#".into(), "admin".into(), "".into()])
        // -- 参数设置按钮
        // insert into sys_menu values("1030".into(), "参数查询".into(), "106".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:query".into(),    "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1031".into(), "参数新增".into(), "106".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:add".into(),      "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1032".into(), "参数修改".into(), "106".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:edit".into(),     "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1033".into(), "参数删除".into(), "106".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:删除".into(),       "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1034".into(), "参数导出".into(), "106".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:导出".into(),       "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1030".into(), "参数查询".into(), "106".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1031".into(), "参数新增".into(), "106".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1032".into(), "参数修改".into(), "106".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1033".into(), "参数删除".into(), "106".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:删除".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1034".into(), "参数导出".into(), "106".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:config:导出".into(), "#".into(), "admin".into(), "".into()])
        // -- 通知公告按钮
        // insert into sys_menu values("1035".into(), "公告查询".into(), "107".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:query".into(),        "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1036".into(), "公告新增".into(), "107".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:add".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1037".into(), "公告修改".into(), "107".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:edit".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1038".into(), "公告删除".into(), "107".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:remove".into(),       "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1035".into(), "公告查询".into(), "107".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1036".into(), "公告新增".into(), "107".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1037".into(), "公告修改".into(), "107".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1038".into(), "公告删除".into(), "107".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "system:notice:remove".into(), "#".into(), "admin".into(), "".into()])
        // -- 操作日志按钮
        // insert into sys_menu values("1039".into(), "操作查询".into(), "500".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:query".into(),      "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1040".into(), "操作删除".into(), "500".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:remove".into(),     "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1041".into(), "日志导出".into(), "500".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:export".into(),     "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1039".into(), "操作查询".into(), "500".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1040".into(), "操作删除".into(), "500".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1041".into(), "日志导出".into(), "500".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:operlog:export".into(), "#".into(), "admin".into(), "".into()])
        // -- 登录日志按钮
        // insert into sys_menu values("1042".into(), "登录查询".into(), "501".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:query".into(),   "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1043".into(), "登录删除".into(), "501".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:remove".into(),  "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1044".into(), "日志导出".into(), "501".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:export".into(),  "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1045".into(), "账户解锁".into(), "501".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:unlock".into(),  "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1042".into(), "登录查询".into(), "501".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1043".into(), "登录删除".into(), "501".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1044".into(), "日志导出".into(), "501".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:export".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1045".into(), "账户解锁".into(), "501".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:logininfor:unlock".into(), "#".into(), "admin".into(), "".into()])
        // -- 在线用户按钮
        // insert into sys_menu values("1046".into(), "在线查询".into(), "109".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:query".into(),       "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1047".into(), "批量强退".into(), "109".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:batchLogout".into(), "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1048".into(), "单条强退".into(), "109".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:forceLogout".into(), "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1046".into(), "在线查询".into(), "109".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1047".into(), "批量强退".into(), "109".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:batchLogout".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1048".into(), "单条强退".into(), "109".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:online:forceLogout".into(), "#".into(), "admin".into(), "".into()])
        // -- 定时任务按钮
        // insert into sys_menu values("1049".into(), "任务查询".into(), "110".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:query".into(),          "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1050".into(), "任务新增".into(), "110".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:add".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1051".into(), "任务修改".into(), "110".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:edit".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1052".into(), "任务删除".into(), "110".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:remove".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1053".into(), "状态修改".into(), "110".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:changeStatus".into(),   "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1054".into(), "任务导出".into(), "110".into(), "6".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:export".into(),         "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1049".into(), "任务查询".into(), "110".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1050".into(), "任务新增".into(), "110".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:add".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1051".into(), "任务修改".into(), "110".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1052".into(), "任务删除".into(), "110".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1053".into(), "状态修改".into(), "110".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:changeStatus".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1054".into(), "任务导出".into(), "110".into(), "6".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "monitor:job:export".into(), "#".into(), "admin".into(), "".into()])
        // -- 代码生成按钮
        // insert into sys_menu values("1055".into(), "生成查询".into(), "116".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:query".into(),             "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1056".into(), "生成修改".into(), "116".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:edit".into(),              "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1057".into(), "生成删除".into(), "116".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:remove".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1058".into(), "导入代码".into(), "116".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:import".into(),            "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1059".into(), "预览代码".into(), "116".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:preview".into(),           "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        // insert into sys_menu values("1060".into(), "生成代码".into(), "116".into(), "6".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:code".into(),              "#".into(), "admin".into(), sysdate().into(), "".into(), null.into(), "");
        .values_panic(["1055".into(), "生成查询".into(), "116".into(), "1".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:query".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1056".into(), "生成修改".into(), "116".into(), "2".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:edit".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1057".into(), "生成删除".into(), "116".into(), "3".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:remove".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1058".into(), "导入代码".into(), "116".into(), "4".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:import".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1059".into(), "预览代码".into(), "116".into(), "5".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:preview".into(), "#".into(), "admin".into(), "".into()])
        .values_panic(["1060".into(), "生成代码".into(), "116".into(), "6".into(), "#".into(), "".into(), "".into(), "".into(), 1.into(), 0.into(), "F".into(), "0".into(), "0".into(), "tool:gen:code".into(), "#".into(), "admin".into(), "".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}