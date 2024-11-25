use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use common::snowflake::next_id;
use dto::{
    common::res::{ListData, PageParams},
    system::sys_dict_type::{
        SysDictTypeAddReq, SysDictTypeDeleteReq, SysDictTypeEditReq, SysDictTypeSearchReq,
    },
};
use entity::prelude::{SysDictData, SysDictType};
use entity::{sys_dict_data, sys_dict_type};
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysDictTypeSearchReq,
) -> Result<ListData<sys_dict_type::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDictType::find();

    if let Some(x) = req.dict_type {
        s = s.filter(sys_dict_type::Column::Type.contains(x));
    }

    if let Some(x) = req.dict_name {
        s = s.filter(sys_dict_type::Column::Name.contains(x));
    }
    if let Some(x) = req.status {
        s = s.filter(sys_dict_type::Column::Status.eq(x));
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dict_type::Column::CreateTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dict_type::Column::CreateTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dict_type::Column::Id)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_dict_type_is_exist<C>(dict_type: &str, db: &C) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut s = SysDictType::find();
    s = s.filter(sys_dict_type::Column::Type.eq(dict_type));
    let count = s.count(db).await?;
    Ok(count > 0)
}

/// add 添加
pub async fn add(
    db: &DatabaseConnection,
    req: SysDictTypeAddReq,
    user_id: String,
) -> Result<String> {
    //  检查字典类型是否存在
    if check_dict_type_is_exist(&req.dict_type, db).await? {
        return Err(anyhow!("字典类型已存在"));
    }
    let uid = next_id()?;
    let dict_type = sys_dict_type::ActiveModel {
        id: Set(uid.clone()),
        name: Set(Some(req.dict_name)),
        r#type: Set(Some(req.dict_type)),
        status: Set(Some(req.status)),
        remark: Set(req.remark),
        create_by: Set(user_id),
        ..Default::default()
    };
    SysDictType::insert(dict_type).exec(db).await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysDictTypeDeleteReq) -> Result<String> {
    // let count = SysDictType::find()
    //     .select_only()
    //     .column(sys_dict_type::Column::DictTypeId)
    //     .column(sys_dict_data::Column::DictType)
    //     .join_rev(
    //         JoinType::InnerJoin,
    //         sys_dict_data::Entity::belongs_to(sys_dict_type::Entity)
    //             .from(sys_dict_data::Column::DictType)
    //             .to(sys_dict_type::Column::DictType)
    //             .into(),
    //     )
    //     .filter(sys_dict_type::Column::DictTypeId.is_in(delete_req.dict_type_ids.
    // clone()))     .all(db)
    //     .await?;
    let count = SysDictData::find()
        .filter(
            Condition::any().add(
                sys_dict_data::Column::Type.in_subquery(
                    Query::select()
                        .column(sys_dict_type::Column::Type)
                        .from(sys_dict_type::Entity)
                        .and_where(
                            Expr::col(sys_dict_type::Column::Id)
                                .is_in(delete_req.dict_type_ids.clone()),
                        )
                        .to_owned(),
                ),
            ),
        )
        .count(db)
        .await?;
    if count > 0 {
        return Err(anyhow!("存在关联数据，不能删除,请先删除关联字典数据"));
    }
    let txn = db.begin().await?;
    let mut s = SysDictType::delete_many();

    s = s.filter(sys_dict_type::Column::Id.is_in(delete_req.dict_type_ids));

    // 开始删除
    let d = s.exec(db).await?;
    txn.commit().await?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(anyhow!("你要删除的字典类型不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(
    db: &DatabaseConnection,
    req: SysDictTypeEditReq,
    user_id: String,
) -> Result<String> {
    sys_dict_type::Entity::update_many()
        .col_expr(sys_dict_type::Column::Name, Expr::value(req.dict_name))
        .col_expr(sys_dict_type::Column::Type, Expr::value(req.dict_type))
        .col_expr(sys_dict_type::Column::Status, Expr::value(req.status))
        .col_expr(sys_dict_type::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_dict_type::Column::UpdateBy, Expr::value(user_id))
        .col_expr(
            sys_dict_type::Column::UpdateTime,
            Expr::value(Local::now().naive_local()),
        )
        .filter(sys_dict_type::Column::Id.eq(req.dict_type_id))
        .exec(db)
        .await?;
    Ok("数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(
    db: &DatabaseConnection,
    req: SysDictTypeSearchReq,
) -> Result<sys_dict_type::Model> {
    let mut s = SysDictType::find().filter(sys_dict_type::Column::DelFlag.eq(1));
    //
    if let Some(x) = req.dict_type_id {
        s = s.filter(sys_dict_type::Column::Id.eq(x));
    } else {
        return Err(anyhow!("请求参数错误,请输入Id"));
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<sys_dict_type::Model>> {
    let s = SysDictType::find()
        .filter(sys_dict_type::Column::DelFlag.eq(0))
        .filter(sys_dict_type::Column::Status.eq("0"))
        .order_by(sys_dict_type::Column::Id, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}
