use sea_orm_migration::prelude::*;

/*
    * This is a migration script.

    CREATE TABLE guilds (
        id SERIAL PRIMARY KEY NOT NULL,
        guild_id TEXT NOT NULL,
        name TEXT NOT NULL,
        avatar_url TEXT,
        locale TEXT NOT NULL DEFAULT 'ja',
        UNIQUE (guild_id)
    );
*/

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Guild::GuildId)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Guild::Name).text().not_null())
                    .col(ColumnDef::new(Guild::AvatarUrl).text())
                    .col(
                        ColumnDef::new(Guild::Locale)
                            .text()
                            .not_null()
                            .default("ja"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Guild {
    Table,
    Id,
    GuildId,
    Name,
    AvatarUrl,
    Locale,
}
