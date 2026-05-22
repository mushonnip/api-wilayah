use std::collections::HashMap;
use serde::Deserialize;
use crate::models::*;

const PROVINSI_CSV: &str = include_str!("../data/1_provinsi.csv");
const KAB_KOTA_CSV: &str = include_str!("../data/2_kabupaten_kota.csv");
const KECAMATAN_CSV: &str = include_str!("../data/3_kecamatan.csv");
const DESA_KEL_CSV: &str = include_str!("../data/4_desa_kelurahan.csv");

#[derive(Deserialize)]
struct ProvinsiRow {
    kode: String,
    nama: String,
    lat: String,
    lng: String,
}

#[derive(Deserialize)]
struct KabKotaRow {
    kode: String,
    nama: String,
    lat: String,
    lng: String,
    kode_provinsi: String,
}

#[derive(Deserialize)]
struct KecamatanRow {
    kode: String,
    nama: String,
    lat: String,
    lng: String,
    kode_kabupaten_kota: String,
}

#[derive(Deserialize)]
struct DesaKelRow {
    kode: String,
    nama: String,
    lat: String,
    lng: String,
    kode_kecamatan: String,
    kode_pos: String,
}

pub struct RegionStore {
    pub provinsi: Vec<ProvinsiRecord>,
    pub kab_kota: Vec<KabupatenKotaRecord>,
    pub kecamatan: Vec<KecamatanRecord>,
    pub desa_kel: Vec<DesaKelurahanRecord>,

    // kode -> vec index for O(1) lookup
    provinsi_idx: HashMap<String, usize>,
    kab_kota_idx: HashMap<String, usize>,
    kecamatan_idx: HashMap<String, usize>,
    desa_kel_idx: HashMap<String, usize>,

    // parent_kode -> child indexes for O(1) filtered queries (no linear scan)
    kab_kota_by_prov: HashMap<String, Vec<usize>>,
    kecamatan_by_kab_kota: HashMap<String, Vec<usize>>,
    desa_kel_by_kecamatan: HashMap<String, Vec<usize>>,
}

impl RegionStore {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let provinsi = load_provinsi()?;
        let kab_kota = load_kab_kota()?;
        let kecamatan = load_kecamatan()?;
        let desa_kel = load_desa_kel()?;

        let mut provinsi_idx = HashMap::with_capacity(provinsi.len());
        for (i, r) in provinsi.iter().enumerate() {
            provinsi_idx.insert(r.kode.clone(), i);
        }

        let mut kab_kota_idx = HashMap::with_capacity(kab_kota.len());
        let mut kab_kota_by_prov: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, r) in kab_kota.iter().enumerate() {
            kab_kota_idx.insert(r.kode.clone(), i);
            kab_kota_by_prov.entry(r.kode_provinsi.clone()).or_default().push(i);
        }

        let mut kecamatan_idx = HashMap::with_capacity(kecamatan.len());
        let mut kecamatan_by_kab_kota: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, r) in kecamatan.iter().enumerate() {
            kecamatan_idx.insert(r.kode.clone(), i);
            kecamatan_by_kab_kota
                .entry(r.kode_kabupaten_kota.clone())
                .or_default()
                .push(i);
        }

        let mut desa_kel_idx = HashMap::with_capacity(desa_kel.len());
        let mut desa_kel_by_kecamatan: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, r) in desa_kel.iter().enumerate() {
            desa_kel_idx.insert(r.kode.clone(), i);
            desa_kel_by_kecamatan
                .entry(r.kode_kecamatan.clone())
                .or_default()
                .push(i);
        }

        Ok(RegionStore {
            provinsi,
            kab_kota,
            kecamatan,
            desa_kel,
            provinsi_idx,
            kab_kota_idx,
            kecamatan_idx,
            desa_kel_idx,
            kab_kota_by_prov,
            kecamatan_by_kab_kota,
            desa_kel_by_kecamatan,
        })
    }

    // --- Search (O(n), n bounded per level) ---

    pub fn search_provinsi(&self, term: &str) -> Vec<ProvinsiRecord> {
        let lower = term.to_lowercase();
        self.provinsi
            .iter()
            .filter(|r| {
                r.nama.to_lowercase().contains(&lower) || r.kode.contains(term)
            })
            .cloned()
            .collect()
    }

    pub fn search_kab_kota(&self, term: &str) -> Vec<KabupatenKotaRecord> {
        let lower = term.to_lowercase();
        self.kab_kota
            .iter()
            .filter(|r| {
                r.nama.to_lowercase().contains(&lower) || r.kode.contains(term)
            })
            .cloned()
            .collect()
    }

    pub fn search_kecamatan(&self, term: &str) -> Vec<KecamatanRecord> {
        let lower = term.to_lowercase();
        self.kecamatan
            .iter()
            .filter(|r| {
                r.nama.to_lowercase().contains(&lower) || r.kode.contains(term)
            })
            .cloned()
            .collect()
    }

    pub fn search_desa_kel(&self, term: &str) -> Vec<DesaKelurahanRecord> {
        let lower = term.to_lowercase();
        self.desa_kel
            .iter()
            .filter(|r| {
                r.nama.to_lowercase().contains(&lower) || r.kode.contains(term)
            })
            .cloned()
            .collect()
    }

    // --- Parent-filtered queries: O(result_size), no full scan ---

    pub fn get_kab_kota_by_prov(&self, kode_prov: &str) -> Vec<KabupatenKotaRecord> {
        self.kab_kota_by_prov
            .get(kode_prov)
            .map(|idxs| idxs.iter().map(|&i| self.kab_kota[i].clone()).collect())
            .unwrap_or_default()
    }

    pub fn get_kecamatan_by_kab_kota(&self, kode_kk: &str) -> Vec<KecamatanRecord> {
        self.kecamatan_by_kab_kota
            .get(kode_kk)
            .map(|idxs| idxs.iter().map(|&i| self.kecamatan[i].clone()).collect())
            .unwrap_or_default()
    }

    pub fn get_desa_kel_by_kecamatan(&self, kode_kec: &str) -> Vec<DesaKelurahanRecord> {
        self.desa_kel_by_kecamatan
            .get(kode_kec)
            .map(|idxs| idxs.iter().map(|&i| self.desa_kel[i].clone()).collect())
            .unwrap_or_default()
    }

    // --- Hierarchical detail lookup ---

    pub fn get_region_details(&self, codes: &[String]) -> Result<Vec<RegionDetail>, String> {
        if codes.is_empty() {
            return Err("tidak ditemukan data".to_string());
        }

        let mut results = Vec::with_capacity(codes.len());

        for code in codes {
            let code = code.trim().to_string();
            let level = code.split('.').count();

            let detail = match level {
                1 => {
                    let &idx = self
                        .provinsi_idx
                        .get(&code)
                        .ok_or_else(|| format!("provinsi kode {} tidak ditemukan", code))?;
                    RegionDetail {
                        kode: code,
                        provinsi: Some(self.provinsi[idx].nama.clone()),
                        kabupaten_kota: None,
                        kecamatan: None,
                        desa_kelurahan: None,
                    }
                }
                2 => {
                    let &idx = self
                        .kab_kota_idx
                        .get(&code)
                        .ok_or_else(|| format!("kabupaten/kota kode {} tidak ditemukan", code))?;
                    let kk = &self.kab_kota[idx];
                    let prov = self
                        .provinsi_idx
                        .get(&kk.kode_provinsi)
                        .map(|&i| self.provinsi[i].nama.clone());
                    RegionDetail {
                        kode: code,
                        provinsi: prov,
                        kabupaten_kota: Some(kk.nama.clone()),
                        kecamatan: None,
                        desa_kelurahan: None,
                    }
                }
                3 => {
                    let &idx = self
                        .kecamatan_idx
                        .get(&code)
                        .ok_or_else(|| format!("kecamatan kode {} tidak ditemukan", code))?;
                    let kec = &self.kecamatan[idx];
                    let (kk_nama, prov_nama) =
                        match self.kab_kota_idx.get(&kec.kode_kabupaten_kota) {
                            Some(&ki) => {
                                let kk = &self.kab_kota[ki];
                                let prov = self
                                    .provinsi_idx
                                    .get(&kk.kode_provinsi)
                                    .map(|&pi| self.provinsi[pi].nama.clone());
                                (Some(kk.nama.clone()), prov)
                            }
                            None => (None, None),
                        };
                    RegionDetail {
                        kode: code,
                        provinsi: prov_nama,
                        kabupaten_kota: kk_nama,
                        kecamatan: Some(kec.nama.clone()),
                        desa_kelurahan: None,
                    }
                }
                4 => {
                    let &idx = self
                        .desa_kel_idx
                        .get(&code)
                        .ok_or_else(|| format!("desa/kelurahan kode {} tidak ditemukan", code))?;
                    let desa = &self.desa_kel[idx];
                    let (kec_nama, kk_nama, prov_nama) =
                        match self.kecamatan_idx.get(&desa.kode_kecamatan) {
                            Some(&ki) => {
                                let kec = &self.kecamatan[ki];
                                match self.kab_kota_idx.get(&kec.kode_kabupaten_kota) {
                                    Some(&kki) => {
                                        let kk = &self.kab_kota[kki];
                                        let prov = self
                                            .provinsi_idx
                                            .get(&kk.kode_provinsi)
                                            .map(|&pi| self.provinsi[pi].nama.clone());
                                        (Some(kec.nama.clone()), Some(kk.nama.clone()), prov)
                                    }
                                    None => (Some(kec.nama.clone()), None, None),
                                }
                            }
                            None => (None, None, None),
                        };
                    RegionDetail {
                        kode: code,
                        provinsi: prov_nama,
                        kabupaten_kota: kk_nama,
                        kecamatan: kec_nama,
                        desa_kelurahan: Some(desa.nama.clone()),
                    }
                }
                _ => return Err(format!("invalid code format: {}", code)),
            };

            results.push(detail);
        }

        Ok(results)
    }
}

fn parse_float(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

fn load_provinsi() -> Result<Vec<ProvinsiRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(PROVINSI_CSV.as_bytes());
    let mut records = Vec::new();
    for result in rdr.deserialize::<ProvinsiRow>() {
        let row = result?;
        records.push(ProvinsiRecord {
            kode: row.kode,
            nama: row.nama,
            lat: parse_float(&row.lat),
            lng: parse_float(&row.lng),
        });
    }
    Ok(records)
}

fn load_kab_kota() -> Result<Vec<KabupatenKotaRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(KAB_KOTA_CSV.as_bytes());
    let mut records = Vec::new();
    for result in rdr.deserialize::<KabKotaRow>() {
        let row = result?;
        records.push(KabupatenKotaRecord {
            kode: row.kode,
            nama: row.nama,
            lat: parse_float(&row.lat),
            lng: parse_float(&row.lng),
            kode_provinsi: row.kode_provinsi,
        });
    }
    Ok(records)
}

fn load_kecamatan() -> Result<Vec<KecamatanRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(KECAMATAN_CSV.as_bytes());
    let mut records = Vec::new();
    for result in rdr.deserialize::<KecamatanRow>() {
        let row = result?;
        records.push(KecamatanRecord {
            kode: row.kode,
            nama: row.nama,
            lat: parse_float(&row.lat),
            lng: parse_float(&row.lng),
            kode_kabupaten_kota: row.kode_kabupaten_kota,
        });
    }
    Ok(records)
}

fn load_desa_kel() -> Result<Vec<DesaKelurahanRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(DESA_KEL_CSV.as_bytes());
    let mut records = Vec::new();
    for result in rdr.deserialize::<DesaKelRow>() {
        let row = result?;
        records.push(DesaKelurahanRecord {
            kode: row.kode,
            nama: row.nama,
            lat: parse_float(&row.lat),
            lng: parse_float(&row.lng),
            kode_kecamatan: row.kode_kecamatan,
            kode_pos: row.kode_pos,
        });
    }
    Ok(records)
}
