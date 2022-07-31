use actix_files::NamedFile;
use actix_web::{HttpResponse, Error};
use actix_web::web::Query;
use crate::random::{generate_number, generate_numbers, generate_uuid};
use serde::{Serialize, Deserialize};
use std::str;
use std::path::PathBuf;

const API_VERSION: &str = env!("CARGO_PKG_VERSION");
const OPENAPI_SPEC_PATH: &str = "./static/openapi.yml";
const INDEX_PATH: &str = "./static/index.html";
const FAVICON_PATH: &str = "./static/favicon.ico";

#[derive(Deserialize, Serialize)]
pub struct NumberQuery {
    min: u32,
    max: u32,
}

#[derive(Deserialize, Serialize)]
pub struct NumbersQuery {
    min: u32,
    max: u32,
    length: u32,
}

#[derive(Serialize)]
pub struct ApiResponse<T, K> {
    #[serde(rename(serialize = "apiVersion"))]
    api_version: String,
    params: K,
    data: ApiData<T>,
}

#[derive(Serialize)]
pub struct ApiData<T> {
    kind: String,
    value: T,
}

pub async fn home() -> Result<NamedFile, Error> {
    let path: PathBuf = INDEX_PATH.parse().unwrap();
    Ok(NamedFile::open(path).unwrap().set_content_type(mime::TEXT_HTML))
}

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn serve_openapi_spec() -> Result<NamedFile, Error> {
    let path: PathBuf = OPENAPI_SPEC_PATH.parse().unwrap();
    Ok(NamedFile::open(path).unwrap().set_content_type(mime::TEXT_PLAIN))
}
pub async fn serve_favicon() -> Result<NamedFile, Error> {
    let path: PathBuf = FAVICON_PATH.parse().unwrap();
    Ok(NamedFile::open(path).unwrap().set_content_type(mime::TEXT_PLAIN))
}

pub async fn number(bounds: Query<NumberQuery>) -> HttpResponse {
    let value = generate_number(bounds.min, bounds.max);
    let api_response = create_response(value, bounds.into_inner(), "number");
    HttpResponse::Ok().body(serde_json::to_string(&api_response).unwrap())
}

pub async fn numbers(bounds: Query<NumbersQuery>) -> HttpResponse {
    let value = generate_numbers(bounds.min, bounds.max, bounds.length);
    let api_response = create_response(value, bounds.into_inner(), "number");
    HttpResponse::Ok().body(serde_json::to_string(&api_response).unwrap())
}

pub async fn uuid() -> HttpResponse {
    let value = generate_uuid();
    let api_response = create_response(value, (), "string");
    HttpResponse::Ok().body(serde_json::to_string(&api_response).unwrap())
}

fn create_response<T, K>(value: T, params: K, kind: &str) -> ApiResponse<T, K> {
    ApiResponse {
        api_version: API_VERSION.to_string(),
        params,
        data: ApiData {
            kind: kind.to_string(),
            value,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::self, body};

    #[actix_web::test]
    async fn ping_ok() {
        let resp = ping().await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn home_path() {
        let resp = home().await.unwrap();
        let path = resp.path().as_os_str();
        assert!(path.to_str().unwrap().to_string().contains("index.html"));
    }

    #[actix_web::test]
    async fn serve_openapi_spec_path() {
        let resp = serve_openapi_spec().await.unwrap();
        let path = resp.path().as_os_str();
        assert!(path.to_str().unwrap().to_string().contains("openapi.yml"));
    }

    #[actix_web::test]
    async fn serve_favicon_path() {
        let resp = serve_favicon().await.unwrap();
        let path = resp.path().as_os_str();
        assert!(path.to_str().unwrap().to_string().contains("favicon.ico"));
    }

    #[actix_web::test]
    async fn number_ok() {
        let resp = number(Query { 0: (NumberQuery { min: 0, max: 10 }) }).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn numbers_ok() {
        let resp = numbers(Query { 0: (NumbersQuery { min: 0, max: 10, length: 3 }) }).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn uuid_ok() {
        let resp = uuid().await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn number_api_version() {
        let resp = number(Query { 0: (NumberQuery { min: 0, max: 10 }) }).await;
        let bytes = body::to_bytes(resp.into_body()).await.unwrap();

        let actual = str::from_utf8(&bytes).unwrap();
        let expected = "\"apiVersion\":\"0.3.0\"";
        assert!(actual.contains(expected));
    }
}
