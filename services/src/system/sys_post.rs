use anyhow::anyhow;
use chrono::{Local, NaiveDateTime};
use common::snowflake::next_id;
use dto::common::res::{ListData, PageParams};
use dto::system::sys_post::{
    SysPostAddReq, SysPostDeleteReq, SysPostEditReq, SysPostResp, SysPostSearchReq,
};
use entity::prelude::{SysPost, SysUserPost};
use entity::{sys_post, sys_user_post};
use sea_orm::{
    sea_query::Expr, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    Order, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Set, TransactionTrait,
};
use silent::Result;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysPostSearchReq,
) -> Result<ListData<sys_post::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysPost::find();

    if let Some(x) = req.code {
        if !x.is_empty() {
            s = s.filter(sys_post::Column::Code.contains(&x));
        }
    }

    if let Some(x) = req.name {
        if !x.is_empty() {
            s = s.filter(sys_post::Column::Name.contains(&x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_post::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| anyhow!("{:?}", e))?;
            s = s.filter(sys_post::Column::CreateTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| anyhow!("{:?}", e))?;
            s = s.filter(sys_post::Column::CreateTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(|e| anyhow!("{:?}", e))?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_post::Column::Id)
        .paginate(db, page_per_size);
    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;

    let res = ListData {
        total,
        total_pages,
        list,
        page_num,
    };
    Ok(res)
}

pub async fn check_data_is_exist(
    post_code: String,
    post_name: String,
    db: &DatabaseConnection,
) -> Result<bool> {
    let s1 = SysPost::find().filter(sys_post::Column::Code.eq(post_code));
    let s2 = SysPost::find().filter(sys_post::Column::Name.eq(post_name));
    let count1 = s1.count(db).await.map_err(|e| anyhow!("{:?}", e))?;
    let count2 = s2.count(db).await.map_err(|e| anyhow!("{:?}", e))?;
    Ok(count1 > 0 || count2 > 0)
}

pub async fn eidt_check_data_is_exist(
    post_id: String,
    post_code: String,
    post_name: String,
    db: &DatabaseConnection,
) -> Result<bool> {
    let count1 = SysPost::find()
        .filter(sys_post::Column::Code.eq(post_code))
        .filter(sys_post::Column::Id.ne(post_id.clone()))
        .count(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    let count2 = SysPost::find()
        .filter(sys_post::Column::Name.eq(post_name))
        .filter(sys_post::Column::Id.ne(post_id))
        .count(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加

pub async fn add(db: &DatabaseConnection, req: SysPostAddReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(req.clone().code, req.clone().name, db).await? {
        return Err(anyhow!("数据已存在").into());
    }

    let uid = next_id()?;
    let user = sys_post::ActiveModel {
        id: Set(uid.clone()),
        code: Set(req.code),
        sort: Set(req.sort),
        name: Set(req.name),
        status: Set(req.status),
        remark: Set(Some(req.remark.unwrap_or_default())),
        create_by: Set(user_id),
        ..Default::default()
    };
    let txn = db.begin().await.map_err(|e| anyhow!("{:?}", e))?;
    SysPost::insert(user)
        .exec(&txn)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    txn.commit().await.map_err(|e| anyhow!("{:?}", e))?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysPostDeleteReq) -> Result<String> {
    let mut s = SysPost::delete_many();

    s = s.filter(sys_post::Column::Id.is_in(delete_req.post_ids));

    // 开始删除
    let d = s.exec(db).await.map_err(|e| anyhow!("{:?}", e))?;

    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在").into()),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: SysPostEditReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if eidt_check_data_is_exist(req.id.clone(), req.code.clone(), req.name.clone(), db).await? {
        return Err(anyhow!("数据已存在").into());
    }
    sys_post::Entity::update_many()
        .col_expr(sys_post::Column::Code, Expr::value(req.code))
        .col_expr(sys_post::Column::Name, Expr::value(req.name))
        .col_expr(sys_post::Column::Sort, Expr::value(req.sort))
        .col_expr(sys_post::Column::Status, Expr::value(req.status))
        .col_expr(sys_post::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_post::Column::UpdateBy, Expr::value(user_id))
        .col_expr(
            sys_post::Column::UpdateTime,
            Expr::value(Local::now().naive_local()),
        )
        .filter(sys_post::Column::Id.eq(req.id))
        .exec(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    // 更新
    Ok("用户数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(
    db: &DatabaseConnection,
    search_req: SysPostSearchReq,
) -> Result<SysPostResp> {
    let mut s = SysPost::find();
    s = s.filter(sys_post::Column::DelFlag.eq(0));
    //
    if let Some(x) = search_req.id {
        s = s.filter(sys_post::Column::Id.eq(x));
    } else {
        return Err(anyhow!("请求参数错误").into());
    }

    let res = match s
        .into_model::<SysPostResp>()
        .one(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?
    {
        Some(m) => m,
        None => return Err(anyhow!("数据不存在").into()),
    };

    // let result: Resp =
    // serde_json::from_value(serde_json::json!(res))?;
    // //这种数据转换效率不知道怎么样

    Ok(res)
}

pub async fn get_post_ids_by_user_id(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Vec<String>> {
    let s = SysUserPost::find()
        .filter(sys_user_post::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;

    let mut res = Vec::new();

    for x in s {
        res.push(x.post_id);
    }

    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<SysPostResp>> {
    let s = SysPost::find()
        .filter(sys_post::Column::DelFlag.eq(0))
        .filter(sys_post::Column::Status.eq("0"))
        .order_by(sys_post::Column::Id, Order::Asc)
        .into_model::<SysPostResp>()
        .all(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(s)
}

pub async fn delete_post_by_user_id<C>(db: &C, user_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    if user_ids.is_empty() {
        return Ok(());
    }
    println!(
        "delete_post_by_user_id user_ids:{:?}",
        SysUserPost::delete_many()
            .filter(sys_user_post::Column::UserId.is_in(user_ids.clone()))
            .build(DbBackend::MySql)
            .to_string()
    );
    SysUserPost::delete_many()
        .filter(sys_user_post::Column::UserId.is_in(user_ids))
        .exec(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}

pub async fn add_post_by_user_id<C>(db: &C, user_id: &str, post: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut inser_data: Vec<sys_user_post::ActiveModel> = Vec::new();
    for x in post {
        let act = sys_user_post::ActiveModel {
            user_id: Set(user_id.to_string()),
            post_id: Set(x),
        };
        inser_data.push(act);
    }
    SysUserPost::insert_many(inser_data)
        .exec(db)
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}
