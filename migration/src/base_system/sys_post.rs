use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// drop table if exists sys_post;
// create table sys_post
// (
//   post_id       bigint(20)      not null auto_increment    comment '岗位ID',
//   post_code     varchar(64)     not null                   comment '岗位编码',
//   post_name     varchar(50)     not null                   comment '岗位名称',
//   post_sort     int(4)          not null                   comment '显示顺序',
//   status        char(1)         not null                   comment '状态（0正常 1停用）',
//   create_by     varchar(64)     default ''                 comment '创建者',
//   create_time   datetime                                   comment '创建时间',
//   update_by     varchar(64)     default ''			       comment '更新者',
//   update_time   datetime                                   comment '更新时间',
//   remark        varchar(500)    default null               comment '备注',
//   primary key (post_id)
// ) engine=innodb comment = '岗位信息表';

#[derive(DeriveIden)]
enum SysPost {
    Table,
    Code,
    Name,
    Sort,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_post");
    if manager.has_table(SysPost::Table.to_string()).await? {
        manager
            .drop_table(Table::drop().table(SysPost::Table).to_owned())
            .await?;
    }
    manager
        .create_table(
            Table::create()
                .table(SysPost::Table)
                .append_base_columns()
                .col(
                    ColumnDef::new(SysPost::Code)
                        .string_len(64)
                        .not_null()
                        .comment("岗位编码"),
                )
                .col(
                    ColumnDef::new(SysPost::Name)
                        .string_len(50)
                        .not_null()
                        .comment("岗位名称"),
                )
                .col(
                    ColumnDef::new(SysPost::Sort)
                        .integer()
                        .not_null()
                        .comment("显示顺序"),
                )
                .col(
                    ColumnDef::new(SysPost::Status)
                        .string_len(1)
                        .not_null()
                        .comment("状态（0正常 1停用）"),
                )
                .col(
                    ColumnDef::new(SysPost::Remark)
                        .string_len(500)
                        .comment("备注"),
                )
                .comment("岗位信息表")
                .to_owned(),
        )
        .await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_post");
    manager
        .drop_table(Table::drop().table(SysPost::Table).to_owned())
        .await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_post");
    // -- ----------------------------
    // -- 初始化-岗位信息表数据
    // -- ----------------------------
    // insert into sys_post values(1, 'ceo',  '董事长',    1, '0', 'admin', sysdate(), '', null, '');
    // insert into sys_post values(2, 'se',   '项目经理',  2, '0', 'admin', sysdate(), '', null, '');
    // insert into sys_post values(3, 'hr',   '人力资源',  3, '0', 'admin', sysdate(), '', null, '');
    // insert into sys_post values(4, 'user', '普通员工',  4, '0', 'admin', sysdate(), '', null, '');

    let insert = Query::insert()
        .into_table(SysPost::Table)
        .columns([
            BaseModel::Id.into_iden(),
            SysPost::Code.into_iden(),
            SysPost::Name.into_iden(),
            SysPost::Sort.into_iden(),
            SysPost::Status.into_iden(),
            BaseModel::CreateBy.into_iden(),
        ])
        .values_panic([
            "1".into(),
            "ceo".into(),
            "董事长".into(),
            1.into(),
            "0".into(),
            "admin".into(),
        ])
        .values_panic([
            "2".into(),
            "se".into(),
            "项目经理".into(),
            2.into(),
            "0".into(),
            "admin".into(),
        ])
        .values_panic([
            "3".into(),
            "hr".into(),
            "人力资源".into(),
            3.into(),
            "0".into(),
            "admin".into(),
        ])
        .values_panic([
            "4".into(),
            "user".into(),
            "普通员工".into(),
            4.into(),
            "0".into(),
            "admin".into(),
        ])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}
