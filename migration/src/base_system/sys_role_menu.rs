use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 7、角色和菜单关联表  角色1-N菜单
// -- ----------------------------
// drop table if exists sys_role_menu;
// create table sys_role_menu (
//   role_id   bigint(20) not null comment '角色ID',
//   menu_id   bigint(20) not null comment '菜单ID',
//   primary key(role_id, menu_id)
// ) engine=innodb comment = '角色和菜单关联表';

#[derive(DeriveIden)]
enum SysRoleMenu {
    Table,
    RoleId,
    MenuId,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_role_menu");
    if manager.has_table(SysRoleMenu::Table.to_string()).await? {
        manager
            .drop_table(Table::drop().table(SysRoleMenu::Table).to_owned())
            .await?;
    }
    manager
        .create_table(
            Table::create()
                .table(SysRoleMenu::Table)
                .col(
                    ColumnDef::new(SysRoleMenu::RoleId)
                        .string_len(36)
                        .not_null()
                        .comment("角色ID"),
                )
                .col(
                    ColumnDef::new(SysRoleMenu::MenuId)
                        .string_len(36)
                        .not_null()
                        .comment("菜单ID"),
                )
                .primary_key(
                    Index::create()
                        .name("sys_role_menu_pk")
                        .col(SysRoleMenu::RoleId)
                        .col(SysRoleMenu::MenuId),
                )
                .comment("角色和菜单关联表")
                .to_owned(),
        )
        .await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_role_menu");
    manager
        .drop_table(Table::drop().table(SysRoleMenu::Table).to_owned())
        .await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_role_menu");
    // -- ----------------------------
    // -- 初始化-角色和菜单关联表数据
    // -- ----------------------------
    // insert into sys_role_menu values ('2', '1');
    // insert into sys_role_menu values ('2', '2');
    // insert into sys_role_menu values ('2', '3');
    // insert into sys_role_menu values ('2', '4');
    // insert into sys_role_menu values ('2', '100');
    // insert into sys_role_menu values ('2', '101');
    // insert into sys_role_menu values ('2', '102');
    // insert into sys_role_menu values ('2', '103');
    // insert into sys_role_menu values ('2', '104');
    // insert into sys_role_menu values ('2', '105');
    // insert into sys_role_menu values ('2', '106');
    // insert into sys_role_menu values ('2', '107');
    // insert into sys_role_menu values ('2', '108');
    // insert into sys_role_menu values ('2', '109');
    // insert into sys_role_menu values ('2', '110');
    // insert into sys_role_menu values ('2', '111');
    // insert into sys_role_menu values ('2', '112');
    // insert into sys_role_menu values ('2', '113');
    // insert into sys_role_menu values ('2', '114');
    // insert into sys_role_menu values ('2', '115');
    // insert into sys_role_menu values ('2', '116');
    // insert into sys_role_menu values ('2', '117');
    // insert into sys_role_menu values ('2', '500');
    // insert into sys_role_menu values ('2', '501');
    // insert into sys_role_menu values ('2', '1000');
    // insert into sys_role_menu values ('2', '1001');
    // insert into sys_role_menu values ('2', '1002');
    // insert into sys_role_menu values ('2', '1003');
    // insert into sys_role_menu values ('2', '1004');
    // insert into sys_role_menu values ('2', '1005');
    // insert into sys_role_menu values ('2', '1006');
    // insert into sys_role_menu values ('2', '1007');
    // insert into sys_role_menu values ('2', '1008');
    // insert into sys_role_menu values ('2', '1009');
    // insert into sys_role_menu values ('2', '1010');
    // insert into sys_role_menu values ('2', '1011');
    // insert into sys_role_menu values ('2', '1012');
    // insert into sys_role_menu values ('2', '1013');
    // insert into sys_role_menu values ('2', '1014');
    // insert into sys_role_menu values ('2', '1015');
    // insert into sys_role_menu values ('2', '1016');
    // insert into sys_role_menu values ('2', '1017');
    // insert into sys_role_menu values ('2', '1018');
    // insert into sys_role_menu values ('2', '1019');
    // insert into sys_role_menu values ('2', '1020');
    // insert into sys_role_menu values ('2', '1021');
    // insert into sys_role_menu values ('2', '1022');
    // insert into sys_role_menu values ('2', '1023');
    // insert into sys_role_menu values ('2', '1024');
    // insert into sys_role_menu values ('2', '1025');
    // insert into sys_role_menu values ('2', '1026');
    // insert into sys_role_menu values ('2', '1027');
    // insert into sys_role_menu values ('2', '1028');
    // insert into sys_role_menu values ('2', '1029');
    // insert into sys_role_menu values ('2', '1030');
    // insert into sys_role_menu values ('2', '1031');
    // insert into sys_role_menu values ('2', '1032');
    // insert into sys_role_menu values ('2', '1033');
    // insert into sys_role_menu values ('2', '1034');
    // insert into sys_role_menu values ('2', '1035');
    // insert into sys_role_menu values ('2', '1036');
    // insert into sys_role_menu values ('2', '1037');
    // insert into sys_role_menu values ('2', '1038');
    // insert into sys_role_menu values ('2', '1039');
    // insert into sys_role_menu values ('2', '1040');
    // insert into sys_role_menu values ('2', '1041');
    // insert into sys_role_menu values ('2', '1042');
    // insert into sys_role_menu values ('2', '1043');
    // insert into sys_role_menu values ('2', '1044');
    // insert into sys_role_menu values ('2', '1045');
    // insert into sys_role_menu values ('2', '1046');
    // insert into sys_role_menu values ('2', '1047');
    // insert into sys_role_menu values ('2', '1048');
    // insert into sys_role_menu values ('2', '1049');
    // insert into sys_role_menu values ('2', '1050');
    // insert into sys_role_menu values ('2', '1051');
    // insert into sys_role_menu values ('2', '1052');
    // insert into sys_role_menu values ('2', '1053');
    // insert into sys_role_menu values ('2', '1054');
    // insert into sys_role_menu values ('2', '1055');
    // insert into sys_role_menu values ('2', '1056');
    // insert into sys_role_menu values ('2', '1057');
    // insert into sys_role_menu values ('2', '1058');
    // insert into sys_role_menu values ('2', '1059');
    // insert into sys_role_menu values ('2', '1060');
    let insert = Query::insert()
        .into_table(SysRoleMenu::Table)
        .columns([SysRoleMenu::RoleId, SysRoleMenu::MenuId])
        .values_panic(["2".into(), "1".into()])
        .values_panic(["2".into(), "2".into()])
        .values_panic(["2".into(), "3".into()])
        .values_panic(["2".into(), "4".into()])
        .values_panic(["2".into(), "100".into()])
        .values_panic(["2".into(), "101".into()])
        .values_panic(["2".into(), "102".into()])
        .values_panic(["2".into(), "103".into()])
        .values_panic(["2".into(), "104".into()])
        .values_panic(["2".into(), "105".into()])
        .values_panic(["2".into(), "106".into()])
        .values_panic(["2".into(), "107".into()])
        .values_panic(["2".into(), "108".into()])
        .values_panic(["2".into(), "109".into()])
        .values_panic(["2".into(), "110".into()])
        .values_panic(["2".into(), "111".into()])
        .values_panic(["2".into(), "112".into()])
        .values_panic(["2".into(), "113".into()])
        .values_panic(["2".into(), "114".into()])
        .values_panic(["2".into(), "115".into()])
        .values_panic(["2".into(), "116".into()])
        .values_panic(["2".into(), "117".into()])
        .values_panic(["2".into(), "500".into()])
        .values_panic(["2".into(), "501".into()])
        .values_panic(["2".into(), "1000".into()])
        .values_panic(["2".into(), "1001".into()])
        .values_panic(["2".into(), "1002".into()])
        .values_panic(["2".into(), "1003".into()])
        .values_panic(["2".into(), "1004".into()])
        .values_panic(["2".into(), "1005".into()])
        .values_panic(["2".into(), "1006".into()])
        .values_panic(["2".into(), "1007".into()])
        .values_panic(["2".into(), "1008".into()])
        .values_panic(["2".into(), "1009".into()])
        .values_panic(["2".into(), "1010".into()])
        .values_panic(["2".into(), "1011".into()])
        .values_panic(["2".into(), "1012".into()])
        .values_panic(["2".into(), "1013".into()])
        .values_panic(["2".into(), "1014".into()])
        .values_panic(["2".into(), "1015".into()])
        .values_panic(["2".into(), "1016".into()])
        .values_panic(["2".into(), "1017".into()])
        .values_panic(["2".into(), "1018".into()])
        .values_panic(["2".into(), "1019".into()])
        .values_panic(["2".into(), "1020".into()])
        .values_panic(["2".into(), "1021".into()])
        .values_panic(["2".into(), "1022".into()])
        .values_panic(["2".into(), "1023".into()])
        .values_panic(["2".into(), "1024".into()])
        .values_panic(["2".into(), "1025".into()])
        .values_panic(["2".into(), "1026".into()])
        .values_panic(["2".into(), "1027".into()])
        .values_panic(["2".into(), "1028".into()])
        .values_panic(["2".into(), "1029".into()])
        .values_panic(["2".into(), "1030".into()])
        .values_panic(["2".into(), "1031".into()])
        .values_panic(["2".into(), "1032".into()])
        .values_panic(["2".into(), "1033".into()])
        .values_panic(["2".into(), "1034".into()])
        .values_panic(["2".into(), "1035".into()])
        .values_panic(["2".into(), "1036".into()])
        .values_panic(["2".into(), "1037".into()])
        .values_panic(["2".into(), "1038".into()])
        .values_panic(["2".into(), "1039".into()])
        .values_panic(["2".into(), "1040".into()])
        .values_panic(["2".into(), "1041".into()])
        .values_panic(["2".into(), "1042".into()])
        .values_panic(["2".into(), "1043".into()])
        .values_panic(["2".into(), "1044".into()])
        .values_panic(["2".into(), "1045".into()])
        .values_panic(["2".into(), "1046".into()])
        .values_panic(["2".into(), "1047".into()])
        .values_panic(["2".into(), "1048".into()])
        .values_panic(["2".into(), "1049".into()])
        .values_panic(["2".into(), "1050".into()])
        .values_panic(["2".into(), "1051".into()])
        .values_panic(["2".into(), "1052".into()])
        .values_panic(["2".into(), "1053".into()])
        .values_panic(["2".into(), "1054".into()])
        .values_panic(["2".into(), "1055".into()])
        .values_panic(["2".into(), "1056".into()])
        .values_panic(["2".into(), "1057".into()])
        .values_panic(["2".into(), "1058".into()])
        .values_panic(["2".into(), "1059".into()])
        .values_panic(["2".into(), "1060".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}
