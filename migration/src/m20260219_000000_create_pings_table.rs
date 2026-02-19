use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pings::Table)
                    .if_not_exists()
                    .col(pk_auto(Pings::Id).big_unsigned())
                    .col(big_unsigned(Pings::TrackerId).not_null())
                    .col(double(Pings::Lat).not_null())
                    .col(double(Pings::Lon).not_null())
                    .col(string(Pings::Note))
                    .col(
                        timestamp(Pings::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Pings::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .name("idx_lat")
                            .table(Pings::Table)
                            .col(Pings::Lat),
                    )
                    .index(
                        Index::create()
                            .name("idx_lon")
                            .table(Pings::Table)
                            .col(Pings::Lon),
                    )
                    .index(
                        Index::create()
                            .name("idx_note")
                            .table(Pings::Table)
                            .col(Pings::Note),
                    )
                    .index(
                        Index::create()
                            .name("idx_created_at")
                            .table(Pings::Table)
                            .col(Pings::CreatedAt),
                    )
                    .index(
                        Index::create()
                            .name("idx_updated_at")
                            .table(Pings::Table)
                            .col(Pings::UpdatedAt),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Pings::Table)
                            .from_col(Pings::TrackerId)
                            .to_tbl(Trackers::Table)
                            .to_col(Trackers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pings {
    Table,
    Id,
    TrackerId,
    Lat,
    Lon,
    Note,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Trackers {
    Table,
    Id,
}
