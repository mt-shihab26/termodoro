use sea_orm_migration::prelude::*;

/// SeaORM migration for creating and dropping the `sessions` table.
pub struct Migration;

/// Column identifiers for the `sessions` table schema.
#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    Phase,
    DurationSecs,
    StartedAt,
    EndedAt,
    TodoId,
    CreatedAt,
    UpdatedAt,
}

impl MigrationName for Migration {
    /// Returns the stable migration identifier.
    fn name(&self) -> &str {
        "m_01_create_sessions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Creates the `sessions` table if it does not already exist.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sessions::Phase).string().not_null())
                    .col(ColumnDef::new(Sessions::DurationSecs).integer().not_null())
                    .col(ColumnDef::new(Sessions::StartedAt).string().null())
                    .col(ColumnDef::new(Sessions::EndedAt).string().null())
                    .col(ColumnDef::new(Sessions::TodoId).integer().null())
                    .col(ColumnDef::new(Sessions::CreatedAt).string().not_null().default(""))
                    .col(ColumnDef::new(Sessions::UpdatedAt).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    /// Drops the `sessions` table, rolling back this migration.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}
