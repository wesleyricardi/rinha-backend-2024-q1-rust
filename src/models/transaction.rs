use sqlx::PgPool;

use crate::dtos::transaction::{RequestTransaction, ResponseBalance, TransactionType};

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
    ) -> Result<ResponseBalance, TransactionModelError> {
        let result = sqlx::query!(
            r#"--sql
                SELECT * FROM create_new_transaction($1, $2, $3, $4)"#,
            user_id,
            transaction.transaction_type as TransactionType,
            transaction.description,
            transaction.value,
        )
        .fetch_one(self.pg_pool)
        .await?;

        //safe unwrap
        Ok(ResponseBalance {
            balance: result.balance.unwrap(),
            limit: result.limit.unwrap(),
        })
    }
}

pub enum TransactionModelError {
    NotFound,
    TransactionDenied,
    Other,
}

pub(self) mod implementation_for_error {
    use super::*;

    impl From<sqlx::Error> for TransactionModelError {
        fn from(sqlx_error: sqlx::Error) -> Self {
            match sqlx_error {
                sqlx::Error::RowNotFound => TransactionModelError::NotFound,
                sqlx::Error::Database(db_error) => match db_error.code().as_deref() {
                    Some("P0001") => {
                        if db_error.message() == "insufficient balance for transaction" {
                            return TransactionModelError::TransactionDenied;
                        }

                        TransactionModelError::Other
                    }
                    _others_db_error_code => TransactionModelError::Other,
                },
                _others_sqlx_error => TransactionModelError::Other,
            }
        }
    }
}
