use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(DeriveIden)]
enum TimerSessions {
    Table,
    Id,
    Phase,
    DurationSecs,
    CompletedAt,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260101_000002_create_timer_sessions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TimerSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimerSessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimerSessions::Phase).string().not_null())
                    .col(ColumnDef::new(TimerSessions::DurationSecs).integer().not_null())
                    .col(ColumnDef::new(TimerSessions::CompletedAt).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimerSessions::Table).to_owned())
            .await
    }
}
