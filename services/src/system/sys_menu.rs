use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use common::snowflake::next_id;
use dto::{
    common::res::{ListData, PageParams},
    system::sys_menu::{
        LogCacheEditReq, MenuRelated, MenuResp, Meta, SysMenuAddReq, SysMenuEditReq,
        SysMenuSearchReq, SysMenuTree, SysMenuTreeAll, UserMenu,
    },
};
use entity::prelude::SysMenu;
use entity::sys_menu;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, ModelTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysMenuSearchReq,
) -> Result<ListData<sys_menu::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(u32::MAX as u64);
    //  生成查询条件
    let mut s = SysMenu::find();

    if let Some(x) = req.id {
        if !x.is_empty() {
            s = s.filter(sys_menu::Column::Id.eq(x));
        }
    }
    if let Some(x) = req.pid {
        if !x.is_empty() {
            s = s.filter(sys_menu::Column::ParentId.eq(x));
        }
    }

    if let Some(x) = req.menu_name {
        if !x.is_empty() {
            s = s.filter(sys_menu::Column::Name.contains(&x));
        }
    }
    // if let Some(x) = req.method {
    //     if !x.is_empty() {
    //         s = s.filter(sys_menu::Column::Method.eq(x));
    //     }
    // }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_menu::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.menu_type {
        if !x.is_empty() {
            s = s.filter(sys_menu::Column::MenuType.eq(x));
        }
    }
    if let Some(x) = req.menu_types {
        if !x.is_empty() {
            let y: Vec<&str> = x.split(',').collect();
            s = s.filter(sys_menu::Column::MenuType.is_in(y));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_menu::Column::CreateTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_menu::Column::CreateTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_menu::Column::OrderNum)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        list,
        total,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_router_is_exist_update(
    db: &DatabaseConnection,
    req: SysMenuEditReq,
) -> Result<bool> {
    let s1 = SysMenu::find()
        .filter(sys_menu::Column::Path.eq(req.path.clone()))
        .filter(sys_menu::Column::ParentId.eq(req.parent_id.clone()))
        .filter(sys_menu::Column::MenuType.ne("F"))
        .filter(sys_menu::Column::Id.ne(req.id.clone()));
    let count1 = s1.count(db).await?;
    let s2 = SysMenu::find()
        .filter(sys_menu::Column::Name.eq(req.name.clone()))
        .filter(sys_menu::Column::ParentId.eq(req.parent_id.clone()))
        .filter(sys_menu::Column::MenuType.ne("F"))
        .filter(sys_menu::Column::Id.ne(req.id.clone()));
    let count2 = s2.count(db).await?;
    Ok(count1 > 0 || count2 > 0)
}

pub async fn check_router_is_exist_add<C>(db: &C, req: SysMenuAddReq) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s1 = SysMenu::find()
        .filter(sys_menu::Column::Path.eq(req.path.clone()))
        .filter(sys_menu::Column::ParentId.eq(req.parent_id.clone()))
        .filter(sys_menu::Column::MenuType.ne("F"));
    let count1 = s1.count(db).await?;
    let s2 = SysMenu::find()
        .filter(sys_menu::Column::Name.eq(req.clone().name))
        .filter(sys_menu::Column::ParentId.eq(req.parent_id.clone()));
    let count2 = s2.count(db).await?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: SysMenuAddReq) -> Result<String> {
    //  检查数据是否存在
    if check_router_is_exist_add(db, req.clone()).await? {
        return Err(anyhow!("路径或者名称重复"));
    }
    let uid = next_id()?;
    let active_model = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        parent_id: Set(Some(req.parent_id)),
        name: Set(req.name),
        icon: Set(req.icon),
        remark: Set(req.remark),
        menu_type: Set(Some(req.menu_type)),
        query: Set(req.query),
        // api: Set(req.api),
        // method: Set(req.method.unwrap_or_default()),
        order_num: Set(Some(req.order_num)),
        status: Set(Some(req.status)),
        visible: Set(Some(req.visible)),
        path: Set(req.path),
        component: Set(req.component),
        // data_scope: Set(req.data_scope),
        // data_cache_method: Set(req.data_cache_method),
        // log_method: Set(req.log_method),
        is_frame: Set(Some(req.is_frame)),
        is_cache: Set(Some(req.is_cache)),
        // i18n: Set(req.i18n),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysMenu::insert(active_model).exec(&txn).await?;
    txn.commit().await?;
    let res = format!("{} 添加成功", &uid);
    // // 添加api到全局
    // service_utils::ApiUtils::add_api(
    //     db,
    //     &uid,
    //     &reqq.api,
    //     &reqq.menu_name,
    //     &reqq.data_cache_method,
    //     &reqq.log_method,
    // )
    // .await;
    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<String> {
    match SysMenu::find()
        .filter(sys_menu::Column::ParentId.eq(id))
        .count(db)
        .await?
    {
        0 => {
            let s = SysMenu::find()
                .filter(sys_menu::Column::Id.eq(id))
                .one(db)
                .await?;
            let txn = db.begin().await?;
            if let Some(m) = s {
                // service_utils::ApiUtils::remove_api(&m.api).await;
                m.delete(db).await?;
            }
            txn.commit().await?;
            Ok("菜单删除成功".to_string())
        }
        _ => Err(anyhow!("请先删除子菜单")),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: SysMenuEditReq) -> Result<String> {
    //  检查数据是否存在
    if check_router_is_exist_update(db, req.clone()).await? {
        return Err(anyhow!("路径或者名称重复"));
    }
    let uid = req.id;
    let s_s = match SysMenu::find_by_id(uid.clone()).one(db).await? {
        Some(s) => s,
        None => return Err(anyhow!("菜单不存在")),
    };
    // let s_y = s_s.clone();
    let s_r: sys_menu::ActiveModel = s_s.into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        parent_id: Set(Some(req.parent_id)),
        name: Set(req.name.clone()),
        icon: Set(req.icon),
        remark: Set(req.remark),
        // api: Set(req.api),
        // method: Set(req.method.unwrap_or_default()),
        query: Set(req.query),
        menu_type: Set(Some(req.menu_type)),
        order_num: Set(Some(req.order_num)),
        status: Set(Some(req.status)),
        visible: Set(Some(req.visible)),
        path: Set(Some(req.path)),
        component: Set(req.component),
        // data_scope: Set(req.data_scope),
        is_frame: Set(Some(req.is_frame)),
        is_cache: Set(Some(req.is_cache)),
        // log_method: Set(req.log_method),
        // data_cache_method: Set(req.data_cache_method),
        // i18n: Set(req.i18n),
        update_time: Set(now),
        ..s_r
    };
    // 更新
    let _up_model = act.update(db).await?; // 这个两种方式一样 都要多查询一次

    // tokio::spawn(async move {
    //     let _db = db.clone();
    //     // service_utils::ApiUtils::remove_api(&s_y.api).await;
    //     // service_utils::ApiUtils::add_api(
    //     //     &db,
    //     //     &up_model.id,
    //     //     &up_model.api,
    //     //     &up_model.menu_name,
    //     //     &up_model.data_cache_method,
    //     //     &up_model.log_method,
    //     // )
    //     // .await;
    //     // super::sys_role_api::update_api(
    //     //     db,
    //     //     (&s_y.api, &s_y.method),
    //     //     (&up_model.api, &up_model.method),
    //     // )
    //     // .await
    //     // .expect("更新api失败");
    // });

    let res = format!("{} 修改成功", req.name);
    Ok(res)
}

/// 更新日志和缓存方法
pub async fn update_log_cache_method(
    db: &DatabaseConnection,
    req: LogCacheEditReq,
) -> Result<String> {
    sys_menu::Entity::update_many()
        // .col_expr(sys_menu::Column::LogMethod, Expr::value(req.log_method))
        // .col_expr(
        //     sys_menu::Column::DataCacheMethod,
        //     Expr::value(req.data_cache_method),
        // )
        .col_expr(
            sys_menu::Column::CreateTime,
            Expr::value(Local::now().naive_local()),
        )
        .filter(sys_menu::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok("数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(db: &DatabaseConnection, id: String) -> Result<MenuResp> {
    let mut s = SysMenu::find();
    s = s
        .filter(sys_menu::Column::DelFlag.eq(0))
        .filter(sys_menu::Column::Id.eq(id));

    let res = match s.into_model::<MenuResp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("数据不存在")),
    };

    Ok(res)
}

/// 获取全部菜单 或者 除开按键api级别的外的路由
/// is_router 是否是菜单路由，用于前端生成路由
/// is_only_api 仅获取按键，api级别的路由
/// 不能同时为true
/// 同时false 为获取全部路由
/// is_only_enabled 获取启用的路由，false 为全部路由
pub async fn get_menus<C>(
    db: &C,
    is_router: bool,
    is_only_api: bool,
    is_only_enabled: bool,
) -> Result<Vec<MenuResp>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut s = SysMenu::find().filter(sys_menu::Column::DelFlag.eq(0));
    if is_router {
        s = s.filter(sys_menu::Column::MenuType.ne("F"));
    };
    if is_only_api {
        s = s.filter(sys_menu::Column::MenuType.eq("F"));
    };
    if is_only_enabled {
        s = s.filter(sys_menu::Column::Status.eq("0"));
    };

    let res = s
        .order_by(sys_menu::Column::OrderNum, Order::Asc)
        .into_model::<MenuResp>()
        .all(db)
        .await?;
    Ok(res)
}

/// get_all_router_tree 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all_router_tree(db: &DatabaseConnection) -> Result<Vec<SysMenuTree>> {
    let menus = get_menus(db, true, false, true).await?;
    let menu_data = self::get_menu_data(menus);
    let menu_tree = self::get_menu_tree(menu_data, "0".to_string());

    Ok(menu_tree)
}

/// get_all_menu_tree 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all_enabled_menu_tree(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysMenuSearchReq,
) -> Result<Vec<SysMenuTreeAll>> {
    let menus = get_sort_list(db, page_params, req).await?;
    let menu_data = self::get_menu_data2(menus.list);
    let menu_tree = self::get_menu_tree2(menu_data, "0".to_string());

    Ok(menu_tree)
}

//  获取角色对应的api 和 api id
// 返回结果(Vec<String>, Vec<String>) 为（apis,api_ids）
pub async fn get_role_permissions(
    _db: &DatabaseConnection,
    _role_id: &str,
) -> Result<(Vec<String>, Vec<String>)> {
    // let s = SysMenu::find()
    //     .join_rev(
    //         JoinType::InnerJoin,
    //         SysRoleApi::belongs_to(SysMenu).from(sys_role_api::Column::Api).
    // to(sys_menu::Column::Api).into(),     )
    //     .filter(sys_role_api::Column::RoleId.eq(role_id))
    //     .all(db)
    //     .await?;

    // let s = sys_menu::Entity::find()
    //     .filter(
    //         Condition::any().add(
    //             sys_menu::Column::Api.in_subquery(
    //                 Query::select()
    //                     .column(sys_role_api::Column::Api)
    //                     .from(sys_role_api::Entity)
    //                     .and_where(
    //                         Expr::col(sys_role_api::Column::RoleId).eq(role_id.to_owned().clone()),
    //                     )
    //                     .to_owned(),
    //             ),
    //         ),
    //     )
    //     .all(db)
    //     .await?;

    let apis: Vec<String> = Vec::new();
    let api_ids: Vec<String> = Vec::new();
    // for x in s {
    //     apis.push(x.api.clone());
    //     api_ids.push(x.id.clone());
    // }
    Ok((apis, api_ids))
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_admin_menu_by_role_ids(
    db: &DatabaseConnection,
    role_id: &str,
) -> Result<Vec<SysMenuTree>> {
    let (_menu_apis, _) = self::get_role_permissions(db, role_id).await?;
    //  todo 可能以后加条件判断
    let _router_all = get_menus(db, true, false, false).await?;
    //  生成menus
    let menus: Vec<MenuResp> = Vec::new();
    // for ele in router_all {
    //     if menu_apis.contains(&ele.api) {
    //         menus.push(ele);
    //     }
    // }
    let menu_data = get_menu_data(menus);
    let menu_tree = get_menu_tree(menu_data, "0".to_string());
    Ok(menu_tree)
}

/// 将菜单转换为树
pub fn get_menu_tree(user_menus: Vec<SysMenuTree>, pid: String) -> Vec<SysMenuTree> {
    let mut menu_tree: Vec<SysMenuTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.user_menu.parent_id == pid {
            user_menu.children = Some(get_menu_tree(
                user_menus.clone(),
                user_menu.user_menu.id.clone(),
            ));
            if user_menu.user_menu.menu_type == "M" {
                if user_menu.user_menu.parent_id == "0" {
                    user_menu.user_menu.component = Some("Layout".to_string());
                    if user_menu.user_menu.meta.link.is_none() {
                        user_menu.user_menu.path.insert(0, '/');
                    }
                } else {
                    user_menu.user_menu.component = Some("ParentView".to_string());
                }

                if user_menu.user_menu.is_frame == 1 {
                    user_menu.user_menu.redirect = Some("noRedirect".to_string())
                }

                if user_menu.user_menu.meta.link.is_none() {
                    if user_menu.user_menu.is_frame == 0 {
                        user_menu.user_menu.component = Some("InnerLink".to_string());
                    }
                }

                if !user_menu.children.as_ref().unwrap().is_empty() {
                    user_menu.user_menu.always_show = Some(true);
                }
            }
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
pub fn get_menu_tree2(user_menus: Vec<SysMenuTreeAll>, pid: String) -> Vec<SysMenuTreeAll> {
    let mut menu_tree: Vec<SysMenuTreeAll> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.menu.parent_id.clone().unwrap_or_default().trim() == pid.trim() {
            user_menu.children = Some(get_menu_tree2(
                user_menus.clone(),
                user_menu.menu.id.clone(),
            ));
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
// 多写个方法，少返回点数据
pub fn get_menu_data2(menus: Vec<sys_menu::Model>) -> Vec<SysMenuTreeAll> {
    let mut menu_res: Vec<SysMenuTreeAll> = Vec::new();
    for menu in menus {
        let menu_tree = SysMenuTreeAll {
            menu,
            children: None,
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

//  整理菜单数据
pub fn get_menu_data(menus: Vec<MenuResp>) -> Vec<SysMenuTree> {
    let mut menu_res: Vec<SysMenuTree> = Vec::new();
    for mut menu in menus {
        menu.parent_id = menu.parent_id.trim().to_string();
        let meta = Meta {
            icon: menu.icon.clone(),
            title: menu.name.clone(),
            hidden: menu.visible.clone() != "0",
            link: if menu.path.clone().starts_with("http") {
                Some(menu.path.clone())
            } else {
                None
            },
            no_cache: menu.is_cache.clone() != 0,
        };
        let user_menu = UserMenu {
            meta,
            id: menu.id.clone(),
            parent_id: menu.parent_id.clone(),
            menu_name: menu.name.clone(),
            menu_type: menu.menu_type.clone(),
            path: menu.path.clone(),
            name: menu.path.clone().as_str()[0..1].to_uppercase()
                + &menu.path.clone().as_str()[1..],
            always_show: if menu.is_cache.clone() == 1 && menu.parent_id.clone() == "0" {
                Some(true)
            } else {
                None
            },
            component: menu.component.clone(),
            hidden: menu.visible.clone() != "0",
            is_frame: menu.is_frame,
            ..Default::default()
        };
        let menu_tree = SysMenuTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

/// 获取一个菜单通过数据库相关联的api
pub async fn get_related_api_by_db_name<C>(_db: &C, _api_id: &str) -> Result<Vec<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    // let mut s = sys_menu::Entity::find()
    //     .join_rev(
    //         JoinType::LeftJoin,
    //         sys_api_db::Entity::belongs_to(sys_menu::Entity)
    //             .from(sys_api_db::Column::ApiId)
    //             .to(sys_menu::Column::Id)
    //             .into(),
    //     )
    //     .filter(
    //         Condition::all().add(
    //             sys_api_db::Column::Db.in_subquery(
    //                 Query::select()
    //                     .column(sys_api_db::Column::Db)
    //                     .from(sys_api_db::Entity)
    //                     .and_where(sys_api_db::Column::ApiId.eq(api_id))
    //                     .to_owned(),
    //             ),
    //         ),
    //     );
    // s = s.filter(sys_menu::Column::Method.eq("GET"));
    // let r = s.all(db).await?;
    // let res = r.iter().map(|x| x.api.clone()).collect::<Vec<String>>();
    // Ok(res)
    Ok(Vec::new())
}

pub async fn get_related_api_and_db(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysMenuSearchReq,
) -> Result<ListData<MenuRelated>> {
    let menus = self::get_sort_list(db, page_params, req).await?;
    let res: Vec<MenuRelated> = Vec::new();
    // for item in menus.list {
    //     let (dbs_model, apis) = tokio::join!(
    //         sys_api_db::Entity::find()
    //             .filter(sys_api_db::Column::ApiId.eq(item.id.clone()))
    //             .all(db),
    //         self::get_related_api_by_db_name(db, &item.id),
    //     );
    //
    //     let dbs = match dbs_model {
    //         Ok(v) => v.iter().map(|x| x.db.clone()).collect::<Vec<String>>(),
    //         Err(e) => return Err(anyhow!("{}", e)),
    //     };
    //     let apis = match apis {
    //         Ok(v) => v,
    //         Err(e) => return Err(anyhow!("{}", e)),
    //     };
    //     res.push(MenuRelated {
    //         menu: item,
    //         dbs,
    //         apis,
    //     });
    // }

    Ok(ListData {
        list: res,
        total: menus.total,
        total_pages: menus.total_pages,
        page_num: menus.page_num,
    })
}
