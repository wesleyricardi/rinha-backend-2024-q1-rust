use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{error::Error, models::statement::StatementModel, state::AppState};

#[derive(Deserialize)]
struct Params {
    user_id: i32,
}

#[utoipa::path(
  params(("user_id" = String, description = "id do cliente")),
  tag = "extrato",
  responses(
      (status = 200, description = "Retorno de sucesso", body = ResponseStatement),
      (status = 404, description = "Retorno em caso do cliente não existir"),
  )
)]
#[get("/clientes/{user_id}/extrato")]
pub async fn get_statement(
    param: web::Path<Params>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match StatementModel::new(&app_state.pg_pool)
        .get_statement(param.user_id)
        .await
    {
        Ok(success) => HttpResponse::Ok().json(success),
        Err(Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
