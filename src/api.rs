use actix_web::{HttpResponse};
use actix_web::web::Query;
use crate::random::generate_number;
use serde::{Serialize, Deserialize};
use std::str;

#[derive(Deserialize, Serialize)]
pub struct Bounds {
    min: u32,
    max: u32,
}

#[derive(Serialize)]
pub struct ApiResponse {
    #[serde(rename(serialize = "apiVersion"))]
    api_version: String,
    params: Bounds,
    data: ApiData,
}

#[derive(Serialize)]
pub struct ApiData {
    kind: String,
    value: u32,
}

pub async fn number(bounds: Query<Bounds>) -> HttpResponse {
    let value = generate_number(bounds.min, bounds.max);
    let api_response = create_response(value, bounds.into_inner());
    HttpResponse::Ok().body(serde_json::to_string(&api_response).unwrap())
}

fn create_response(value: u32, params: Bounds) -> ApiResponse {
    ApiResponse {
        api_version: "0.1.0".to_string(),
        params,
        data: ApiData {
            kind: "number".to_string(),
            value,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::self, body};

    #[actix_web::test]
    async fn number_ok() {
        let resp = number(Query { 0: (Bounds { min: 0, max: 10 }) }).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn number_api_version() {
        let resp = number(Query { 0: (Bounds { min: 0, max: 10 }) }).await;
        let bytes = body::to_bytes(resp.into_body()).await.unwrap();

        let actual = str::from_utf8(&bytes).unwrap();
        let expected = "\"apiVersion\":\"0.1.0\"";
        assert!(actual.contains(expected));
    }
}
