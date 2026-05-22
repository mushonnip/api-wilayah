use ntex::web::{self, middleware, App, HttpServer};
use std::sync::Arc;

mod data;
mod handlers;
mod models;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8989".to_string());
    let addr = format!("0.0.0.0:{}", port);

    log::info!("Loading region data...");
    let store = Arc::new(data::RegionStore::load().expect("failed to load region data"));
    log::info!(
        "Loaded: {} provinsi | {} kab/kota | {} kecamatan | {} desa/kel",
        store.provinsi.len(),
        store.kab_kota.len(),
        store.kecamatan.len(),
        store.desa_kel.len(),
    );

    log::info!("Server starting on http://{}", addr);

    HttpServer::new(async move || {
        App::new()
            .state(store.clone())
            .middleware(middleware::Logger::default())
            // CORS: wildcard origin, no credentials (credentials + wildcard is invalid per spec)
            .middleware(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, OPTIONS")
                    .header("Access-Control-Allow-Headers", "Content-Type")
                    .header("Access-Control-Expose-Headers", "x-total-count"),
            )
            .service(
                web::scope("/v2")
                    // flat endpoints registered before dynamic ones (static > dynamic priority)
                    .service(
                        web::resource("/provinsi").route(web::get().to(handlers::get_provinsi)),
                    )
                    .service(
                        web::resource("/kab-kota").route(web::get().to(handlers::get_kab_kota)),
                    )
                    .service(
                        web::resource("/kecamatan").route(web::get().to(handlers::get_kecamatan)),
                    )
                    .service(
                        web::resource("/desa-kel").route(web::get().to(handlers::get_desa_kel)),
                    )
                    .service(
                        web::resource("/region/details")
                            .route(web::get().to(handlers::get_region_details)),
                    )
                    // dynamic filtered endpoints
                    .service(
                        web::resource("/{kode}/kab-kota")
                            .route(web::get().to(handlers::get_kab_kota_by_prov)),
                    )
                    .service(
                        web::resource("/{kode}/kecamatan")
                            .route(web::get().to(handlers::get_kecamatan_by_kab_kota)),
                    )
                    .service(
                        web::resource("/{kode}/desa-kel")
                            .route(web::get().to(handlers::get_desa_kel_by_kecamatan)),
                    ),
            )
    })
    .bind(&addr)?
    .run()
    .await
}
