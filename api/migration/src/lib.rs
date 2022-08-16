pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220803_053917_create_event;
mod m20220816_074716_billing;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220803_053917_create_event::Migration),
            Box::new(m20220816_074716_billing::Migration),
        ]
    }
}
