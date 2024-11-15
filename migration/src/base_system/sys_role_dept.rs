use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 8、角色和部门关联表  角色1-N部门
// -- ----------------------------
// drop table if exists sys_role_dept;
// create table sys_role_dept (
//   role_id   bigint(20) not null comment '角色ID',
//   dept_id   bigint(20) not null comment '部门ID',
//   primary key(role_id, dept_id)
// ) engine=innodb comment = '角色和部门关联表';

#[derive(DeriveIden)]
enum SysRoleDept {
    Table,
    RoleId,
    DeptId,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_role_dept");
    manager
        .create_table(
            Table::create()
                .table(SysRoleDept::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysRoleDept::RoleId).string_len(36).not_null().comment("角色ID"))
                .col(ColumnDef::new(SysRoleDept::DeptId).string_len(36).not_null().comment("部门ID"))
                .primary_key(Index::create().col(SysRoleDept::RoleId).col(SysRoleDept::DeptId))
                .comment("角色和部门关联表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_role_dept");
    manager.drop_table(Table::drop().table(SysRoleDept::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_role_dept");
    // -- ----------------------------
    // -- 初始化-角色和部门关联表数据
    // -- ----------------------------
    // insert into sys_role_dept values ('2', '100');
    // insert into sys_role_dept values ('2', '101');
    // insert into sys_role_dept values ('2', '105');
    let insert = Query::insert()
        .into_table(SysRoleDept::Table)
        .columns(
            [
                SysRoleDept::RoleId,
                SysRoleDept::DeptId,
            ])
        .values_panic(["2".into(), "100".into()])
        .values_panic(["2".into(), "101".into()])
        .values_panic(["2".into(), "105".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}