use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::FromRow;

use crate::ctx::Ctx;

use super::base::{self, Condition};
use super::error::Result;
use super::{base::DbBmc, ModelManager};

// region:    --- Balance Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Balance {
    pub token: String,
    pub invoice: String,
    pub balance_msat: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Fields, Deserialize)]
pub struct BalanceForCreate {
    pub token: String,
    pub invoice: String,
    pub balance_msat: i64,
}

#[derive(Fields, Deserialize)]
pub struct BalanceForUpdate {
    pub token: Option<String>,
    pub balance_msat: Option<i64>,
}
// endregion: --- Balance Types

// region:    --- BalanceBmc
pub struct BalanceBmc;

impl DbBmc for BalanceBmc {
    const TABLE: &'static str = "balance";
}

#[allow(dead_code)]
impl BalanceBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, balance_c: BalanceForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, balance_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Balance> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_by_conditions(
        ctx: &Ctx,
        mm: &ModelManager,
        conditions: Vec<Condition>,
    ) -> Result<Balance> {
        base::get_by_conditions::<Self, _>(ctx, mm, conditions).await
    }

    pub async fn get_by_token(
        _ctx: &Ctx,
        mm: &ModelManager,
        token: String,
    ) -> Result<Option<Balance>> {
        let db = mm.db();

        let entity = sqlb::select()
            .table(Self::TABLE)
            .columns(Balance::field_names())
            .and_where("token", "=", token)
            .fetch_optional(db)
            .await?;

        Ok(entity)
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Balance>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        balance_u: BalanceForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, balance_u).await
    }

    pub async fn update_by_token(
        _ctx: &Ctx,
        mm: &ModelManager,
        token: String,
        balance_u: BalanceForUpdate,
    ) -> Result<()> {
        let db = mm.db();

        let fields = balance_u.not_none_fields();
        sqlb::update()
            .table(Self::TABLE)
            .data(fields)
            .and_where("token", "=", token)
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn delete_by_token(_ctx: &Ctx, mm: &ModelManager, token: String) -> Result<()> {
        let db = mm.db();

        sqlb::delete()
            .table(Self::TABLE)
            .and_where("token", "=", token)
            .exec(db)
            .await?;

        Ok(())
    }
}
// endregion: --- BalanceBmc
