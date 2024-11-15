use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_role;
// create table sys_role (
//   role_id              bigint(20)      not null auto_increment    comment '角色ID',
//   role_name            varchar(30)     not null                   comment '角色名称',
//   role_key             varchar(100)    not null                   comment '角色权限字符串',
//   role_sort            int(4)          not null                   comment '显示顺序',
//   data_scope           char(1)         default '1'                comment '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
//   menu_check_strictly  tinyint(1)      default 1                  comment '菜单树选择项是否关联显示',
//   dept_check_strictly  tinyint(1)      default 1                  comment '部门树选择项是否关联显示',
//   status               char(1)         not null                   comment '角色状态（0正常 1停用）',
//   del_flag             char(1)         default '0'                comment '删除标志（0代表存在 2代表删除）',
//   create_by            varchar(64)     default ''                 comment '创建者',
//   create_time          datetime                                   comment '创建时间',
//   update_by            varchar(64)     default ''                 comment '更新者',
//   update_time          datetime                                   comment '更新时间',
//   remark               varchar(500)    default null               comment '备注',
//   primary key (role_id)
// ) engine=innodb auto_increment=100 comment = '角色信息表';

#[derive(DeriveIden)]
enum SysRole {
    Table,
    Name,
    Key,
    Sort,
    DataScope,
    MenuCheckStrictly,
    DeptCheckStrictly,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_role");
    manager
        .create_table(
            Table::create()
                .table(SysRole::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysRole::Name).string_len(30).not_null().comment("角色名称"))
                .col(ColumnDef::new(SysRole::Key).string_len(100).not_null().comment("角色权限字符串"))
                .col(ColumnDef::new(SysRole::Sort).integer().not_null().comment("显示顺序"))
                .col(ColumnDef::new(SysRole::DataScope).string_len(1).default("1").comment("数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）"))
                .col(ColumnDef::new(SysRole::MenuCheckStrictly).boolean().default("1").comment("菜单树选择项是否关联显示"))
                .col(ColumnDef::new(SysRole::DeptCheckStrictly).boolean().default("1").comment("部门树选择项是否关联显示"))
                .col(ColumnDef::new(SysRole::Status).string_len(1).not_null().comment("角色状态（0正常 1停用）"))
                .col(ColumnDef::new(SysRole::Remark).string_len(500).default("").comment("备注"))
                .comment("岗位信息表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_role");
    manager.drop_table(Table::drop().table(SysRole::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_role");
    // -- ----------------------------
    // -- 初始化-角色信息表数据
    // -- ----------------------------
    // insert into sys_role values('1', '超级管理员',  'admin',  1, 1, 1, 1, '0', '0', 'admin', sysdate(), '', null, '超级管理员');
    // insert into sys_role values('2', '普通角色',    'common', 2, 2, 1, 1, '0', '0', 'admin', sysdate(), '', null, '普通角色');

    let insert = Query::insert()
        .into_table(SysRole::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysRole::Name.into_iden(),
                SysRole::Key.into_iden(),
                SysRole::Sort.into_iden(),
                SysRole::DataScope.into_iden(),
                SysRole::MenuCheckStrictly.into_iden(),
                SysRole::DeptCheckStrictly.into_iden(),
                SysRole::Status.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysRole::Remark.into_iden(),
            ])
        .values_panic(["1".into(), "超级管理员".into(), "admin".into(), "1".into(), "1".into(), "1".into(), "1".into(), "0".into(), "admin".into(), "超级管理员".into()])
        .values_panic(["2".into(), "普通角色".into(), "common".into(), "2".into(), "2".into(), "1".into(), "1".into(), "0".into(), "admin".into(), "普通角色".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}