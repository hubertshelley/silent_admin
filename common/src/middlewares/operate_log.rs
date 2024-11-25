use core::time::Duration;
use std::time::Instant;

use anyhow::Result;
use chrono::Local;
use configs::CFG;
use dto::common::ctx::{ReqCtx, UserInfoCtx};
use entity::prelude::SysOperateLog;
use sea_orm::prelude::async_trait;
use sea_orm::{EntityTrait, Set};
use serde_json::json;
// use services::{service_utils::api_utils::ALL_APIS, system::check_user_online};
use crate::snowflake::next_id;
use crate::{get_db_conn, DB};
use entity::sys_operate_log;
use silent::prelude::ResBody;
use silent::{Handler, MiddleWareHandler, Next, Request, Response, Result as SilentResult};

pub struct OperLogMiddleware;

#[async_trait::async_trait]
impl MiddleWareHandler for OperLogMiddleware {
    async fn handle(&self, req: Request, next: &Next) -> SilentResult<Response> {
        let start = Instant::now();
        // 查询ctx
        let req_ctx = req.extensions().get::<ReqCtx>().unwrap().clone();

        let ctx_user = req.extensions().get::<UserInfoCtx>().unwrap().clone();
        let res = next.call(req).await;
        let duration = start.elapsed();
        let mut res_string = "".to_string();
        let mut err_msg = "".to_string();
        let mut status = 200;
        match &res {
            Ok(res) => {
                if let ResBody::Once(body) = res.body() {
                    res_string = String::from_utf8(body.to_vec()).unwrap_or_default();
                }
            }
            Err(e) => {
                err_msg = e.to_string();
                status = e.status().as_u16();
            }
        }
        oper_log_add(req_ctx, ctx_user, res_string, status, err_msg, duration).await;
        res
    }
}

pub async fn oper_log_add(
    ctx: ReqCtx,
    ctx_user: UserInfoCtx,
    res: String,
    status: u16,
    err_msg: String,
    duration: Duration,
) {
    tokio::spawn(async move {
        match oper_log_add_fn(ctx, ctx_user, res, status, err_msg, duration).await {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
}

/// add 添加
pub async fn oper_log_add_fn(
    ctx: ReqCtx,
    ctx_user: UserInfoCtx,
    res: String,
    status: u16,
    err_msg: String,
    duration: Duration,
) -> Result<()> {
    if !CFG.log.enable_oper_log {
        return Ok(());
    }
    let now = Local::now().naive_local();
    let duration_data = duration;
    tokio::spawn(async move {
        match db_log(
            duration_data,
            ctx,
            ctx_user,
            now,
            "".to_string(),
            res,
            status,
            err_msg,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn db_log(
    duration: Duration,
    ctx: ReqCtx,
    ctx_user: UserInfoCtx,
    now: chrono::NaiveDateTime,
    api_name: String,
    res: String,
    status: u16,
    err_msg: String,
) -> Result<()> {
    let d = duration.as_micros() as i32;
    let db = DB.get_or_init(get_db_conn).await;
    // let (_, m) = check_user_online(Some(db), ctx_user.token_id.clone()).await;
    // let user = match m {
    //     Some(x) => x,
    //     None => return Ok(()),
    // };
    let business_type = match ctx.method.as_str() {
        "GET" => 1,    // 查询
        "POST" => 2,   // 新增
        "PUT" => 3,    // 修改
        "DELETE" => 4, // 删除
        _ => 0,        // 其他
    };
    let add_data = sys_operate_log::ActiveModel {
        id: Set(next_id()?),
        title: Set(Some(api_name)),
        business_type: Set(Some(business_type)),
        method: Set(Some("".to_string())),
        request_method: Set(Some(ctx.method)),
        operator_type: Set(Some(1)),
        operator_name: Set(Some(ctx_user.name)),
        dept_name: Set(Some("".to_owned())),
        operate_url: Set(Some(ctx.ori_uri)),
        operate_ip: Set(Some(ctx.remote_addr)),
        operate_location: Set(Some("".to_string())),
        operate_param: Set(Some(if ctx.path_params.len() > 10000 {
            "数据太长不保存".to_string()
        } else {
            ctx.path_params
        })),
        json_result: Set(if res.len() > 65535 {
            Some(json!({"result": "数据太长不保存"}))
        } else {
            Some(
                serde_json::from_str::<serde_json::Value>(&res)
                    .unwrap_or(json!({"result": "数据解析失败"})),
            )
        }),
        status: Set(Some(status as i32)),
        error_msg: Set(Some(err_msg)),
        cost_time: Set(Some(d)),
        operate_time: Set(Some(now)),
    };
    SysOperateLog::insert(add_data)
        .exec(db)
        .await
        .expect("oper_log_add error");
    Ok(())
}
