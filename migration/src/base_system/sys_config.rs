use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 13、参数配置表
// -- ----------------------------
// drop table if exists sys_config;
// create table sys_config (
//   config_id         int(5)          not null auto_increment    comment '参数主键',
//   config_name       varchar(100)    default ''                 comment '参数名称',
//   config_key        varchar(100)    default ''                 comment '参数键名',
//   config_value      varchar(500)    default ''                 comment '参数键值',
//   config_type       char(1)         default 'N'                comment '系统内置（Y是 N否）',
//   create_by         varchar(64)     default ''                 comment '创建者',
//   create_time       datetime                                   comment '创建时间',
//   update_by         varchar(64)     default ''                 comment '更新者',
//   update_time       datetime                                   comment '更新时间',
//   remark            varchar(500)    default null               comment '备注',
//   primary key (config_id)
// ) engine=innodb auto_increment=100 comment = '参数配置表';

#[derive(DeriveIden)]
enum SysConfig {
    Table,
    Name,
    Key,
    Value,
    Type,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_config");
    manager
        .create_table(
            Table::create()
                .table(SysConfig::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysConfig::Name).string().not_null().default("").comment("参数名称"))
                .col(ColumnDef::new(SysConfig::Key).string().not_null().default("").comment("参数键名"))
                .col(ColumnDef::new(SysConfig::Value).string().not_null().default("").comment("参数键值"))
                .col(ColumnDef::new(SysConfig::Type).char().not_null().default("N").comment("系统内置（Y是 N否）"))
                .col(ColumnDef::new(SysConfig::Remark).string().default("").comment("备注"))
                .comment("参数配置表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_config");
    manager.drop_table(Table::drop().table(SysConfig::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_config");
    // insert into sys_config values(1, '主框架页-默认皮肤样式名称',     'sys.index.skinName',            'skin-blue',     'Y', 'admin', sysdate(), '', null, '蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow' );
    // insert into sys_config values(2, '用户管理-账号初始密码',         'sys.user.initPassword',         '123456',        'Y', 'admin', sysdate(), '', null, '初始化密码 123456' );
    // insert into sys_config values(3, '主框架页-侧边栏主题',           'sys.index.sideTheme',           'theme-dark',    'Y', 'admin', sysdate(), '', null, '深色主题theme-dark，浅色主题theme-light' );
    // insert into sys_config values(4, '账号自助-验证码开关',           'sys.account.captchaEnabled',    'true',          'Y', 'admin', sysdate(), '', null, '是否开启验证码功能（true开启，false关闭）');
    // insert into sys_config values(5, '账号自助-是否开启用户注册功能', 'sys.account.registerUser',      'false',         'Y', 'admin', sysdate(), '', null, '是否开启注册用户功能（true开启，false关闭）');
    // insert into sys_config values(6, '用户登录-黑名单列表',           'sys.login.blackIPList',         '',              'Y', 'admin', sysdate(), '', null, '设置登录IP黑名单限制，多个匹配项以;分隔，支持匹配（*通配、网段）');
    let insert = Query::insert()
        .into_table(SysConfig::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysConfig::Name.into_iden(),
                SysConfig::Key.into_iden(),
                SysConfig::Value.into_iden(),
                SysConfig::Type.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysConfig::Remark.into_iden(),
            ])
        .values_panic(["1".into(), "主框架页-默认皮肤样式名称".into(), "sys.index.skinName".into(), "skin-blue".into(), "Y".into(), "admin".into(), "蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow".into()])
        .values_panic(["2".into(), "用户管理-账号初始密码".into(), "sys.user.initPassword".into(), "123456".into(), "Y".into(), "admin".into(), "初始化密码 123456".into()])
        .values_panic(["3".into(), "主框架页-侧边栏主题".into(), "sys.index.sideTheme".into(), "theme-dark".into(), "Y".into(), "admin".into(), "深色主题theme-dark，浅色主题theme-light".into()])
        .values_panic(["4".into(), "账号自助-验证码开关".into(), "sys.account.captchaEnabled".into(), "true".into(), "Y".into(), "admin".into(), "是否开启验证码功能（true开启，false关闭）".into()])
        .values_panic(["5".into(), "账号自助-是否开启用户注册功能".into(), "sys.account.registerUser".into(), "false".into(), "Y".into(), "admin".into(), "是否开启注册用户功能（true开启，false关闭）".into()])
        .values_panic(["6".into(), "用户登录-黑名单列表".into(), "sys.login.blackIPList".into(), "".into(), "Y".into(), "admin".into(), "设置登录IP黑名单限制，多个匹配项以;分隔，支持匹配（*通配、网段）".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}