use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 8、角色和部门关联表  角色1-N部门
// -- ----------------------------
// drop table if exists sys_user_dept;
// create table sys_user_dept (
//   role_id   bigint(20) not null comment '角色ID',
//   dept_id   bigint(20) not null comment '部门ID',
//   primary key(role_id, dept_id)
// ) engine=innodb comment = '角色和部门关联表';

#[derive(DeriveIden)]
enum SysUserDept {
    Table,
    UserId,
    DeptId,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_user_dept");
    if manager.has_table(SysUserDept::Table.to_string()).await? {
        manager
            .drop_table(Table::drop().table(SysUserDept::Table).to_owned())
            .await?;
    }
    manager
        .create_table(
            Table::create()
                .table(SysUserDept::Table)
                .col(
                    ColumnDef::new(SysUserDept::UserId)
                        .string_len(36)
                        .not_null()
                        .comment("用户ID"),
                )
                .col(
                    ColumnDef::new(SysUserDept::DeptId)
                        .string_len(36)
                        .not_null()
                        .comment("部门ID"),
                )
                .primary_key(
                    Index::create()
                        .name("sys_user_dept_pk")
                        .col(SysUserDept::UserId)
                        .col(SysUserDept::DeptId),
                )
                .comment("角色和部门关联表")
                .to_owned(),
        )
        .await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_user_dept");
    manager
        .drop_table(Table::drop().table(SysUserDept::Table).to_owned())
        .await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_user_dept");
    // -- ----------------------------
    // -- 初始化-角色和部门关联表数据
    // -- ----------------------------
    // insert into sys_user_dept values ('2', '100');
    // insert into sys_user_dept values ('2', '101');
    // insert into sys_user_dept values ('2', '105');
    let insert = Query::insert()
        .into_table(SysUserDept::Table)
        .columns([SysUserDept::UserId, SysUserDept::DeptId])
        .values_panic(["1".into(), "100".into()])
        .values_panic(["2".into(), "101".into()])
        .values_panic(["2".into(), "105".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}
