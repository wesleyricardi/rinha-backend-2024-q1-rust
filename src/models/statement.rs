use chrono::Utc;
use sqlx::PgPool;
use tokio::try_join;

use crate::{
    dtos::{
        statement::{Balance, ResponseStatement, Transaction},
        transaction::TransactionType,
    },
    error::Error,
};

pub struct StatementModel<'a> {
    pg_pool: &'a PgPool,
}

impl<'a> StatementModel<'a> {
    pub fn new(pg_pool: &'a PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn get_statement(&self, user_id: i32) -> Result<ResponseStatement, Error> {
        let balance = sqlx::query!(
            r#"--sql
                SELECT user_limit.limit, COALESCE(transaction.current_balance, 0) AS current_balance
                FROM user_limit
                LEFT JOIN transaction ON user_limit.user_id = transaction.user_id
                WHERE user_limit.user_id = $1
                ORDER BY transaction.created_at DESC
                LIMIT 1"#,
            user_id
        )
        .fetch_one(self.pg_pool);

        let last_transactions = 
            sqlx::query_as!(Transaction, r#"--sql
                    SELECT transaction.type AS "transaction_type!:TransactionType", TRIM(transaction.description) AS description, transaction.value, transaction.created_at AS accomplish_at
                    FROM transaction 
                    WHERE transaction.user_id = $1 
                    ORDER BY transaction.created_at DESC 
                    LIMIT 10"#, 
                user_id)
            .fetch_all(self.pg_pool);

        let (balance, last_transactions) = match try_join!(balance, last_transactions) {
            Ok((balance, transactions)) => (balance, transactions),
            Err(sqlx::Error::RowNotFound) => return Error::not_found(),
            Err(err) => {
                log::error!("{:?}", err);
                return Error::other();
            }
        };

        Ok(ResponseStatement {
            balance: Balance {
                total: balance.current_balance.unwrap_or(0),
                balance_date: Utc::now(),
                limit: balance.limit,
            },
            last_transactions,
        })
    }
}
