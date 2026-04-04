//! Migration that creates the `todos` table used to persist task records.

use sea_orm_migration::prelude::*;

/// SeaORM migration for creating and dropping the `todos` table.
pub struct Migration;

/// Column identifiers for the `todos` table schema.
#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    Text,
    Done,
    DueDate,
    Repeat,
    ParentId,
    CreatedAt,
    UpdatedAt,
}

impl MigrationName for Migration {
    /// Returns the stable migration identifier.
    fn name(&self) -> &str {
        "m_00_create_todos"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Creates the `todos` table if it does not already exist.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todos::Text).string().not_null())
                    .col(ColumnDef::new(Todos::Done).boolean().not_null().default(false))
                    .col(ColumnDef::new(Todos::DueDate).string().null())
                    .col(ColumnDef::new(Todos::Repeat).string().null())
                    .col(ColumnDef::new(Todos::ParentId).integer().null())
                    .col(ColumnDef::new(Todos::CreatedAt).string().not_null().default(""))
                    .col(ColumnDef::new(Todos::UpdatedAt).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    /// Drops the `todos` table, rolling back this migration.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Todos::Table).to_owned()).await
    }
}
