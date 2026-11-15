# Konfigurasi publik

`public.json` adalah satu satunya sumber nilai publik yang wajib sama di aplikasi macOS, Windows, dan Linux. `public.schema.json` dibuat generator untuk editor; jangan disunting dengan tangan. Aturan lengkap dan alasannya ada di [spec 0004](../../docs/specs/_root/0004-konfigurasi-publik/index.md).

## Skema versi 1

| Kunci | Tipe | Arti | Aturan |
| --- | --- | --- | --- |
| `$schema` | string | Path relatif ke skema hasil generator. | Tepat `./public.schema.json`. |
| `schemaVersion` | bilangan bulat | Versi skema yang dipakai file ini. | Sama dengan versi di baris terakhir "Riwayat skema". |
| `build.product_name` | string | Nama produk yang tampil kepada pengguna. Sama untuk semua bahasa. | Paling sedikit satu karakter. Tanpa karakter kontrol dan tanpa pemisah baris. Tidak diawali atau diakhiri whitespace. |
| `build.app_identifier` | string | Dasar pengenal aplikasi bergaya reverse DNS. | Paling banyak 100 karakter. Dua segmen atau lebih, dipisah satu titik. Tiap segmen diawali huruf ASCII, hanya memuat huruf ASCII, angka ASCII, dan tanda `-`, dan tidak diakhiri `-`. |

Semua kunci wajib ada. Kunci lain ditolak. Versi aplikasi tidak ada di sini; sumbernya `[workspace.package].version` di `Cargo.toml` root.

Belum ada nilai yang dibaca saat aplikasi berjalan dan belum ada definisi environment. Keduanya ditambah lewat pembaruan spec 0004 saat ada kebutuhan nyata.

## Cara aplikasi memakai nilai

| Nilai | Sumber | Dipakai saat | Tujuan di tiap OS |
| --- | --- | --- | --- |
| Nama produk | `build.product_name` | Build | Nama tampil aplikasi. Setting native ditetapkan spec OS. |
| Dasar pengenal aplikasi | `build.app_identifier` | Build | Pengenal aplikasi tiap OS, menurut `K-6`. |
| Versi aplikasi | `[workspace.package].version` di `Cargo.toml` root | Build | Versi yang tampil dan tercatat di aplikasi. |

- **K-1**: Nilai publik bersama hanya ditulis tangan di sumbernya, yaitu `public.json` atau `Cargo.toml` root untuk versi.
- **K-2**: Setiap salinan nilai itu di luar sumbernya wajib dibuat generator, atau dibandingkan dengan sumbernya oleh pemeriksaan otomatis. Spec OS memilih salah satunya dan menulis perintah buktinya.
- **K-3**: Semua nilai dipakai saat build. Aplikasi tidak membuka `shared/config/` saat berjalan, dan folder ini tidak ikut dikemas.
- **K-4**: Rust Core dan jembatan macOS tidak membaca konfigurasi. Bila Core kelak butuh sebuah nilai, aplikasi mengirimnya sebagai argumen API publik.
- **K-5**: Aplikasi Rust mengambil versi dari Cargo saat kompilasi. Aplikasi yang proyeknya bukan Cargo mengikuti `K-2`.
- **K-6**: Pengenal aplikasi tiap OS diturunkan dari `build.app_identifier` dengan aturan tetap yang spec OS tulis. Aturan bawaan: sama dengan nilai itu, atau nilai itu ditambah titik dan satu segmen. Bila format native menolak nilai itu, spec OS menulis pemetaan tetap dari nilai itu. Tidak ada pengenal kedua yang ditulis tangan.
- **K-7**: Tidak ada file timpaan dan tidak ada variabel lingkungan yang mengubah nilai publik. Satu nilai berlaku untuk semua orang dan semua build.

## Batas pengaturan khusus OS

Tiga pertanyaan, dijawab berurutan:

1. Apakah nilai itu rahasia, atau hanya benar di satu mesin atau untuk satu orang? Bila ya, nilai itu tidak masuk repo.
2. Apakah lebih dari satu aplikasi butuh nilai itu dengan makna yang sama? Bila ya, nilai itu masuk `public.json` lewat pembaruan spec 0004.
3. Selain itu, nilai itu pengaturan khusus OS dan tinggal di proyek native aplikasi itu.

| OS | Contoh pengaturan khusus OS |
| --- | --- |
| macOS | Versi minimum macOS, nomor build, nama target, gaya penandatanganan, sandbox, entitlement, kategori aplikasi, aturan turunan pengenal bundel. |
| Windows | Manifest aplikasi, identitas paket, versi Windows App SDK dan versi minimum Windows, aturan turunan pengenal aplikasi. |
| Linux | Isi berkas `.desktop` selain nama, kategori menu, versi minimum GTK dan libadwaita, lokasi ikon, aturan turunan pengenal aplikasi. |

Debug dan Release adalah konfigurasi build tiap toolchain, bukan environment.

## Yang tidak boleh ada di folder ini

| Kelas | Arti | Tempat | Dilacak git |
| --- | --- | --- | --- |
| Nilai publik bersama | Boleh dilihat siapa pun dan wajib sama maknanya di lebih dari satu aplikasi. | `shared/config/public.json`. Versi aplikasi di `Cargo.toml` root. | Ya |
| Pengaturan khusus OS | Hanya dimengerti satu OS, atau tiap OS punya nilainya sendiri. | Proyek native di `apps/<os>/`. | Ya |
| Pengaturan lokal | Hanya berlaku di satu mesin atau untuk satu orang. | File yang diabaikan git atau variabel lingkungan shell. | Tidak |
| Rahasia | Memberi akses atau membuktikan identitas. | Keychain di mesin pengembang atau penyimpanan rahasia CI. Tidak pernah di repo. | Tidak |

Hanya kelas pertama yang boleh ada di folder ini. Folder ini berisi tepat tiga file: `public.json`, `public.schema.json`, dan `README.md`. File atau subfolder lain membuat validasi gagal, kecuali `.DS_Store` yang diabaikan.

## Cara mengubah

- Memeriksa: `cargo run -p desktop-codegen --locked -- config check` dari root repo.
- Mengganti nilai: sunting `public.json`, jalankan pemeriksaan di atas, lalu jalankan generator atau pemeriksaan tiap OS supaya salinannya ikut. Versi skema tidak naik.
- Mengubah skema (menambah, menghapus, atau mengganti nama kunci, atau mengubah aturan): perbarui spec 0004 lebih dulu, lalu dalam satu commit ubah aturan dan versi di `tooling/codegen/src/config/mod.rs`, naikkan `schemaVersion` di `public.json` sebesar satu, tambah satu baris di "Riwayat skema", perbarui tabel "Skema versi" di atas, jalankan `cargo run -p desktop-codegen --locked -- config generate`, dan sesuaikan setiap konsumen.

## Riwayat skema

| Versi | Tanggal | Perubahan | Spec |
| --- | --- | --- | --- |
| 1 | 2026-11-15 | Skema pertama: `build.product_name` dan `build.app_identifier`. | 0004 |
