use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum SysUserOnline {
    Table,
    Id,
    UserId,
    TokenId,
    TokenExp,
    LoginTime,
    UserName,
    DeptName,
    Net,
    IpAddr,
    LoginLocation,
    Device,
    Browser,
    Os,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_user_online");
    if manager.has_table(SysUserOnline::Table.to_string()).await? {
        manager
            .drop_table(Table::drop().table(SysUserOnline::Table).to_owned())
            .await?;
    }
    manager
        .create_table(
            Table::create()
                .table(SysUserOnline::Table)
                .col(
                    ColumnDef::new(SysUserOnline::Id)
                        .string_len(36)
                        .not_null()
                        .primary_key()
                        .comment("主键"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::UserId)
                        .string_len(36)
                        .not_null()
                        .comment("用户ID"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::TokenId)
                        .string_len(36)
                        .not_null()
                        .comment("TokenID"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::TokenExp)
                        .integer()
                        .not_null()
                        .comment("Token过期时间"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::LoginTime)
                        .timestamp()
                        .not_null()
                        .comment("登录时间"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::UserName)
                        .string_len(255)
                        .not_null()
                        .comment("用户名"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::DeptName)
                        .string_len(100)
                        .not_null()
                        .comment("部门名称"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::Net)
                        .string_len(100)
                        .not_null()
                        .comment("网络地址"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::IpAddr)
                        .string_len(120)
                        .not_null()
                        .comment("IP地址"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::LoginLocation)
                        .string_len(255)
                        .not_null()
                        .comment("登录地点"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::Device)
                        .string_len(100)
                        .not_null()
                        .comment("设备名称"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::Browser)
                        .string_len(100)
                        .not_null()
                        .comment("浏览器类型"),
                )
                .col(
                    ColumnDef::new(SysUserOnline::Os)
                        .string_len(100)
                        .not_null()
                        .comment("操作系统"),
                )
                .comment("在线用户记录表")
                .to_owned(),
        )
        .await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_user_online");
    manager
        .drop_table(Table::drop().table(SysUserOnline::Table).to_owned())
        .await?;
    Ok(())
}

pub(crate) async fn init_data(_manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    Ok(())
}
