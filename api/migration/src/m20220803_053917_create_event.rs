use sea_orm_migration::prelude::*;
/*
    * SQL

    CREATE TABLE events (
        id SERIAL PRIMARY KEY NOT NULL,
        guild_id TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT,
        notifications TEXT NOT NULL DEFAULT '[]',
        color TEXT NOT NULL DEFAULT '#FF0000',
        is_all_day BOOLEAN NOT NULL DEFAULT FALSE,
        start_at TIMESTAMP NOT NULL,
        end_at TIMESTAMP NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        discord_event_id TEXT NOT NULL,
        notify_channel_id TEXT,
    );
*/

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::GuildId).string().not_null())
                    .col(ColumnDef::new(Event::Name).string().not_null())
                    .col(ColumnDef::new(Event::Description).string())
                    .col(
                        ColumnDef::new(Event::Notifications)
                            .text()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Event::Color)
                            .text()
                            .not_null()
                            .default("#FF0000"),
                    )
                    .col(
                        ColumnDef::new(Event::IsAllDay)
                            .boolean()
                            .not_null()
                            .default("false"),
                    )
                    .col(ColumnDef::new(Event::StartAt).timestamp().not_null())
                    .col(ColumnDef::new(Event::EndAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Event::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(ColumnDef::new(Event::DiscordEventId).string())
                    .col(ColumnDef::new(Event::NotifyChannelId).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Event {
    Table,
    Id,
    GuildId,
    Name,
    Description,
    Notifications,
    Color,
    IsAllDay,
    StartAt,
    EndAt,
    CreatedAt,
    DiscordEventId,
    NotifyChannelId,
}
