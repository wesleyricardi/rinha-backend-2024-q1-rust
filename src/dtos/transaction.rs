use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "d")]
    D,
}

#[derive(Deserialize, Debug, ToSchema)]
#[schema(title = "post_transação_request")]
pub struct RequestTransaction {
    #[serde(rename(deserialize = "valor"))]
    #[schema(example = "100", rename = "valor")]
    pub value: i64,
    #[serde(rename(deserialize = "tipo"))]
    #[schema(example = "c", rename = "tipo", value_type = String)]
    pub transaction_type: TransactionType,
    #[serde(rename(deserialize = "descricao"))]
    #[schema(example = "alguma descrição", rename = "descricao")]
    pub description: String,
}

#[derive(Serialize, ToSchema, Debug)]
#[schema(title = "post_transação_response")]
pub struct ResponseBalance {
    #[serde(rename = "limite")]
    #[schema(example = "100000", rename = "limite")]
    pub limit: i64,
    #[serde(rename = "saldo")]
    #[schema(example = "-9098", rename = "saldo")]
    pub balance: i64,
}
