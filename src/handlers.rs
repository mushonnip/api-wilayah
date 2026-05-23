use ntex::web::{HttpRequest, HttpResponse};
use ntex::web::types::{Path, Query, State};
use serde::Serialize;
use std::sync::Arc;

use crate::data::RegionStore;
use crate::models::*;

fn paginate<T: Clone>(items: &[T], query: &PaginationQuery) -> Vec<T> {
    if query.get_all() {
        return items.to_vec();
    }
    let page = query.page();
    let size = query.size();
    let start = page * size;
    if start >= items.len() {
        return vec![];
    }
    let end = (start + size).min(items.len());
    items[start..end].to_vec()
}

fn list_resp<T: Serialize>(items: Vec<T>, total: usize) -> HttpResponse {
    let body = match serde_json::to_string(&ListResponse { data: items, total }) {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .set_header("x-total-count", total.to_string())
        .body(body)
}

pub async fn get_provinsi(
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    match query.search.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => {
            let records = state.search_provinsi(s);
            let total = records.len();
            list_resp(paginate(&records, &query), total)
        }
        None => {
            let total = state.provinsi.len();
            list_resp(paginate(&state.provinsi, &query), total)
        }
    }
}

pub async fn get_kab_kota(
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    match query.search.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => {
            let records = state.search_kab_kota(s);
            let total = records.len();
            list_resp(paginate(&records, &query), total)
        }
        None => {
            let total = state.kab_kota.len();
            list_resp(paginate(&state.kab_kota, &query), total)
        }
    }
}

pub async fn get_kab_kota_by_prov(
    path: Path<KodeParam>,
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    let mut records = state.get_kab_kota_by_prov(&path.kode);

    if let Some(s) = query.search.as_deref().filter(|s| !s.is_empty()) {
        let lower = s.to_lowercase();
        records.retain(|r| {
            r.nama.to_lowercase().contains(&lower) || r.kode.contains(s)
        });
    }

    let total = records.len();
    list_resp(paginate(&records, &query), total)
}

pub async fn get_kecamatan(
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    match query.search.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => {
            let records = state.search_kecamatan(s);
            let total = records.len();
            list_resp(paginate(&records, &query), total)
        }
        None => {
            let total = state.kecamatan.len();
            list_resp(paginate(&state.kecamatan, &query), total)
        }
    }
}

pub async fn get_kecamatan_by_kab_kota(
    path: Path<KodeParam>,
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    let mut records = state.get_kecamatan_by_kab_kota(&path.kode);

    if let Some(s) = query.search.as_deref().filter(|s| !s.is_empty()) {
        let lower = s.to_lowercase();
        records.retain(|r| {
            r.nama.to_lowercase().contains(&lower) || r.kode.contains(s)
        });
    }

    let total = records.len();
    list_resp(paginate(&records, &query), total)
}

pub async fn get_desa_kel(
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    match query.search.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => {
            let records = state.search_desa_kel(s);
            let total = records.len();
            list_resp(paginate(&records, &query), total)
        }
        None => {
            // avoid cloning all 82K records: paginate from slice directly if not get_all
            let total = state.desa_kel.len();
            list_resp(paginate(&state.desa_kel, &query), total)
        }
    }
}

pub async fn get_desa_kel_by_kecamatan(
    path: Path<KodeParam>,
    state: State<Arc<RegionStore>>,
    query: Query<PaginationQuery>,
) -> HttpResponse {
    let mut records = state.get_desa_kel_by_kecamatan(&path.kode);

    if let Some(s) = query.search.as_deref().filter(|s| !s.is_empty()) {
        let lower = s.to_lowercase();
        records.retain(|r| {
            r.nama.to_lowercase().contains(&lower) || r.kode.contains(s)
        });
    }

    let total = records.len();
    list_resp(paginate(&records, &query), total)
}

pub async fn get_region_details(
    req: HttpRequest,
    state: State<Arc<RegionStore>>,
) -> HttpResponse {
    // multi-value query param: ?ids=11&ids=32.73
    let ids: Vec<String> = req
        .query_string()
        .split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            if k == "ids" && !v.is_empty() {
                Some(v.to_string())
            } else {
                None
            }
        })
        .collect();

    if ids.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"parameter 'ids' is required"}"#);
    }

    match state.get_region_details(&ids) {
        Ok(details) => {
            let body = match serde_json::to_string(&DataResponse { data: details }) {
                Ok(b) => b,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        }
        Err(e) => {
            // region not found -> 404, not 500
            HttpResponse::NotFound()
                .content_type("application/json")
                .body(serde_json::json!({ "error": e }).to_string())
        }
    }
}
