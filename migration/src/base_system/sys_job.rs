use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 15、定时任务调度表
// -- ----------------------------
// drop table if exists sys_job;
// create table sys_job (
//   job_id              bigint(20)    not null auto_increment    comment '任务ID',
//   job_name            varchar(64)   default ''                 comment '任务名称',
//   job_group           varchar(64)   default 'DEFAULT'          comment '任务组名',
//   invoke_target       varchar(500)  not null                   comment '调用目标字符串',
//   cron_expression     varchar(255)  default ''                 comment 'cron执行表达式',
//   misfire_policy      varchar(20)   default '3'                comment '计划执行错误策略（1立即执行 2执行一次 3放弃执行）',
//   concurrent          char(1)       default '1'                comment '是否并发执行（0允许 1禁止）',
//   status              char(1)       default '0'                comment '状态（0正常 1暂停）',
//   create_by           varchar(64)   default ''                 comment '创建者',
//   create_time         datetime                                 comment '创建时间',
//   update_by           varchar(64)   default ''                 comment '更新者',
//   update_time         datetime                                 comment '更新时间',
//   remark              varchar(500)  default ''                 comment '备注信息',
//   primary key (job_id, job_name, job_group)
// ) engine=innodb auto_increment=100 comment = '定时任务调度表';

#[derive(DeriveIden)]
enum SysJob {
    Table,
    Name,
    Group,
    InvokeTarget,
    CronExpression,
    MisfirePolicy,
    Concurrent,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_job");
    manager
        .create_table(
            Table::create()
                .table(SysJob::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysJob::Name).string_len(64).default("").comment("任务名称"))
                .col(ColumnDef::new(SysJob::Group).string_len(64).default("DEFAULT").comment("任务组名"))
                .col(ColumnDef::new(SysJob::InvokeTarget).string_len(500).not_null().comment("调用目标字符串"))
                .col(ColumnDef::new(SysJob::CronExpression).string_len(255).default("").comment("cron执行表达式"))
                .col(ColumnDef::new(SysJob::MisfirePolicy).string_len(20).default("3").comment("计划执行错误策略（1立即执行 2执行一次 3放弃执行）"))
                .col(ColumnDef::new(SysJob::Concurrent).char_len(1).default("1").comment("是否并发执行（0允许 1禁止）"))
                .col(ColumnDef::new(SysJob::Status).char_len(1).default("0").comment("状态（0正常 1暂停）"))
                .col(ColumnDef::new(SysJob::Remark).string().default(Value::String(None)).comment("备注"))
                .comment("定时任务调度表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_job");
    manager.drop_table(Table::drop().table(SysJob::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_job");
    // insert into sys_job values(1, '系统默认（无参）', 'DEFAULT', 'ryTask.ryNoParams',        '0/10 * * * * ?', '3', '1', '1', 'admin', sysdate(), '', null, '');
    // insert into sys_job values(2, '系统默认（有参）', 'DEFAULT', 'ryTask.ryParams(\'ry\')',  '0/15 * * * * ?', '3', '1', '1', 'admin', sysdate(), '', null, '');
    // insert into sys_job values(3, '系统默认（多参）', 'DEFAULT', 'ryTask.ryMultipleParams(\'ry\', true, 2000L, 316.50D, 100)',  '0/20 * * * * ?', '3', '1', '1', 'admin', sysdate(), '', null, '');
    let insert = Query::insert()
        .into_table(SysJob::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysJob::Name.into_iden(),
                SysJob::Group.into_iden(),
                SysJob::InvokeTarget.into_iden(),
                SysJob::CronExpression.into_iden(),
                SysJob::MisfirePolicy.into_iden(),
                SysJob::Concurrent.into_iden(),
                SysJob::Status.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysJob::Remark.into_iden(),
            ])
        .values_panic(["1".into(), "系统默认（无参）".into(), "DEFAULT".into(), "ryTask.ryNoParams".into(), "0/10 * * * * ?".into(), "3".into(), "1".into(), "1".into(), "admin".into(), "".into()])
        .values_panic(["2".into(), "系统默认（有参）".into(), "DEFAULT".into(), "ryTask.ryParams('ry')".into(), "0/15 * * * * ?".into(), "3".into(), "1".into(), "1".into(), "admin".into(), "".into()])
        .values_panic(["3".into(), "系统默认（多参）".into(), "DEFAULT".into(), "ryTask.ryMultipleParams('ry', true, 2000L, 316.50D, 100)".into(), "0/20 * * * * ?".into(), "3".into(), "1".into(), "1".into(), "admin".into(), "".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}