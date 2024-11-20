use silent::{Request, Result};

use services::{service_utils::jwt::Claims, test};
use dto::{common::res::ListData, db_conn, test::prelude::TestDataScopeModel, DB};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<TestDataScopeModel>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = req.get_config()?;
    let res = test::test_data_scope::get_sort_list(db, page_params, req, &user.id).await;
    res.map_err(|e| e.into())
}
/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = test::test_data_scope::add(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

pub async fn delete(mut req: Request) -> Result<String> {
    let req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = test::test_data_scope::delete(db, req).await;
    res.map_err(|e| e.into())
}
