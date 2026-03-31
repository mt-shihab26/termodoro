use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    Text,
    Done,
    DueDate,
    Repeat,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250101_000001_create_todos"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
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
                    .col(
                        ColumnDef::new(Todos::Done)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Todos::DueDate).string().null())
                    .col(ColumnDef::new(Todos::Repeat).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await
    }
}
