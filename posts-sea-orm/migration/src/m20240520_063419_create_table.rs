use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS tasks (
                  id            SERIAL PRIMARY KEY,
                  priority      VARCHAR(4) DEFAULT NULL,
                  title         VARCHAR(255) NOT NULL,
                  completed_at  TIMESTAMPTZ DEFAULT NULL,
                  description   TEXT DEFAULT NULL,
                  deleted_at    TIMESTAMPTZ DEFAULT NULL,
                  user_id       INTEGER DEFAULT NULL,
                  is_default    BOOLEAN DEFAULT FALSE
                );"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    Title,
    Text,
}
