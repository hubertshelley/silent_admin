use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 16、定时任务调度日志表
// -- ----------------------------
// drop table if exists sys_job_log;
// create table sys_job_log (
//   job_log_id          bigint(20)     not null auto_increment    comment '任务日志ID',
//   job_name            varchar(64)    not null                   comment '任务名称',
//   job_group           varchar(64)    not null                   comment '任务组名',
//   invoke_target       varchar(500)   not null                   comment '调用目标字符串',
//   job_message         varchar(500)                              comment '日志信息',
//   status              char(1)        default '0'                comment '执行状态（0正常 1失败）',
//   exception_info      varchar(2000)  default ''                 comment '异常信息',
//   create_time         datetime                                  comment '创建时间',
//   primary key (job_log_id)
// ) engine=innodb comment = '定时任务调度日志表';

#[derive(DeriveIden)]
enum SysJobLog {
    Table,
    Id,
    Name,
    Group,
    InvokeTarget,
    JobMessage,
    Status,
    ExceptionInfo,
    CreateTime,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_job_log");
    manager
        .create_table(
            Table::create()
                .table(SysJobLog::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysJobLog::Id).string_len(36).primary_key().comment("日志主键"))
                .col(ColumnDef::new(SysJobLog::Name).string_len(64).not_null().comment("任务名称"))
                .col(ColumnDef::new(SysJobLog::Group).string_len(64).not_null().comment("任务组名"))
                .col(ColumnDef::new(SysJobLog::InvokeTarget).string_len(500).not_null().comment("调用目标字符串"))
                .col(ColumnDef::new(SysJobLog::JobMessage).string_len(500).comment("日志信息"))
                .col(ColumnDef::new(SysJobLog::Status).char_len(1).default("0").comment("执行状态（0正常 1失败）"))
                .col(ColumnDef::new(SysJobLog::ExceptionInfo).string_len(2000).default("").comment("异常信息"))
                .col(ColumnDef::new(SysJobLog::CreateTime).date_time().comment("创建时间"))
                .comment("定时任务调度日志表")
                .to_owned(),
        ).await?;
    Ok(())
}


pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_job_log");
    manager.drop_table(Table::drop().table(SysJobLog::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(_manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    Ok(())
}