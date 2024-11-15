use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_dept;
// create table sys_dept (
//   dept_id           bigint(20)      not null auto_increment    comment '部门id',
//   parent_id         bigint(20)      default 0                  comment '父部门id',
//   ancestors         varchar(50)     default ''                 comment '祖级列表',
//   dept_name         varchar(30)     default ''                 comment '部门名称',
//   order_num         int(4)          default 0                  comment '显示顺序',
//   leader            varchar(20)     default null               comment '负责人',
//   phone             varchar(11)     default null               comment '联系电话',
//   email             varchar(50)     default null               comment '邮箱',
//   status            char(1)         default '0'                comment '部门状态（0正常 1停用）',
//   del_flag          char(1)         default '0'                comment '删除标志（0代表存在 2代表删除）',
//   create_by         varchar(64)     default ''                 comment '创建者',
//   create_time 	    datetime                                   comment '创建时间',
//   update_by         varchar(64)     default ''                 comment '更新者',
//   update_time       datetime                                   comment '更新时间',
//   primary key (dept_id)
// ) engine=innodb auto_increment=200 comment = '部门表';

#[derive(DeriveIden)]
enum SysDept {
    Table,
    ParentId,
    Ancestors,
    DeptName,
    OrderNum,
    Leader,
    Phone,
    Email,
    Status,
}
pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_dept");
    manager
        .create_table(
            Table::create()
                .table(SysDept::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysDept::ParentId).string_len(36).comment("父部门id"))
                .col(ColumnDef::new(SysDept::Ancestors).string().comment("祖级列表"))
                .col(ColumnDef::new(SysDept::DeptName).string_len(30).comment("部门名称"))
                .col(ColumnDef::new(SysDept::OrderNum).integer().comment("显示顺序"))
                .col(ColumnDef::new(SysDept::Leader).string_len(36).comment("负责人"))
                .col(ColumnDef::new(SysDept::Phone).string_len(16).comment("联系电话"))
                .col(ColumnDef::new(SysDept::Email).string_len(50).comment("邮箱"))
                .col(ColumnDef::new(SysDept::Status).string_len(1).default("0").comment("部门状态（0正常 1停用）"))
                .index(Index::create().name("idx_ancestor").col(SysDept::Ancestors).index_type(IndexType::BTree))
                .comment("部门表")
                .to_owned(),
        )
        .await
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_dept");
    manager
        .drop_table(Table::drop().table(SysDept::Table).to_owned())
        .await
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_dept");
    // insert into sys_dept values(100,  0,   '0',          '若依科技',   0, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(101,  100, '0,100',      '深圳总公司', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(102,  100, '0,100',      '长沙分公司', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(103,  101, '0,100,101',  '研发部门',   1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(104,  101, '0,100,101',  '市场部门',   2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(105,  101, '0,100,101',  '测试部门',   3, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(106,  101, '0,100,101',  '财务部门',   4, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(107,  101, '0,100,101',  '运维部门',   5, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(108,  102, '0,100,102',  '市场部门',   1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    // insert into sys_dept values(109,  102, '0,100,102',  '财务部门',   2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', sysdate(), '', null);
    let insert = Query::insert()
        .into_table(SysDept::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysDept::ParentId.into_iden(),
                SysDept::Ancestors.into_iden(),
                SysDept::DeptName.into_iden(),
                SysDept::OrderNum.into_iden(),
                SysDept::Leader.into_iden(),
                SysDept::Phone.into_iden(),
                SysDept::Email.into_iden(),
                SysDept::Status.into_iden(),
                BaseModel::CreateBy.into_iden(),
            ])
        .values_panic(["100".into(), "0".into(), "0".into(), "若依科技".into(), 0.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["101".into(), "100".into(), "0,100".into(), "深圳总公司".into(), 1.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["102".into(), "100".into(), "0,100".into(), "长沙分公司".into(), 2.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["103".into(), "101".into(), "0,100,101".into(), "研发部门".into(), 1.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["104".into(), "101".into(), "0,100,101".into(), "市场部门".into(), 2.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["105".into(), "101".into(), "0,100,101".into(), "测试部门".into(), 3.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["106".into(), "101".into(), "0,100,101".into(), "财务部门".into(), 4.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["107".into(), "101".into(), "0,100,101".into(), "运维部门".into(), 5.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["108".into(), "102".into(), "0,100,102".into(), "市场部门".into(), 1.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .values_panic(["109".into(), "102".into(), "0,100,102".into(), "财务部门".into(), 2.into(), "若依".into(), "15888888888".into(), "ry@qq.com".into(), "0".into(), "admin".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}