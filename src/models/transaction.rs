use sqlx::PgPool;

use crate::{
    dtos::transaction::{RequestTransaction, ResponseBalance, TransactionType},
    error::Error,
};

pub struct TransactionModel<'a> {
    pg_pool: &'a PgPool,
}

impl<'a> TransactionModel<'a> {
    pub fn new(pg_pool: &'a PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn create_new_transaction(
        &self,
        user_id: i32,
        transaction: RequestTransaction,
    ) -> Result<ResponseBalance, Error> {
        let result = match sqlx::query!(
            r#"--sql
                SELECT * FROM create_new_transaction($1, $2, $3, $4)"#,
            user_id,
            transaction.transaction_type as TransactionType,
            transaction.description,
            transaction.value,
        )
        .fetch_one(self.pg_pool)
        .await
        {
            Ok(balance) => balance,
            Err(sqlx::Error::RowNotFound) => return Error::not_found(),
            Err(sqlx::Error::Database(db_error)) => {
                return match db_error.code().as_deref() {
                    Some("P0001") => {
                        if db_error.message() == "insufficient balance for transaction" {
                            Error::transaction_denied()
                        } else {
                            log::error!("database_error: {}", db_error.message());
                            Error::other()
                        }
                    }
                    sql_code => {
                        log::error!("database_error: {}, sql code: {:?}", db_error.message(), sql_code);
                        Error::other()
                    },
                }
            }
            err => {
                log::error!("{:?}", err);
                return Error::other()},
        };

        Ok(ResponseBalance {
            balance: result.balance.unwrap(),
            limit: result.limit.unwrap(),
        })
    }
}
