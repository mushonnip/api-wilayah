# api-wilayah

Indonesian administrative region API. Data embedded at compile time from BPS/Kemendagri CSV files.

## Stack

- Rust + [Ntex](https://ntex.rs) 3.x
- ~83K villages loaded into memory with O(1) lookup indexes
- Single self-contained binary, no external files needed at runtime

## Run

```bash
cp .env.example .env
cargo run
# release
cargo build --release && ./target/release/api-wilayah
```

Default port: `8989`. Override with `PORT=<n>`.

## Docker

```bash
docker build -t api-wilayah .
docker run -p 8989:8989 api-wilayah
```

---

## Endpoints

### Common query params

| Param | Default | Max | Description |
|-------|---------|-----|-------------|
| `page` | `0` | — | 0-based page |
| `size` | `10` | `500` | Items per page |
| `search` | — | — | Filter by name or kode (case-insensitive) |
| `get_all` | `false` | — | Get all items without pagination (ignores `page` and `size`) |

All list responses: `{ "data": [...], "total": N }` where `total` is count before pagination.  
Total also available as `x-total-count` response header.

---

### GET /v2/provinsi

```bash
curl "http://localhost:8989/v2/provinsi?size=2"
```

```json
{
  "data": [
    { "kode": "11", "nama": "Aceh",           "lat": 4.225728583038235,  "lng": 96.91187408609952 },
    { "kode": "12", "nama": "Sumatera Utara", "lat": 2.1884379790819697, "lng": 99.05805717982136 }
  ],
  "total": 38
}
```

```bash
curl "http://localhost:8989/v2/provinsi?search=jawa"
```

```json
{
  "data": [
    { "kode": "32", "nama": "Jawa Barat",  "lat": -6.920631357785788, "lng": 107.60331918557674 },
    { "kode": "33", "nama": "Jawa Tengah", "lat": -7.259392265876053, "lng": 110.2014881759208  },
    { "kode": "35", "nama": "Jawa Timur",  "lat": -7.719383482967257, "lng": 112.73244037323282 }
  ],
  "total": 3
}
```

---

### GET /v2/kab-kota

```bash
curl "http://localhost:8989/v2/kab-kota?size=2"
```

```json
{
  "data": [
    { "kode": "11.01", "nama": "Kabupaten Aceh Selatan",  "lat": 3.1618538408941346, "lng": 97.43651771865193, "kode_provinsi": "11" },
    { "kode": "11.02", "nama": "Kabupaten Aceh Tenggara", "lat": 3.368931368634976,   "lng": 97.69759716544476, "kode_provinsi": "11" }
  ],
  "total": 514
}
```

### GET /v2/{kode_provinsi}/kab-kota

Filter by province. Faster than global search — uses index, no full scan.

```bash
curl "http://localhost:8989/v2/32/kab-kota?search=bandung"
```

```json
{
  "data": [
    { "kode": "32.04", "nama": "Kabupaten Bandung",       "lat": -7.097588023276304, "lng": 107.60872729837905, "kode_provinsi": "32" },
    { "kode": "32.17", "nama": "Kabupaten Bandung Barat", "lat": -6.890437540720964, "lng": 107.41498197504157, "kode_provinsi": "32" },
    { "kode": "32.73", "nama": "Kota Bandung",            "lat": -6.919241381150077, "lng": 107.6366006270422,  "kode_provinsi": "32" }
  ],
  "total": 3
}
```

---

### GET /v2/kecamatan

```bash
curl "http://localhost:8989/v2/kecamatan?size=2"
```

```json
{
  "data": [
    { "kode": "11.01.01", "nama": "Bakongan",     "lat": 2.960325743420683, "lng": 97.46087307098534, "kode_kabupaten_kota": "11.01" },
    { "kode": "11.01.02", "nama": "Kluet Utara",  "lat": 3.123248075034301, "lng": 97.34619985236354, "kode_kabupaten_kota": "11.01" }
  ],
  "total": 7277
}
```

### GET /v2/{kode_kabupaten_kota}/kecamatan

```bash
curl "http://localhost:8989/v2/32.73/kecamatan?size=2"
```

```json
{
  "data": [
    { "kode": "32.73.01", "nama": "Sukasari", "lat": -6.866710075709871, "lng": 107.58716604539175, "kode_kabupaten_kota": "32.73" },
    { "kode": "32.73.02", "nama": "Coblong",  "lat": -6.884883447472015, "lng": 107.61538017670745, "kode_kabupaten_kota": "32.73" }
  ],
  "total": 30
}
```

---

### GET /v2/desa-kel

82K rows — always paginate.

```bash
curl "http://localhost:8989/v2/desa-kel?size=2"
```

```json
{
  "data": [
    { "kode": "11.01.01.2001", "nama": "Keude Bakongan", "lat": 2.931094803160483,  "lng": 97.48458404258515, "kode_kecamatan": "11.01.01", "kode_pos": "23773" },
    { "kode": "11.01.01.2002", "nama": "Ujong Mangki",   "lat": 2.9527245335971086, "lng": 97.43761867741745, "kode_kecamatan": "11.01.01", "kode_pos": "23773" }
  ],
  "total": 82978
}
```

### GET /v2/{kode_kecamatan}/desa-kel

```bash
curl "http://localhost:8989/v2/32.73.01/desa-kel"
```

```json
{
  "data": [
    { "kode": "32.73.01.1001", "nama": "Sukarasa",    "lat": -6.874227470295148, "lng": 107.58539617161965, "kode_kecamatan": "32.73.01", "kode_pos": "40152" },
    { "kode": "32.73.01.1002", "nama": "Gegerkalong", "lat": -6.869350446627759, "lng": 107.58886500774767, "kode_kecamatan": "32.73.01", "kode_pos": "40153" }
  ],
  "total": 4
}
```

---

### GET /v2/region/details

Resolve one or more codes to full hierarchical names. Accepts codes at any level, mixed.

```bash
# provinsi level
curl "http://localhost:8989/v2/region/details?ids=32"
```

```json
{ "data": [{ "kode": "32", "provinsi": "Jawa Barat" }] }
```

```bash
# kabupaten/kota level
curl "http://localhost:8989/v2/region/details?ids=32.73"
```

```json
{ "data": [{ "kode": "32.73", "kabupaten_kota": "Kota Bandung", "provinsi": "Jawa Barat" }] }
```

```bash
# kecamatan level
curl "http://localhost:8989/v2/region/details?ids=32.73.01"
```

```json
{ "data": [{ "kode": "32.73.01", "kecamatan": "Sukasari", "kabupaten_kota": "Kota Bandung", "provinsi": "Jawa Barat" }] }
```

```bash
# desa/kelurahan level
curl "http://localhost:8989/v2/region/details?ids=32.73.01.1001"
```

```json
{ "data": [{ "kode": "32.73.01.1001", "desa_kelurahan": "Sukarasa", "kecamatan": "Sukasari", "kabupaten_kota": "Kota Bandung", "provinsi": "Jawa Barat" }] }
```

```bash
# multiple codes at once (mixed levels)
curl "http://localhost:8989/v2/region/details?ids=11&ids=32.73"
```

```json
{
  "data": [
    { "kode": "11",    "provinsi": "Aceh" },
    { "kode": "32.73", "kabupaten_kota": "Kota Bandung", "provinsi": "Jawa Barat" }
  ]
}
```

#### Errors

```bash
# code not found → 404
curl "http://localhost:8989/v2/region/details?ids=99"
# → {"error":"provinsi kode 99 tidak ditemukan"}

# missing ids param → 400
curl "http://localhost:8989/v2/region/details"
# → {"error":"parameter 'ids' is required"}
```

---

## Code format

| Level | Format | Example |
|-------|--------|---------|
| Provinsi | `XX` | `32` |
| Kabupaten / Kota | `XX.XX` | `32.73` |
| Kecamatan | `XX.XX.XX` | `32.73.01` |
| Desa / Kelurahan | `XX.XX.XX.XXXX` | `32.73.01.1001` |
