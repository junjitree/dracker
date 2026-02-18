pub use sea_orm_migration::prelude::*;

mod m20260218_095047_create_users_table;
mod m20260218_095048_create_trackers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260218_095047_create_users_table::Migration),
            Box::new(m20260218_095048_create_trackers_table::Migration),
        ]
    }
}
