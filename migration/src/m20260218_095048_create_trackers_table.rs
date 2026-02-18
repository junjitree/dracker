use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trackers::Table)
                    .if_not_exists()
                    .col(pk_auto(Trackers::Id).big_unsigned())
                    .col(big_unsigned(Trackers::UserId).not_null())
                    .col(string(Trackers::Name))
                    .col(text(Trackers::Desc))
                    .col(
                        timestamp(Trackers::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Trackers::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .name("idx_tracker_name")
                            .table(Trackers::Table)
                            .col(Trackers::Name),
                    )
                    .index(
                        Index::create()
                            .name("idx_created_at")
                            .table(Trackers::Table)
                            .col(Trackers::CreatedAt),
                    )
                    .index(
                        Index::create()
                            .name("idx_updated_at")
                            .table(Trackers::Table)
                            .col(Trackers::UpdatedAt),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Trackers::Table)
                            .from_col(Trackers::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trackers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Trackers {
    Table,
    Id,
    UserId,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
