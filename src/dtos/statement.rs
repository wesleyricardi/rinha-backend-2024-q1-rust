use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use super::transaction::TransactionType;

#[derive(Serialize, Debug, ToSchema)]
#[schema(title = "extrato_saldo")]
pub struct Balance {
    #[schema(example = "-9098")]
    pub total: i64,
    #[serde(rename = "data_extrato")]
    #[schema(example = "2024-01-17T02:34:41.217753Z", rename = "data_extrato", value_type = String)]
    pub balance_date: DateTime<Utc>,
    #[serde(rename = "limite")]
    #[schema(example = "100000", rename = "limite")]
    pub limit: i64,
}

#[derive(Serialize, Debug, ToSchema)]
#[schema(title = "extrato_transações")]
pub struct Transaction {
    #[serde(rename = "valor")]
    #[schema(example = "100", rename = "valor")]
    pub value: i64,
    #[serde(rename = "tipo")]
    #[schema(example = "c", rename = "tipo", value_type = String)]
    pub transaction_type: TransactionType,
    #[serde(rename = "descricao")]
    #[schema(example = "descrição", rename = "descricao")]
    pub description: Option<String>,
    #[serde(rename = "realizada_em")]
    #[schema(example = "2024-01-17T02:34:41.217753Z", rename = "realizada_em", value_type = String)]
    pub accomplish_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, ToSchema)]
#[schema(title = "get_extrato_response")]
pub struct ResponseStatement {
    #[serde(rename = "saldo")]
    #[schema(rename = "saldo")]
    pub balance: Balance,
    #[serde(rename = "ultimas_transacoes")]
    #[schema(rename = "ultimas_transacoes")]
    pub last_transactions: Vec<Transaction>,
}
