use std::ops::Deref;

use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait, Select};
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T>
where
    T: Serialize,
{
    data: Vec<T>,
    page_size: u64,
    page_no: u64,
    total: u64,
}

impl<T> Deref for Pagination<T>
where
    T: Serialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Pagination<T>
where
    T: Serialize,
{
    pub fn new(data: Vec<T>, total: u64, pagination_params: PaginationParams) -> Self {
        let (page_size, page_no) = if pagination_params.no_page() {
            (0, 0)
        } else {
            (pagination_params.page_size(), pagination_params.page_no())
        };
        Self {
            data,
            page_size,
            page_no,
            total,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    page_size: Option<u64>,
    page_no: Option<u64>,
    #[serde(default)]
    no_page: bool,
}

impl PaginationParams {
    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(10)
    }
    pub fn page_no(&self) -> u64 {
        self.page_no.unwrap_or(1) - 1
    }
    pub fn no_page(&self) -> bool {
        self.no_page
    }
    pub fn set_page_size(&mut self, page_size: u64) {
        self.page_size = Some(page_size);
    }
    pub fn set_page_no(&mut self, page_no: u64) {
        self.page_no = Some(page_no);
    }
    pub fn set_no_page(&mut self, no_page: bool) {
        self.no_page = no_page;
    }
    pub fn with_page_size(mut self, page_size: u64) -> Self {
        self.set_page_size(page_size);
        self
    }
    pub fn with_page_no(mut self, page_no: u64) -> Self {
        self.set_page_no(page_no);
        self
    }
    pub fn with_no_page(mut self, no_page: bool) -> Self {
        self.set_no_page(no_page);
        self
    }
    pub async fn paginate<'db, C, M, E>(self, db: &'db C, data: Select<E>) -> Result<Pagination<M>>
    where
        M: Serialize,
        C: ConnectionTrait,
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync + 'db,
    {
        let total = data.clone().count(db).await?;
        let datas = if self.no_page() {
            data.all(db).await?
        } else {
            data.paginate(db, self.page_size())
                .fetch_page(self.page_no())
                .await?
        };
        Ok(Pagination::new(datas, total, self))
    }
    pub async fn into_paginate<'db, C, M, E, RM>(
        self,
        db: &'db C,
        data: Select<E>,
    ) -> Result<Pagination<RM>>
    where
        RM: Serialize,
        M: Into<RM>,
        C: ConnectionTrait,
        E: EntityTrait<Model = M>,
        M: FromQueryResult + Sized + Send + Sync + 'db,
    {
        let total = data.clone().count(db).await?;
        let datas = if self.no_page() {
            data.all(db).await?
        } else {
            data.paginate(db, self.page_size())
                .fetch_page(self.page_no())
                .await?
        };
        Ok(Pagination::new(
            datas.into_iter().map(|m| m.into()).collect(),
            total,
            self,
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Debug, Serialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_pagination_happy_path() {
        // 创建一个包含数据的 Pagination 实例
        let data = vec![
            TestData {
                id: 1,
                name: "Alice".to_string(),
            },
            TestData {
                id: 2,
                name: "Bob".to_string(),
            },
        ];
        let pagination = Pagination {
            data,
            page_size: 10,
            page_no: 1,
            total: 2,
        };

        // 验证 Pagination 实例的各个字段
        assert_eq!(pagination.data.len(), 2);
        assert_eq!(pagination.page_size, 10);
        assert_eq!(pagination.page_no, 1);
        assert_eq!(pagination.total, 2);
        assert_eq!(
            serde_json::to_string(&pagination).unwrap(),
            r#"{"data":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}],"pageSize":10,"pageNo":1,"total":2}"#
        );
    }

    #[test]
    fn test_pagination_empty_data() {
        // 创建一个空数据的 Pagination 实例
        let data: Vec<TestData> = vec![];
        let pagination = Pagination {
            data,
            page_size: 10,
            page_no: 1,
            total: 0,
        };

        // 验证 Pagination 实例的各个字段
        assert_eq!(pagination.data.len(), 0);
        assert_eq!(pagination.page_size, 10);
        assert_eq!(pagination.page_no, 1);
        assert_eq!(pagination.total, 0);
        assert_eq!(
            serde_json::to_string(&pagination).unwrap(),
            r#"{"data":[],"pageSize":10,"pageNo":1,"total":0}"#
        )
    }

    #[test]
    fn test_pagination_params_default_values() {
        // 创建一个没有提供参数的 PaginationParams 实例
        let json = r#"{}"#;
        let params: PaginationParams = serde_json::from_str(json).unwrap();

        // 验证 PaginationParams 实例的默认值
        assert_eq!(params.page_size(), 10);
        assert_eq!(params.page_no(), 1);
        assert_eq!(params.no_page, false);
    }

    #[test]
    fn test_pagination_params_default_values_no_page() {
        // 创建一个没有提供参数的 PaginationParams 实例
        let json = r#"{"noPage": true}"#;
        let params: PaginationParams = serde_json::from_str(json).unwrap();

        // 验证 PaginationParams 实例的默认值
        assert_eq!(params.page_size(), 10);
        assert_eq!(params.page_no(), 1);
        assert_eq!(params.no_page(), true);
    }

    #[test]
    fn test_pagination_params_custom_values() {
        let json = r#"{"pageSize": 20, "pageNo": 2}"#;
        let params: PaginationParams = serde_json::from_str(json).unwrap();

        // 验证 PaginationParams 实例的自定义值
        assert_eq!(params.page_size(), 20);
        assert_eq!(params.page_no(), 2);
    }
}
