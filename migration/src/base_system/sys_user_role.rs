use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 9、用户与岗位关联表  用户1-N岗位
// -- ----------------------------
// drop table if exists sys_user_post;
// create table sys_user_post
// (
//   user_id   bigint(20) not null comment '用户ID',
//   post_id   bigint(20) not null comment '岗位ID',
//   primary key (user_id, post_id)
// ) engine=innodb comment = '用户与岗位关联表';

#[derive(DeriveIden)]
enum SysUserPost {
    Table,
    UserId,
    PostId,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_user_post");
    manager
        .create_table(
            Table::create()
                .table(SysUserPost::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysUserPost::UserId).string_len(36).not_null().comment("用户ID"))
                .col(ColumnDef::new(SysUserPost::PostId).string_len(36).not_null().comment("岗位ID"))
                .primary_key(Index::create().col(SysUserPost::UserId).col(SysUserPost::PostId))
                .comment("用户与岗位关联表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_user_post");
    manager.drop_table(Table::drop().table(SysUserPost::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_user_post");
    // -- ----------------------------
    // -- 初始化-用户与岗位关联表数据
    // -- ----------------------------
    // insert into sys_user_post values ('1', '1');
    // insert into sys_user_post values ('2', '2');
    let insert = Query::insert()
        .into_table(SysUserPost::Table)
        .columns(
            [
                SysUserPost::UserId,
                SysUserPost::PostId,
            ])
        .values_panic(["1".into(), "1".into()])
        .values_panic(["2".into(), "2".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}