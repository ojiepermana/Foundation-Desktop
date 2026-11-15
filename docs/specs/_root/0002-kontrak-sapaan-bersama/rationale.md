# 0002. Kontrak sapaan bersama: catatan keputusan

Berkas ini menyimpan alasan di balik [spec 0002](index.md). Kamu tidak perlu membacanya untuk membangun. Bacalah saat ingin tahu mengapa sebuah pilihan diambil, atau saat spec ini akan diubah.

## Context

Fondasi ini punya tiga antarmuka native dan satu sumber perilaku. Sapaan adalah perilaku pertama yang melintasi batas itu: pengguna mengetik nama, menekan tombol, lalu melihat teks yang dibuat Rust Core. Kalau bentuk fungsi itu tidak ditetapkan sekarang, tiap spec OS (004, 006, 007) akan menebak sendiri, dan tiga tebakan yang berbeda berarti tiga perilaku.

Beberapa hal sudah tetap sebelum spec ini ditulis:

* [Kontrak Rust](../../../contracts/README.md) menetapkan bentuk fungsi: `pub fn hello(name: String) -> String` dengan hasil `Hello, {name}!`. Dokumen itu juga menyebut bahwa format hasil mengikuti kontrak, sedangkan teks antarmuka seperti label tombol boleh memakai katalog bahasa.
* [Arsitektur](../../../architecture/README.md) menjadikan Core pemilik API publik, validasi, dan makna kesalahan. Core harus bisa dibangun dan diuji tanpa pustaka UI.
* macOS memanggil Core lewat binding Swift hasil UniFFI (alat yang membuat kode Swift dari API Rust). Windows dan Linux memanggil Core langsung sebagai dependensi Rust. Jadi bentuk API harus bisa dipakai lewat dua jalur itu tanpa diubah.
* Batas fase fondasi dan kriteria 32 melarang layanan untuk fitur mendatang. Yang dibangun cukup satu fungsi sapaan.

Yang belum tetap, dan menjadi isi spec ini:

* Perilaku untuk masukan di tepi: nama kosong, hanya spasi, Unicode, dan sangat panjang. Kontrak hanya memberi contoh `Ayu`.
* Apakah ada validasi, dan kalau ada, bagaimana kesalahannya sampai ke tiga UI.
* Di modul mana fungsi tinggal, dan apa saja yang terlihat dari luar crate.
* Aturan untuk kontrak publik berikutnya (kriteria 22): kompatibilitas, versi, dan siapa yang boleh menambah API.
* Uji yang membuktikan semuanya tanpa membuka jendela (kriteria 2 dan 3).

Kalau hal hal itu dibiarkan terbuka, developer Core akan memilih sendiri saat menulis kode, dan architect tiap OS akan menulis spec di atas asumsi yang belum tentu sama.

## Options considered

Keputusan utama adalah perilaku `hello` untuk masukan apa pun. Keputusan lain yang lebih kecil ada di tabel sesudah bagian ini.

### Option 1: Fungsi total, nama dipakai apa adanya

`hello` menerima setiap `String` dan selalu mengembalikan `Hello, ` lalu nama persis seperti yang diterima lalu `!`. Tidak ada pemangkasan, normalisasi, batas panjang, atau kesalahan. Nama kosong menghasilkan `Hello, !`.

**Pros**:
* Sama persis dengan bentuk di dokumen kontrak, jadi tidak ada dokumen yang perlu diubah.
* Tidak ada tipe kesalahan, sehingga jembatan macOS tidak perlu memetakan kesalahan Rust ke Swift untuk alur pertama.
* Perilaku bisa dijelaskan dalam satu kalimat dan diuji dengan tabel kecil. Tiga UI tidak punya ruang untuk berbeda.

**Cons**:
* Nama kosong atau hanya spasi memberi hasil yang kurang enak dilihat (`Hello, !`).
* Kalau produk kelak ingin menolak nama tertentu, itu perubahan yang memutus kontrak dan menyentuh ketiga UI.

### Option 2: Validasi di Core dengan kesalahan bertipe

`hello` memangkas spasi, menolak nama kosong, dan membatasi panjang. Tanda tangan menjadi `Result<String, HelloError>` dengan enum kesalahan milik Core.

**Pros**:
* Aturan nama yang sah tinggal di Core, sesuai peran Core sebagai pemilik validasi.
* UI tidak pernah menampilkan sapaan untuk nama kosong.

**Cons**:
* Mengubah tanda tangan yang sudah ditetapkan dokumen kontrak, dan dokumen itu tidak boleh diubah dari sesi ini.
* Menambah tipe kesalahan, pemetaan kesalahan lewat UniFFI, dan tampilan kesalahan di tiga UI sebelum alur pertama terbukti. Itu layanan untuk kebutuhan yang belum diminta siapa pun (kriteria 32).
* Batas panjang dan definisi "kosong" untuk Unicode adalah aturan produk yang belum ada pemiliknya.

### Option 3: Rapikan diam diam, tanda tangan tetap

Tanda tangan tetap `String` ke `String`, tetapi Core memangkas spasi di tepi dan memakai nama pengganti (misalnya `World`) saat nama kosong.

**Pros**:
* Hasil selalu enak dilihat, tanpa tipe kesalahan.
* Tanda tangan sama dengan dokumen kontrak.

**Cons**:
* Hasil tidak lagi `Hello, {name}!` untuk semua masukan, jadi isi dokumen kontrak tidak lagi benar.
* Nama pengganti adalah teks antarmuka yang akan butuh lokalisasi, padahal lokalisasi milik scope 010.
* Pengguna yang sengaja mengetik spasi melihat masukan mereka diubah tanpa penjelasan. Perilaku yang diam diam mengubah data sulit dicabut sesudah UI bergantung padanya.

### Keputusan yang lebih kecil

| Pertanyaan | Diambil | Mengapa | Pilihan kedua |
| --- | --- | --- | --- |
| Di mana fungsi tinggal dan lewat path apa ia terlihat? | Modul privat `greeting`, diekspor ulang di akar crate | Akar crate menjadi satu satunya permukaan publik, jadi isi crate bisa ditata ulang tanpa memutus pemanggil. | Menulis `hello` langsung di `lib.rs`. Lebih sedikit berkas, tetapi `lib.rs` menjadi tempat menumpuk perilaku. |
| Bagaimana versi kontrak dicatat? | Versi paket Core mengikuti SemVer, ditambah tabel riwayat di spec | Tidak ada API baru hanya untuk versi. Semua pemanggil ada di workspace yang sama, jadi versi berfungsi sebagai catatan. | Tanpa versi sampai ada rilis pertama. Lebih ringan, tetapi dokumen kontrak meminta kompatibilitas dan versi dicatat. |
| Bagaimana tanda tangan dijaga agar tidak berubah diam diam? | Uji kontrak mengikat `hello` ke tipe `fn(String) -> String` saat kompilasi | Tanpa alat tambahan. Perubahan tanda tangan langsung gagal dibangun. | `cargo-semver-checks` atau snapshot `cargo public-api`. Lebih lengkap untuk API besar, berlebihan untuk satu fungsi. |
| Di mana vektor kontrak disimpan? | Tabel di spec (normatif) dan tabel yang sama di uji integrasi Core | Uji berjalan dengan `cargo test` saja. Spec OS cukup merujuk ID vektor. | Berkas fixture bersama di `tests/fixtures/` yang dibaca uji tiap OS. Berguna saat pemanggil non Rust perlu membaca vektor, tetapi butuh parser dan format yang belum diperlukan. |
| Alat uji apa yang dipakai? | Harness bawaan `cargo test` dengan uji tabel, tanpa dependensi dev | Core tetap tanpa dependensi, dan vektor tepi sudah mencakup kelas masukan yang penting. | `proptest` (uji berbasis properti). Menemukan kasus acak, tetapi menambah dependensi untuk fungsi tiga bagian. |
| Siapa yang boleh menambah atau mengubah API publik Core? | Siapa pun, lewat spec `_root` yang diterima lebih dulu | Setiap API publik menyentuh tiga UI, jadi keputusannya perlu tercatat sebelum kode. | Perubahan aditif cukup lewat tinjauan kode. Lebih cepat, tetapi keputusan bersama jadi tersebar di riwayat commit. |
| Apa yang boleh UI lakukan pada nama dan hasil? | Kirim isi kolom apa adanya, tampilkan hasil apa adanya, termasuk untuk nama kosong | Aturan tentang nama yang sah hanya boleh ada di satu tempat. | UI boleh menonaktifkan tombol saat kolom kosong. Tampak lebih rapi, tetapi itu aturan validasi yang ditulis tiga kali. |
| Bagaimana "satu implementasi" dibuktikan sekarang? | Pencarian `git grep` untuk pola format sapaan, dan uji memakai nilai harapan literal | Bisa dijalankan hari ini tanpa skrip baru. | Skrip pemeriksa di `scripts/`. Lebih tahan lama, tetapi pemeriksaan arsitektur adalah milik scope 002 dan `scripts/` punya pemilik lain di wave ini. |
| Bagaimana badan fungsi ditulis? | `format!("Hello, {name}!")`, seperti di dokumen kontrak | Satu baris yang bisa dibandingkan langsung dengan kontrak. | Menyisipkan ke buffer `name` yang sudah ada agar hemat satu alokasi. Belum ada ukuran yang menunjukkan alokasi itu masalah. |

## Rationale

Option 1 dipilih karena tiga hal dari Context.

Pertama, bentuk fungsi sudah ditetapkan di dokumen kontrak, termasuk badan fungsinya. Option 2 mengubah tanda tangan, dan Option 3 membuat kalimat "format hasil mengikuti kontrak" tidak lagi benar. Hanya Option 1 yang membuat dokumen, kode, dan uji mengatakan hal yang sama.

Kedua, tujuan fase ini adalah membuktikan arah dependensi lewat satu alur kecil. Tipe kesalahan berarti pemetaan kesalahan di jembatan macOS dan tampilan kesalahan di tiga UI, sebelum satu jendela pun menampilkan sapaan. Itu pekerjaan nyata untuk aturan produk yang belum diminta. Saat aturan itu diminta, spec ini sudah menyebut jalannya: fungsi yang bisa gagal mengembalikan `Result` dengan kesalahan bertipe milik Core (aturan R-4).

Ketiga, perilaku "apa adanya" adalah satu satunya yang tidak memberi ruang bagi UI untuk berbeda. Karena itu spec ini juga melarang UI memangkas atau menolak nama (aturan R-8). Kalau larangan itu terasa terlalu ketat untuk nama kosong, yang diubah adalah Core, sekali, lewat spec ini.

Harga yang dibayar jelas: `Hello, !` adalah hasil yang sah. Untuk fondasi yang tugasnya membuktikan jalur, hasil yang jujur lebih berguna daripada hasil yang rapi.

Modul privat dengan ekspor ulang di akar crate dipilih karena kriteria 22 meminta aturan kontrak publik, dan aturan paling sederhana adalah "yang publik hanya yang ada di akar crate". Aturan nilai yang dimiliki (`String`, bukan `&str`) berasal dari dokumen kontrak dan dari jalur macOS: nilai yang melintasi jembatan selalu disalin, jadi satu tanda tangan dengan nilai yang dimiliki memberi ketiga pemanggil bentuk yang sama.

Core tanpa dependensi adalah keadaan untuk kontrak ini, bukan janji selamanya. Tipe publik berikutnya (enum kesalahan, record) mungkin butuh turunan UniFFI agar jembatan tidak membuat tipe cermin. Keputusan itu milik spec yang menambah tipe tersebut.
