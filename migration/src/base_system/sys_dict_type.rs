use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_dict_type;
// create table sys_dict_type
// (
//   dict_id          bigint(20)      not null auto_increment    comment '字典主键',
//   dict_name        varchar(100)    default ''                 comment '字典名称',
//   dict_type        varchar(100)    default ''                 comment '字典类型',
//   status           char(1)         default '0'                comment '状态（0正常 1停用）',
//   create_by        varchar(64)     default ''                 comment '创建者',
//   create_time      datetime                                   comment '创建时间',
//   update_by        varchar(64)     default ''                 comment '更新者',
//   update_time      datetime                                   comment '更新时间',
//   remark           varchar(500)    default null               comment '备注',
//   primary key (dict_id),
//   unique (dict_type)
// ) engine=innodb auto_increment=100 comment = '字典类型表';

#[derive(DeriveIden)]
enum SysDictType {
    Table,
    Name,
    Type,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_dict_type");
    manager
        .create_table(
            Table::create()
                .table(SysDictType::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysDictType::Name).string_len(100).default("").comment("字典名称"))
                .col(ColumnDef::new(SysDictType::Type).string_len(100).default("").comment("字典类型"))
                .col(ColumnDef::new(SysDictType::Status).string_len(1).default("0").comment("帐号状态（0正常 1停用）"))
                .col(ColumnDef::new(SysDictType::Remark).string().default(Value::String(None)).comment("备注"))
                .index(Index::create().col(SysDictType::Type).unique())
                .comment("字典类型表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_dict_type");
    manager.drop_table(Table::drop().table(SysDictType::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_dict_type");
    // -- ----------------------------
    // -- 初始化-用户信息表数据
    // -- ----------------------------
    // insert into sys_dict_type values(1,  '用户性别', 'sys_user_sex',        '0', 'admin', sysdate(), '', null, '用户性别列表');
    // insert into sys_dict_type values(2,  '菜单状态', 'sys_show_hide',       '0', 'admin', sysdate(), '', null, '菜单状态列表');
    // insert into sys_dict_type values(3,  '系统开关', 'sys_normal_disable',  '0', 'admin', sysdate(), '', null, '系统开关列表');
    // insert into sys_dict_type values(4,  '任务状态', 'sys_job_status',      '0', 'admin', sysdate(), '', null, '任务状态列表');
    // insert into sys_dict_type values(5,  '任务分组', 'sys_job_group',       '0', 'admin', sysdate(), '', null, '任务分组列表');
    // insert into sys_dict_type values(6,  '系统是否', 'sys_yes_no',          '0', 'admin', sysdate(), '', null, '系统是否列表');
    // insert into sys_dict_type values(7,  '通知类型', 'sys_notice_type',     '0', 'admin', sysdate(), '', null, '通知类型列表');
    // insert into sys_dict_type values(8,  '通知状态', 'sys_notice_status',   '0', 'admin', sysdate(), '', null, '通知状态列表');
    // insert into sys_dict_type values(9,  '操作类型', 'sys_oper_type',       '0', 'admin', sysdate(), '', null, '操作类型列表');
    // insert into sys_dict_type values(10, '系统状态', 'sys_common_status',   '0', 'admin', sysdate(), '', null, '登录状态列表');
    let insert = Query::insert()
        .into_table(SysDictType::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysDictType::Name.into_iden(),
                SysDictType::Type.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysDictType::Remark.into_iden(),
            ])
        .values_panic(["1".into(), "用户性别".into(), "sys_user_sex".into(), "admin".into(), "用户性别列表".into()])
        .values_panic(["2".into(), "菜单状态".into(), "sys_show_hide".into(), "admin".into(), "菜单状态列表".into()])
        .values_panic(["3".into(), "系统开关".into(), "sys_normal_disable".into(), "admin".into(), "系统开关列表".into()])
        .values_panic(["4".into(), "任务状态".into(), "sys_job_status".into(), "admin".into(), "任务状态列表".into()])
        .values_panic(["5".into(), "任务分组".into(), "sys_job_group".into(), "admin".into(), "任务分组列表".into()])
        .values_panic(["6".into(), "系统是否".into(), "sys_yes_no".into(), "admin".into(), "系统是否列表".into()])
        .values_panic(["7".into(), "通知类型".into(), "sys_notice_type".into(), "admin".into(), "通知类型列表".into()])
        .values_panic(["8".into(), "通知状态".into(), "sys_notice_status".into(), "admin".into(), "通知状态列表".into()])
        .values_panic(["9".into(), "操作类型".into(), "sys_oper_type".into(), "admin".into(), "操作类型列表".into()])
        .values_panic(["10".into(), "系统状态".into(), "sys_common_status".into(), "admin".into(), "登录状态列表".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}