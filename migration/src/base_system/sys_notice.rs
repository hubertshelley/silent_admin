use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 17、通知公告表
// -- ----------------------------
// drop table if exists sys_notice;
// create table sys_notice (
//   notice_id         int(4)          not null auto_increment    comment '公告ID',
//   notice_title      varchar(50)     not null                   comment '公告标题',
//   notice_type       char(1)         not null                   comment '公告类型（1通知 2公告）',
//   notice_content    longblob        default null               comment '公告内容',
//   status            char(1)         default '0'                comment '公告状态（0正常 1关闭）',
//   create_by         varchar(64)     default ''                 comment '创建者',
//   create_time       datetime                                   comment '创建时间',
//   update_by         varchar(64)     default ''                 comment '更新者',
//   update_time       datetime                                   comment '更新时间',
//   remark            varchar(255)    default null               comment '备注',
//   primary key (notice_id)
// ) engine=innodb auto_increment=10 comment = '通知公告表';

#[derive(DeriveIden)]
enum SysNotice {
    Table,
    Title,
    Type,
    Content,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_notice");
    manager
        .create_table(
            Table::create()
                .table(SysNotice::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysNotice::Title).string_len(50).not_null().comment("公告标题"))
                .col(ColumnDef::new(SysNotice::Type).string_len(1).not_null().comment("公告类型（1通知 2公告）"))
                .col(ColumnDef::new(SysNotice::Content).text().default(Value::String(None)).comment("公告内容"))
                .col(ColumnDef::new(SysNotice::Status).string_len(1).default("0").comment("公告状态（0正常 1关闭）"))
                .col(ColumnDef::new(SysNotice::Remark).string().default(Value::String(None)).comment("备注"))
                .comment("通知公告表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_notice");
    manager.drop_table(Table::drop().table(SysNotice::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_notice");
    // -- ----------------------------
    // -- 初始化-公告信息表数据
    // -- ----------------------------
    // insert into sys_notice values('1', '温馨提醒：2018-07-01 若依新版本发布啦', '2', '新版本内容', '0', 'admin', sysdate(), '', null, '管理员');
    // insert into sys_notice values('2', '维护通知：2018-07-01 若依系统凌晨维护', '1', '维护内容',   '0', 'admin', sysdate(), '', null, '管理员');
    let insert = Query::insert()
        .into_table(SysNotice::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysNotice::Title.into_iden(),
                SysNotice::Type.into_iden(),
                SysNotice::Content.into_iden(),
                SysNotice::Status.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysNotice::Remark.into_iden(),
            ])
        .values_panic(["1".into(), "温馨提醒：2018-07-01 若依新版本发布啦".into(), "2".into(), "新版本内容".into(), "0".into(), "admin".into(), "管理员".into()])
        .values_panic(["2".into(), "维护通知：2018-07-01 若依系统凌晨维护".into(), "1".into(), "维护内容".into(), "0".into(), "admin".into(), "管理员".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}