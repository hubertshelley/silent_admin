use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 14、系统访问记录
// -- ----------------------------
// drop table if exists sys_logininfor;
// create table sys_logininfor (
//   info_id        bigint(20)     not null auto_increment   comment '访问ID',
//   user_name      varchar(50)    default ''                comment '用户账号',
//   ipaddr         varchar(128)   default ''                comment '登录IP地址',
//   login_location varchar(255)   default ''                comment '登录地点',
//   browser        varchar(50)    default ''                comment '浏览器类型',
//   os             varchar(50)    default ''                comment '操作系统',
//   status         char(1)        default '0'               comment '登录状态（0成功 1失败）',
//   msg            varchar(255)   default ''                comment '提示消息',
//   login_time     datetime                                 comment '访问时间',
//   primary key (info_id),
//   key idx_sys_logininfor_s  (status),
//   key idx_sys_logininfor_lt (login_time)
// ) engine=innodb auto_increment=100 comment = '系统访问记录';

#[derive(DeriveIden)]
enum SysLoginInfoRecord {
    Table,
    Id,
    UserName,
    Ipaddr,
    LoginLocation,
    Browser,
    Os,
    Status,
    Msg,
    LoginTime,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_login_info_record");
    manager
        .create_table(
            Table::create()
                .table(SysLoginInfoRecord::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysLoginInfoRecord::Id).string_len(36).primary_key().comment("日志主键"))
                .col(ColumnDef::new(SysLoginInfoRecord::UserName).string_len(50).default("").comment("用户账号"))
                .col(ColumnDef::new(SysLoginInfoRecord::Ipaddr).string_len(128).default("").comment("登录IP地址"))
                .col(ColumnDef::new(SysLoginInfoRecord::LoginLocation).string_len(255).default("").comment("登录地点"))
                .col(ColumnDef::new(SysLoginInfoRecord::Browser).string_len(50).default("").comment("浏览器类型"))
                .col(ColumnDef::new(SysLoginInfoRecord::Os).string_len(50).default("").comment("操作系统"))
                .col(ColumnDef::new(SysLoginInfoRecord::Status).string_len(1).default("0").comment("登录状态（0成功 1失败）"))
                .col(ColumnDef::new(SysLoginInfoRecord::Msg).string_len(255).default("").comment("提示消息"))
                .col(ColumnDef::new(SysLoginInfoRecord::LoginTime).date_time().default(Expr::current_timestamp()).comment("访问时间"))
                .index(Index::create().name("idx_sys_login_info_record_s").col(SysLoginInfoRecord::Status))
                .index(Index::create().name("idx_sys_login_info_record_lt").col(SysLoginInfoRecord::LoginTime))
                .comment("系统访问记录").to_owned(),
        ).await?;
    Ok(())
}


pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_login_info_record");
    manager.drop_table(Table::drop().table(SysLoginInfoRecord::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(_manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    Ok(())
}