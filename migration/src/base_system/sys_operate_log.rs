use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 10、操作日志记录
// -- ----------------------------
// drop table if exists sys_oper_log;
// create table sys_oper_log (
//   oper_id           bigint(20)      not null auto_increment    comment '日志主键',
//   title             varchar(50)     default ''                 comment '模块标题',
//   business_type     int(2)          default 0                  comment '业务类型（0其它 1新增 2修改 3删除）',
//   method            varchar(200)    default ''                 comment '方法名称',
//   request_method    varchar(10)     default ''                 comment '请求方式',
//   operator_type     int(1)          default 0                  comment '操作类别（0其它 1后台用户 2手机端用户）',
//   oper_name         varchar(50)     default ''                 comment '操作人员',
//   dept_name         varchar(50)     default ''                 comment '部门名称',
//   oper_url          varchar(255)    default ''                 comment '请求URL',
//   oper_ip           varchar(128)    default ''                 comment '主机地址',
//   oper_location     varchar(255)    default ''                 comment '操作地点',
//   oper_param        varchar(2000)   default ''                 comment '请求参数',
//   json_result       varchar(2000)   default ''                 comment '返回参数',
//   status            int(1)          default 0                  comment '操作状态（0正常 1异常）',
//   error_msg         varchar(2000)   default ''                 comment '错误消息',
//   oper_time         datetime                                   comment '操作时间',
//   cost_time         bigint(20)      default 0                  comment '消耗时间',
//   primary key (oper_id),
//   key idx_sys_oper_log_bt (business_type),
//   key idx_sys_oper_log_s  (status),
//   key idx_sys_oper_log_ot (oper_time)
// ) engine=innodb auto_increment=100 comment = '操作日志记录';

#[derive(DeriveIden)]
enum SysOperateLog {
    Table,
    Id,
    Title,
    BusinessType,
    Method,
    RequestMethod,
    OperatorType,
    OperatorName,
    DeptName,
    OperateUrl,
    OperateIp,
    OperateLocation,
    OperateParam,
    JsonResult,
    Status,
    ErrorMsg,
    OperateTime,
    CostTime,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_operate_log");
    manager
        .create_table(
            Table::create()
                .table(SysOperateLog::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysOperateLog::Id).string_len(36).primary_key().comment("日志主键"))
                .col(ColumnDef::new(SysOperateLog::Title).string_len(50).default("").comment("模块标题"))
                .col(ColumnDef::new(SysOperateLog::BusinessType).integer().default(0).comment("业务类型（0其它 1新增 2修改 3删除）"))
                .col(ColumnDef::new(SysOperateLog::Method).string_len(200).default("").comment("方法名称"))
                .col(ColumnDef::new(SysOperateLog::RequestMethod).string_len(10).default("").comment("请求方式"))
                .col(ColumnDef::new(SysOperateLog::OperatorType).integer().default(0).comment("操作类别（0其它 1后台用户 2手机端用户）"))
                .col(ColumnDef::new(SysOperateLog::OperatorName).string_len(50).default("").comment("操作人员"))
                .col(ColumnDef::new(SysOperateLog::DeptName).string_len(50).default("").comment("部门名称"))
                .col(ColumnDef::new(SysOperateLog::OperateUrl).string_len(255).default("").comment("请求URL"))
                .col(ColumnDef::new(SysOperateLog::OperateIp).string_len(128).default("").comment("主机地址"))
                .col(ColumnDef::new(SysOperateLog::OperateLocation).string_len(255).default("").comment("操作地点"))
                .col(ColumnDef::new(SysOperateLog::OperateParam).string_len(2000).default("").comment("请求参数"))
                .col(ColumnDef::new(SysOperateLog::JsonResult).json().comment("返回参数"))
                .col(ColumnDef::new(SysOperateLog::Status).integer().default(0).comment("操作状态（0正常 1异常）"))
                .col(ColumnDef::new(SysOperateLog::ErrorMsg).string_len(2000).default("").comment("错误消息"))
                .col(ColumnDef::new(SysOperateLog::OperateTime).date_time().default(Expr::current_timestamp()).comment("操作时间"))
                .col(ColumnDef::new(SysOperateLog::CostTime).integer().default(0).comment("消耗时间"))
                .index(Index::create().name("idx_sys_operate_log_bt").col(SysOperateLog::BusinessType))
                .index(Index::create().name("idx_sys_operate_log_s").col(SysOperateLog::Status))
                .index(Index::create().name("idx_sys_operate_log_ot").col(SysOperateLog::OperateTime))
                .comment("操作日志记录").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_operate_log");
    manager.drop_table(Table::drop().table(SysOperateLog::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(_manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    Ok(())
}