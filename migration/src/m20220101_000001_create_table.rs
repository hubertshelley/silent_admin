use sea_orm_migration::prelude::*;
use crate::base_system;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // 创建数据表
        base_system::sys_dept::up(manager).await?;
        base_system::sys_user::up(manager).await?;
        base_system::sys_post::up(manager).await?;
        base_system::sys_role::up(manager).await?;
        base_system::sys_menu::up(manager).await?;
        base_system::sys_user_role::up(manager).await?;
        base_system::sys_role_menu::up(manager).await?;
        base_system::sys_role_dept::up(manager).await?;
        base_system::sys_user_post::up(manager).await?;
        base_system::sys_operate_log::up(manager).await?;
        base_system::sys_dict_type::up(manager).await?;
        base_system::sys_dict_data::up(manager).await?;
        base_system::sys_config::up(manager).await?;
        base_system::sys_login_info_record::up(manager).await?;
        base_system::sys_job::up(manager).await?;
        base_system::sys_job_log::up(manager).await?;
        base_system::sys_notice::up(manager).await?;

        // 创建初始数据
        base_system::sys_dept::init_data(manager).await?;
        base_system::sys_user::init_data(manager).await?;
        base_system::sys_post::init_data(manager).await?;
        base_system::sys_role::init_data(manager).await?;
        base_system::sys_menu::init_data(manager).await?;
        base_system::sys_user_role::init_data(manager).await?;
        base_system::sys_role_menu::init_data(manager).await?;
        base_system::sys_role_dept::init_data(manager).await?;
        base_system::sys_user_post::init_data(manager).await?;
        base_system::sys_operate_log::init_data(manager).await?;
        base_system::sys_dict_type::init_data(manager).await?;
        base_system::sys_dict_data::init_data(manager).await?;
        base_system::sys_config::init_data(manager).await?;
        base_system::sys_login_info_record::init_data(manager).await?;
        base_system::sys_job::init_data(manager).await?;
        base_system::sys_job_log::init_data(manager).await?;
        base_system::sys_notice::init_data(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // 回滚数据表
        base_system::sys_notice::down(manager).await?;
        base_system::sys_job_log::down(manager).await?;
        base_system::sys_job::down(manager).await?;
        base_system::sys_login_info_record::down(manager).await?;
        base_system::sys_config::down(manager).await?;
        base_system::sys_dict_data::down(manager).await?;
        base_system::sys_dict_type::down(manager).await?;
        base_system::sys_operate_log::down(manager).await?;
        base_system::sys_user_post::down(manager).await?;
        base_system::sys_role_dept::down(manager).await?;
        base_system::sys_role_menu::down(manager).await?;
        base_system::sys_user_role::down(manager).await?;
        base_system::sys_menu::down(manager).await?;
        base_system::sys_role::down(manager).await?;
        base_system::sys_post::down(manager).await?;
        base_system::sys_user::down(manager).await?;
        base_system::sys_dept::down(manager).await?;

        Ok(())
    }
}
