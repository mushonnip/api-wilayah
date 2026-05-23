use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ProvinsiRecord {
    pub kode: String,
    pub nama: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct KabupatenKotaRecord {
    pub kode: String,
    pub nama: String,
    pub lat: f64,
    pub lng: f64,
    pub kode_provinsi: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KecamatanRecord {
    pub kode: String,
    pub nama: String,
    pub lat: f64,
    pub lng: f64,
    pub kode_kabupaten_kota: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DesaKelurahanRecord {
    pub kode: String,
    pub nama: String,
    pub lat: f64,
    pub lng: f64,
    pub kode_kecamatan: String,
    pub kode_pos: String,
}

#[derive(Debug, Serialize)]
pub struct RegionDetail {
    pub kode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desa_kelurahan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kecamatan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kabupaten_kota: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provinsi: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct DataResponse<T: Serialize> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub search: Option<String>,
    pub get_all: Option<bool>,
}

impl PaginationQuery {
    pub fn page(&self) -> usize {
        self.page.unwrap_or(0)
    }

    pub fn size(&self) -> usize {
        self.size.unwrap_or(10).min(500)
    }

    pub fn get_all(&self) -> bool {
        self.get_all.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct KodeParam {
    pub kode: String,
}
