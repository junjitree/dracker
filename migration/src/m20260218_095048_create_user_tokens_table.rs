use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTokens::Table)
                    .if_not_exists()
                    .col(pk_auto(UserTokens::Id).big_unsigned())
                    .col(big_unsigned(UserTokens::UserId).not_null())
                    .col(uuid(UserTokens::Token).unique_key())
                    .col(string(UserTokens::Agent))
                    .col(
                        timestamp(UserTokens::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(UserTokens::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(UserTokens::Table)
                            .from_col(UserTokens::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-user_tokens-agent")
                            .table(UserTokens::Table)
                            .col(UserTokens::Agent),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserTokens {
    Table,
    Id,
    UserId,
    Token,
    Agent,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
