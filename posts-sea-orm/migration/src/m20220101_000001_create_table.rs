use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS users (
                  id          SERIAL PRIMARY KEY,
                  username    VARCHAR(64) NOT NULL UNIQUE,
                  password    VARCHAR(64) NOT NULL,
                  deleted_at  TIMESTAMPTZ DEFAULT NULL,
                  token       TEXT DEFAULT NULL
                );"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Title,
    Text,
}
