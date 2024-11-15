use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_user;
// create table sys_user (
//   user_id           bigint(20)      not null auto_increment    comment '用户ID',
//   dept_id           bigint(20)      default null               comment '部门ID',
//   user_name         varchar(30)     not null                   comment '用户账号',
//   nick_name         varchar(30)     not null                   comment '用户昵称',
//   user_type         varchar(2)      default '00'               comment '用户类型（00系统用户）',
//   email             varchar(50)     default ''                 comment '用户邮箱',
//   phonenumber       varchar(11)     default ''                 comment '手机号码',
//   sex               char(1)         default '0'                comment '用户性别（0男 1女 2未知）',
//   avatar            varchar(100)    default ''                 comment '头像地址',
//   password          varchar(100)    default ''                 comment '密码',
//   status            char(1)         default '0'                comment '帐号状态（0正常 1停用）',
//   del_flag          char(1)         default '0'                comment '删除标志（0代表存在 2代表删除）',
//   login_ip          varchar(128)    default ''                 comment '最后登录IP',
//   login_date        datetime                                   comment '最后登录时间',
//   create_by         varchar(64)     default ''                 comment '创建者',
//   create_time       datetime                                   comment '创建时间',
//   update_by         varchar(64)     default ''                 comment '更新者',
//   update_time       datetime                                   comment '更新时间',
//   remark            varchar(500)    default null               comment '备注',
//   primary key (user_id)
// ) engine=innodb auto_increment=100 comment = '用户信息表';

#[derive(DeriveIden)]
enum SysUser {
    Table,
    DeptId,
    UserName,
    NickName,
    UserType,
    Email,
    PhoneNumber,
    Sex,
    Avatar,
    Password,
    Status,
    LoginIp,
    LoginDate,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_user");
    manager
        .create_table(
            Table::create()
                .table(SysUser::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysUser::DeptId).string_len(36).default(Value::String(None)).comment("部门ID"))
                .col(ColumnDef::new(SysUser::UserName).string_len(50).not_null().comment("用户账号"))
                .col(ColumnDef::new(SysUser::NickName).string_len(50).not_null().comment("用户昵称"))
                .col(ColumnDef::new(SysUser::UserType).string_len(2).default("00").comment("用户类型（00系统用户）"))
                .col(ColumnDef::new(SysUser::Email).string_len(50).default("").comment("用户邮箱"))
                .col(ColumnDef::new(SysUser::PhoneNumber).string_len(16).default("").comment("手机号码"))
                .col(ColumnDef::new(SysUser::Sex).string_len(1).default("0").comment("用户性别（0男 1女 2未知）"))
                .col(ColumnDef::new(SysUser::Avatar).string_len(100).default("").comment("头像地址"))
                .col(ColumnDef::new(SysUser::Password).string_len(100).default("").comment("密码"))
                .col(ColumnDef::new(SysUser::Status).string_len(1).default("0").comment("帐号状态（0正常 1停用）"))
                .col(ColumnDef::new(SysUser::LoginIp).string_len(36).default("").comment("最后登录IP"))
                .col(ColumnDef::new(SysUser::LoginDate).date_time().default(Value::String(None)).comment("最后登录时间"))
                .col(ColumnDef::new(SysUser::Remark).string().default(Value::String(None)).comment("备注"))
                .comment("用户信息表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_user");
    manager.drop_table(Table::drop().table(SysUser::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_user");
    // -- ----------------------------
    // -- 初始化-用户信息表数据
    // -- ----------------------------
    // insert into sys_user values(1,  103, 'admin', '若依', '00', 'ry@163.com', '15888888888', '1', '', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2', '0', '0', '127.0.0.1', sysdate(), 'admin', sysdate(), '', null, '管理员');
    // insert into sys_user values(2,  105, 'ry',    '若依', '00', 'ry@qq.com',  '15666666666', '1', '', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2', '0', '0', '127.0.0.1', sysdate(), 'admin', sysdate(), '', null, '测试员');
    let insert = Query::insert()
        .into_table(SysUser::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysUser::DeptId.into_iden(),
                SysUser::UserName.into_iden(),
                SysUser::NickName.into_iden(),
                SysUser::UserType.into_iden(),
                SysUser::Email.into_iden(),
                SysUser::PhoneNumber.into_iden(),
                SysUser::Sex.into_iden(),
                SysUser::Avatar.into_iden(),
                SysUser::Password.into_iden(),
                BaseModel::CreateBy.into_iden(),
            ])
        .values_panic(["1".into(), "103".into(), "admin".into(), "若依".into(), "00".into(), "ry@163.com".into(), "15888888888".into(), "1".into(), "".into(), "$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2".into(), "admin".into()])
        .values_panic(["2".into(), "105".into(), "ry".into(), "若依".into(), "00".into(), "ry@qq.com".into(), "15666666666".into(), "1".into(), "".into(), "$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2".into(), "admin".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}