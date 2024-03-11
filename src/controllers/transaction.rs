use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    dtos::transaction::RequestTransaction,
    models::transaction::{TransactionModel, TransactionModelError::*},
    state::AppState,
};

#[derive(Deserialize)]
struct Params {
    user_id: i32,
}

#[utoipa::path(
  params(("user_id" = String, description = "id do cliente")),
  tag = "transação",
  request_body = RequestTransaction,
  responses(
      (status = 200, description = "Retorno de sucesso", body = ResponseBalance),
      (status = 422, description = "Retorno em caso de transação que deixará o saldo incosistente"),
      (status = 404, description = "Retorno em caso do cliente nao existir"),
  )
)]
#[post("/clientes/{user_id}/transacoes")]
pub async fn create_transaction(
    param: web::Path<Params>,
    app_state: web::Data<AppState>,
    request: web::Json<RequestTransaction>,
) -> impl Responder {
    if request.0.description.len() > 10 || request.0.description.is_empty() {
        return HttpResponse::UnprocessableEntity().finish();
    }

    match TransactionModel::new(&app_state.pg_pool)
        .create_new_transaction(param.user_id, request.0)
        .await
    {
        Ok(success) => HttpResponse::Ok().json(success),
        Err(NotFound) => HttpResponse::NotFound().finish(),
        Err(TransactionDenied) => HttpResponse::UnprocessableEntity().finish(),
        Err(Other) => HttpResponse::InternalServerError().finish(),
    }
}
