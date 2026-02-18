use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tracker::Table)
                    .if_not_exists()
                    .col(pk_auto(Tracker::Id).big_unsigned())
                    .col(string(Tracker::Name))
                    .col(text(Tracker::Desc))
                    .col(
                        timestamp(Tracker::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Tracker::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .name("idx_tracker_name")
                            .table(Tracker::Table)
                            .col(Tracker::Name),
                    )
                    .index(
                        Index::create()
                            .name("idx_created_at")
                            .table(Tracker::Table)
                            .col(Tracker::CreatedAt),
                    )
                    .index(
                        Index::create()
                            .name("idx_updated_at")
                            .table(Tracker::Table)
                            .col(Tracker::UpdatedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tracker::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tracker {
    Table,
    Id,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
}
