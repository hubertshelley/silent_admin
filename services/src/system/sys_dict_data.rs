use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use dto::{
    common::res::{ListData, PageParams},
    system::sys_dict_data::{
        SysDictDataAddReq, SysDictDataDeleteReq, SysDictDataEditReq, SysDictDataSearchReq,
    },
};
// use poem::{error::BadRequest, http::StatusCode, Error, Result};
use common::snowflake::next_id;
use entity::prelude::SysDictData;
use entity::sys_dict_data;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysDictDataSearchReq,
) -> Result<ListData<sys_dict_data::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDictData::find();

    if let Some(x) = req.dict_type {
        if !x.is_empty() {
            s = s.filter(sys_dict_data::Column::Type.eq(x));
        }
    }

    if let Some(x) = req.dict_label {
        if !x.is_empty() {
            s = s.filter(sys_dict_data::Column::Label.eq(x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_dict_data::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dict_data::Column::CreateTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dict_data::Column::CreateTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dict_data::Column::Sort)
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

pub async fn check_dict_data_is_exist<C>(req: SysDictDataAddReq, db: &C) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = SysDictData::find().filter(sys_dict_data::Column::Type.eq(req.dict_type));
    let count1 = s
        .clone()
        .filter(sys_dict_data::Column::Value.eq(req.dict_value))
        .count(db)
        .await?;
    let count2 = s
        .clone()
        .filter(sys_dict_data::Column::Label.eq(req.dict_label))
        .count(db)
        .await?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加
pub async fn add(
    db: &DatabaseConnection,
    add_req: SysDictDataAddReq,
    user_id: String,
) -> Result<String> {
    //  检查字典类型是否存在
    if check_dict_data_is_exist(add_req.clone(), db).await? {
        return Err(anyhow!("字典类型或者字典值或者字典标签已经存在"));
    }

    let uid = next_id()?;
    let user = sys_dict_data::ActiveModel {
        id: Set(uid.clone()),
        label: Set(Some(add_req.dict_label)),
        r#type: Set(Some(add_req.dict_type)),
        value: Set(Some(add_req.dict_value)),
        sort: Set(Some(add_req.dict_sort)),
        is_default: Set(Some(add_req.is_default)),
        create_by: Set(user_id),
        css_class: Set(add_req.css_class),
        list_class: Set(add_req.list_class),
        status: Set(Some(add_req.status)),
        remark: Set(add_req.remark),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDictData::insert(user).exec(&txn).await?;
    txn.commit().await?;
    let res = "数据添加成功".to_string();
    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysDictDataDeleteReq) -> Result<String> {
    let mut s = SysDictData::delete_many();

    s = s.filter(sys_dict_data::Column::Id.is_in(delete_req.dict_data_ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的字典类型不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(
    db: &DatabaseConnection,
    req: SysDictDataEditReq,
    user_id: String,
) -> Result<String> {
    let uid = req.dict_data_id;
    let s_s = SysDictData::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_dict_data::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dict_data::ActiveModel {
        label: Set(Some(req.dict_label)),
        r#type: Set(Some(req.dict_type)),
        sort: Set(Some(req.dict_sort)),
        value: Set(Some(req.dict_value)),
        update_by: Set(user_id),
        css_class: Set(req.css_class),
        list_class: Set(req.list_class),
        is_default: Set(Some(req.is_default)),
        status: Set(Some(req.status)),
        remark: Set(req.remark),
        update_time: Set(now),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await; // 这个两种方式一样 都要多查询一次

    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(
    db: &DatabaseConnection,
    search_req: SysDictDataSearchReq,
) -> Result<sys_dict_data::Model> {
    let mut s = SysDictData::find();
    if let Some(x) = search_req.dict_data_id {
        s = s.filter(sys_dict_data::Column::Id.eq(x));
    } else {
        return Err(anyhow!("请输入字典类型id",));
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("字典类型不存在",)),
    };

    Ok(res)
}

pub async fn get_by_type(
    db: &DatabaseConnection,
    dict_type: String,
) -> Result<Vec<sys_dict_data::Model>> {
    let mut s = SysDictData::find().filter(sys_dict_data::Column::Type.eq(dict_type));

    let res = s.order_by_asc(sys_dict_data::Column::Sort).all(db).await?;
    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<sys_dict_data::Model>> {
    let s = SysDictData::find()
        .filter(sys_dict_data::Column::DelFlag.eq(0))
        .filter(sys_dict_data::Column::Status.eq("1"))
        .order_by(sys_dict_data::Column::Id, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}
