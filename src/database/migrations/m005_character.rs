use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Character::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Character::CharacterId)
                        .big_integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                )
                .col(ColumnDef::new(Character::Name).text().not_null())
                .col(ColumnDef::new(Character::Description).text().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Character::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Character {
    Table,
    CharacterId,
    Name,
    Description,
}
