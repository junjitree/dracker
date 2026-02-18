use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id).big_unsigned())
                    .col(string(Users::Email).unique_key().not_null())
                    .col(string(Users::Password).not_null())
                    .col(
                        timestamp(Users::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Users::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("idx_email")
                            .table(Users::Table)
                            .col(Users::Email),
                    )
                    .index(
                        Index::create()
                            .name("idx_created_at")
                            .table(Users::Table)
                            .col(Users::CreatedAt),
                    )
                    .index(
                        Index::create()
                            .name("idx_updated_at")
                            .table(Users::Table)
                            .col(Users::UpdatedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
