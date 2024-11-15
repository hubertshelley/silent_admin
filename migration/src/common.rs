use sea_orm_migration::prelude::*;

pub(crate) trait BaseModelExt {
    fn append_base_columns(&mut self) -> &mut Self;
}

impl BaseModelExt for TableCreateStatement {
    fn append_base_columns(&mut self) -> &mut Self {
        BaseModel::append_base_columns(self)
    }
}

#[derive(DeriveIden)]
pub(crate) enum BaseModel {
    Id,
    DelFlag,
    CreateBy,
    CreateTime,
    UpdateBy,
    UpdateTime,
}

impl BaseModel {
    fn append_base_columns(statement: &mut TableCreateStatement) -> &mut TableCreateStatement {
        statement.col(
            ColumnDef::new(Self::Id)
                .string_len(36)
                .not_null()
                .primary_key().comment("ID"),
        )
            .col(
                ColumnDef::new(Self::DelFlag)
                    .integer()
                    .not_null()
                    .default(0)
                    .comment("删除标记"),
            )
            .col(
                ColumnDef::new(Self::CreateBy)
                    .string_len(36)
                    .not_null()
                    .default("")
                    .comment("创建者"),
            )
            .col(
                ColumnDef::new(Self::CreateTime)
                    .date_time()
                    .not_null()
                    .default(Expr::current_timestamp())
                    .comment("创建时间"),
            )
            .col(
                ColumnDef::new(Self::UpdateBy)
                    .string_len(36)
                    .not_null()
                    .default("")
                    .comment("更新者"),
            )
            .col(
                ColumnDef::new(Self::UpdateTime)
                    .date_time()
                    .not_null()
                    .default(Expr::current_timestamp())
                    .comment("更新时间"),
            )
            .index(Index::create().unique().name("idx-id").col(Self::Id))
    }
}