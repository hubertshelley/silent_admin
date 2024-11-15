use crate::common::{BaseModel, BaseModelExt};
use sea_orm_migration::prelude::*;

// -- ----------------------------
// -- 12、字典数据表
// -- ----------------------------
// drop table if exists sys_dict_data;
// create table sys_dict_data
// (
//   dict_code        bigint(20)      not null auto_increment    comment '字典编码',
//   dict_sort        int(4)          default 0                  comment '字典排序',
//   dict_label       varchar(100)    default ''                 comment '字典标签',
//   dict_value       varchar(100)    default ''                 comment '字典键值',
//   dict_type        varchar(100)    default ''                 comment '字典类型',
//   css_class        varchar(100)    default null               comment '样式属性（其他样式扩展）',
//   list_class       varchar(100)    default null               comment '表格回显样式',
//   is_default       char(1)         default 'N'                comment '是否默认（Y是 N否）',
//   status           char(1)         default '0'                comment '状态（0正常 1停用）',
//   create_by        varchar(64)     default ''                 comment '创建者',
//   create_time      datetime                                   comment '创建时间',
//   update_by        varchar(64)     default ''                 comment '更新者',
//   update_time      datetime                                   comment '更新时间',
//   remark           varchar(500)    default null               comment '备注',
//   primary key (dict_code)
// ) engine=innodb auto_increment=100 comment = '字典数据表';

#[derive(DeriveIden)]
enum SysDictData {
    Table,
    Sort,
    Label,
    Value,
    Type,
    CssClass,
    ListClass,
    IsDefault,
    Status,
    Remark,
}

pub(crate) async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Migrating sys_dict_data");
    manager
        .create_table(
            Table::create()
                .table(SysDictData::Table)
                .if_not_exists()
                .append_base_columns()
                .col(ColumnDef::new(SysDictData::Sort).integer().default(0).comment("字典排序"))
                .col(ColumnDef::new(SysDictData::Label).string_len(100).default("").comment("字典标签"))
                .col(ColumnDef::new(SysDictData::Value).string_len(100).default("").comment("字典键值"))
                .col(ColumnDef::new(SysDictData::Type).string_len(100).default("").comment("字典类型"))
                .col(ColumnDef::new(SysDictData::CssClass).string_len(100).default(Value::String(None)).comment("样式属性（其他样式扩展）"))
                .col(ColumnDef::new(SysDictData::ListClass).string_len(100).default(Value::String(None)).comment("表格回显样式"))
                .col(ColumnDef::new(SysDictData::IsDefault).string_len(1).default("N").comment("是否默认（Y是 N否）"))
                .col(ColumnDef::new(SysDictData::Status).string_len(1).default("0").comment("状态（0正常 1停用）"))
                .col(ColumnDef::new(SysDictData::Remark).string().default(Value::String(None)).comment("备注"))
                .comment("字典数据表").to_owned(),
        ).await?;
    Ok(())
}

pub(crate) async fn down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    println!("Reverting sys_dict_data");
    manager.drop_table(Table::drop().table(SysDictData::Table).to_owned()).await?;
    Ok(())
}

pub(crate) async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    // Replace the sample below with your own data initialization scripts
    println!("Initializing sys_dict_data");
    // insert into sys_dict_data values(1,  1,  '男',       '0',       'sys_user_sex',        '',   '',        'Y', '0', 'admin', sysdate(), '', null, '性别男');
    // insert into sys_dict_data values(2,  2,  '女',       '1',       'sys_user_sex',        '',   '',        'N', '0', 'admin', sysdate(), '', null, '性别女');
    // insert into sys_dict_data values(3,  3,  '未知',     '2',       'sys_user_sex',        '',   '',        'N', '0', 'admin', sysdate(), '', null, '性别未知');
    // insert into sys_dict_data values(4,  1,  '显示',     '0',       'sys_show_hide',       '',   'primary', 'Y', '0', 'admin', sysdate(), '', null, '显示菜单');
    // insert into sys_dict_data values(5,  2,  '隐藏',     '1',       'sys_show_hide',       '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '隐藏菜单');
    // insert into sys_dict_data values(6,  1,  '正常',     '0',       'sys_normal_disable',  '',   'primary', 'Y', '0', 'admin', sysdate(), '', null, '正常状态');
    // insert into sys_dict_data values(7,  2,  '停用',     '1',       'sys_normal_disable',  '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '停用状态');
    // insert into sys_dict_data values(8,  1,  '正常',     '0',       'sys_job_status',      '',   'primary', 'Y', '0', 'admin', sysdate(), '', null, '正常状态');
    // insert into sys_dict_data values(9,  2,  '暂停',     '1',       'sys_job_status',      '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '停用状态');
    // insert into sys_dict_data values(10, 1,  '默认',     'DEFAULT', 'sys_job_group',       '',   '',        'Y', '0', 'admin', sysdate(), '', null, '默认分组');
    // insert into sys_dict_data values(11, 2,  '系统',     'SYSTEM',  'sys_job_group',       '',   '',        'N', '0', 'admin', sysdate(), '', null, '系统分组');
    // insert into sys_dict_data values(12, 1,  '是',       'Y',       'sys_yes_no',          '',   'primary', 'Y', '0', 'admin', sysdate(), '', null, '系统默认是');
    // insert into sys_dict_data values(13, 2,  '否',       'N',       'sys_yes_no',          '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '系统默认否');
    // insert into sys_dict_data values(14, 1,  '通知',     '1',       'sys_notice_type',     '',   'warning', 'Y', '0', 'admin', sysdate(), '', null, '通知');
    // insert into sys_dict_data values(15, 2,  '公告',     '2',       'sys_notice_type',     '',   'success', 'N', '0', 'admin', sysdate(), '', null, '公告');
    // insert into sys_dict_data values(16, 1,  '正常',     '0',       'sys_notice_status',   '',   'primary', 'Y', '0', 'admin', sysdate(), '', null, '正常状态');
    // insert into sys_dict_data values(17, 2,  '关闭',     '1',       'sys_notice_status',   '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '关闭状态');
    // insert into sys_dict_data values(18, 99, '其他',     '0',       'sys_oper_type',       '',   'info',    'N', '0', 'admin', sysdate(), '', null, '其他操作');
    // insert into sys_dict_data values(19, 1,  '新增',     '1',       'sys_oper_type',       '',   'info',    'N', '0', 'admin', sysdate(), '', null, '新增操作');
    // insert into sys_dict_data values(20, 2,  '修改',     '2',       'sys_oper_type',       '',   'info',    'N', '0', 'admin', sysdate(), '', null, '修改操作');
    // insert into sys_dict_data values(21, 3,  '删除',     '3',       'sys_oper_type',       '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '删除操作');
    // insert into sys_dict_data values(22, 4,  '授权',     '4',       'sys_oper_type',       '',   'primary', 'N', '0', 'admin', sysdate(), '', null, '授权操作');
    // insert into sys_dict_data values(23, 5,  '导出',     '5',       'sys_oper_type',       '',   'warning', 'N', '0', 'admin', sysdate(), '', null, '导出操作');
    // insert into sys_dict_data values(24, 6,  '导入',     '6',       'sys_oper_type',       '',   'warning', 'N', '0', 'admin', sysdate(), '', null, '导入操作');
    // insert into sys_dict_data values(25, 7,  '强退',     '7',       'sys_oper_type',       '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '强退操作');
    // insert into sys_dict_data values(26, 8,  '生成代码', '8',       'sys_oper_type',       '',   'warning', 'N', '0', 'admin', sysdate(), '', null, '生成操作');
    // insert into sys_dict_data values(27, 9,  '清空数据', '9',       'sys_oper_type',       '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '清空操作');
    // insert into sys_dict_data values(28, 1,  '成功',     '0',       'sys_common_status',   '',   'primary', 'N', '0', 'admin', sysdate(), '', null, '正常状态');
    // insert into sys_dict_data values(29, 2,  '失败',     '1',       'sys_common_status',   '',   'danger',  'N', '0', 'admin', sysdate(), '', null, '停用状态');
    let insert = Query::insert()
        .into_table(SysDictData::Table)
        .columns(
            [
                BaseModel::Id.into_iden(),
                SysDictData::Sort.into_iden(),
                SysDictData::Label.into_iden(),
                SysDictData::Value.into_iden(),
                SysDictData::Type.into_iden(),
                SysDictData::CssClass.into_iden(),
                SysDictData::ListClass.into_iden(),
                SysDictData::IsDefault.into_iden(),
                SysDictData::Status.into_iden(),
                BaseModel::CreateBy.into_iden(),
                SysDictData::Remark.into_iden(),
            ])
        .values_panic(["1".into(), 1.into(), "男".into(), "0".into(), "sys_user_sex".into(), "".into(), "".into(), "Y".into(), "0".into(), "admin".into(), "性别男".into()])
        .values_panic(["2".into(), 2.into(), "女".into(), "1".into(), "sys_user_sex".into(), "".into(), "".into(), "N".into(), "0".into(), "admin".into(), "性别女".into()])
        .values_panic(["3".into(), 3.into(), "未知".into(), "2".into(), "sys_user_sex".into(), "".into(), "".into(), "N".into(), "0".into(), "admin".into(), "性别未知".into()])
        .values_panic(["4".into(), 1.into(), "显示".into(), "0".into(), "sys_show_hide".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "显示菜单".into()])
        .values_panic(["5".into(), 2.into(), "隐藏".into(), "1".into(), "sys_show_hide".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "隐藏菜单".into()])
        .values_panic(["6".into(), 1.into(), "正常".into(), "0".into(), "sys_normal_disable".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "正常状态".into()])
        .values_panic(["7".into(), 2.into(), "停用".into(), "1".into(), "sys_normal_disable".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "停用状态".into()])
        .values_panic(["8".into(), 1.into(), "正常".into(), "0".into(), "sys_job_status".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "正常状态".into()])
        .values_panic(["9".into(), 2.into(), "暂停".into(), "1".into(), "sys_job_status".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "停用状态".into()])
        .values_panic(["10".into(), 1.into(), "默认".into(), "DEFAULT".into(), "sys_job_group".into(), "".into(), "".into(), "Y".into(), "0".into(), "admin".into(), "默认分组".into()])
        .values_panic(["11".into(), 2.into(), "系统".into(), "SYSTEM".into(), "sys_job_group".into(), "".into(), "".into(), "N".into(), "0".into(), "admin".into(), "系统分组".into()])
        .values_panic(["12".into(), 1.into(), "是".into(), "Y".into(), "sys_yes_no".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "系统默认是".into()])
        .values_panic(["13".into(), 2.into(), "否".into(), "N".into(), "sys_yes_no".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "系统默认否".into()])
        .values_panic(["14".into(), 1.into(), "通知".into(), "1".into(), "sys_notice_type".into(), "".into(), "warning".into(), "Y".into(), "0".into(), "admin".into(), "通知".into()])
        .values_panic(["15".into(), 2.into(), "公告".into(), "2".into(), "sys_notice_type".into(), "".into(), "success".into(), "N".into(), "0".into(), "admin".into(), "公告".into()])
        .values_panic(["16".into(), 1.into(), "正常".into(), "0".into(), "sys_notice_status".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "正常状态".into()])
        .values_panic(["17".into(), 2.into(), "关闭".into(), "1".into(), "sys_notice_status".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "关闭状态".into()])
        .values_panic(["18".into(), 99.into(), "其他".into(), "0".into(), "sys_oper_type".into(), "".into(), "info".into(), "N".into(), "0".into(), "admin".into(), "其他操作".into()])
        .values_panic(["19".into(), 1.into(), "新增".into(), "1".into(), "sys_oper_type".into(), "".into(), "success".into(), "Y".into(), "0".into(), "admin".into(), "新增操作".into()])
        .values_panic(["20".into(), 2.into(), "修改".into(), "2".into(), "sys_oper_type".into(), "".into(), "info".into(), "N".into(), "0".into(), "admin".into(), "修改操作".into()])
        .values_panic(["21".into(), 3.into(), "删除".into(), "3".into(), "sys_oper_type".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "删除操作".into()])
        .values_panic(["22".into(), 4.into(), "授权".into(), "4".into(), "sys_oper_type".into(), "".into(), "warning".into(), "N".into(), "0".into(), "admin".into(), "授权操作".into()])
        .values_panic(["23".into(), 5.into(), "导出".into(), "5".into(), "sys_oper_type".into(), "".into(), "info".into(), "N".into(), "0".into(), "admin".into(), "导出操作".into()])
        .values_panic(["24".into(), 6.into(), "导入".into(), "6".into(), "sys_oper_type".into(), "".into(), "info".into(), "N".into(), "0".into(), "admin".into(), "导入操作".into()])
        .values_panic(["25".into(), 7.into(), "强退".into(), "7".into(), "sys_oper_type".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "强退操作".into()])
        .values_panic(["26".into(), 8.into(), "生成代码".into(), "8".into(), "sys_oper_type".into(), "".into(), "info".into(), "N".into(), "0".into(), "admin".into(), "生成操作".into()])
        .values_panic(["27".into(), 9.into(), "清空数据".into(), "9".into(), "sys_oper_type".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "清空操作".into()])
        .values_panic(["28".into(), 1.into(), "成功".into(), "0".into(), "sys_common_status".into(), "".into(), "primary".into(), "Y".into(), "0".into(), "admin".into(), "成功状态".into()])
        .values_panic(["29".into(), 2.into(), "失败".into(), "1".into(), "sys_common_status".into(), "".into(), "danger".into(), "N".into(), "0".into(), "admin".into(), "失败状态".into()])
        .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
}