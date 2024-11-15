use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 6、用户和角色关联表  用户N-1角色
// -- ----------------------------
// drop table if exists sys_user_role;
// create table sys_user_role (
//   user_id   bigint(20) not null comment '用户ID',
//   role_id   bigint(20) not null comment '角色ID',
//   primary key(user_id, role_id)
// ) engine=innodb comment = '用户和角色关联表';

#[derive(DeriveIden)]
enum SysUserRole {
    Table,
    UserId,
    RoleId,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_user_post");
    manager
        .create_table(
            Table::create()
                .table(SysUserRole::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysUserRole::UserId).string_len(36).not_null().comment("用户ID"))
                .col(ColumnDef::new(SysUserRole::RoleId).string_len(36).not_null().comment("角色ID"))
                .primary_key(Index::create().col(SysUserRole::UserId).col(SysUserRole::RoleId))
                .comment("用户和角色关联表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_user_post");
    manager.drop_table(Table::drop().table(SysUserRole::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_user_post");
    // -- ----------------------------
    // -- 初始化-用户和角色关联表数据
    // -- ----------------------------
    // insert into sys_user_role values ('1', '1');
    // insert into sys_user_role values ('2', '2');
    let insert = Query::insert()
        .into_table(SysUserRole::Table)
        .columns(
            [
                SysUserRole::UserId,
                SysUserRole::RoleId,
            ])
        .values_panic(["1".into(), "1".into()])
        .values_panic(["2".into(), "2".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}