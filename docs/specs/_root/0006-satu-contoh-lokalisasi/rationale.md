# 0006. Satu contoh lokalisasi: catatan keputusan

Berkas ini menyimpan alasan di balik [spec 0006](index.md). Kamu tidak perlu membacanya untuk membangun. Bacalah saat ingin tahu mengapa sebuah pilihan diambil, atau saat spec ini akan diubah.

## Context

Fondasi ini punya tiga antarmuka native dan satu sumber untuk setiap hal yang dipakai bersama. Untuk teks antarmuka, sumber itu belum ada. Teks di layar sapaan macOS sekarang ditulis langsung di kode Swift dalam bahasa Inggris: `Desktop Foundation`, `Name`, dan `Greet`. Aplikasi Windows dan Linux belum ada, dan saat dibuat masing masing akan menulis teksnya sendiri kecuali ada aturan lebih dulu.

Beberapa hal sudah tetap sebelum spec ini ditulis:

* [Resource bersama](../../../shared-resources/README.md) menempatkan katalog di `shared/localization/` dan menyebut isi konvensinya: kunci stabil, nama locale, fallback, placeholder yang harus cocok, dan deteksi entri hilang. Dokumen itu juga menyebut bahwa satu contoh teks cukup, bahwa format angka, tanggal, dan bentuk jamak mengikuti OS, dan bentuk keluaran tiap OS: resource lokalisasi Apple, adapter lokalisasi Rust untuk WinUI, dan resource yang sesuai GTK.
* Berkas fitur scope 010 meminta satu teks antarmuka di layar sapaan, dan baris Done when meminta ketiga aplikasi menampilkan teks dari kunci yang sama.
* Spec `_root/0002` menetapkan bahwa hasil `hello` selalu bahasa Inggris, bukan teks antarmuka, dan ditampilkan apa adanya (R-8). Mengubahnya adalah perubahan kontrak.
* [Arsitektur](../../../architecture/README.md) menjadikan Core bebas dari pustaka UI dan dependensi yang tidak perlu. Core sekarang tanpa dependensi sama sekali.
* Kriteria 32 dan "Batas fase fondasi" melarang layanan untuk fitur mendatang.
* Bentuk generator dan validasi sumber bersama adalah keputusan scope 008 (spec `_root/0003`), yang ditulis bersamaan dengan spec ini. Spec itu menetapkan satu paket `desktop-codegen` dengan dua aksi, `check` dan `generate`, satu modul per sumber, keluaran yang dilacak git, dan uji kontrak tanpa folder fixture.
* Hanya host Mac yang tersedia. Bukti untuk Windows dan Linux menunggu host.

Yang belum tetap, dan menjadi isi spec ini:

* Format dan bentuk berkas katalog, dan apakah satu berkas memuat semua bahasa.
* Tata tulis kunci, nama locale, dan placeholder yang bisa diperiksa mesin.
* Apa arti "hilang", apa yang terjadi saat terjemahan hilang, dan di mana hal itu menggagalkan pemeriksaan.
* Apa yang wajib ada di resource tiap OS, supaya tiga emitter tidak berbeda perilaku.
* Teks mana yang menjadi contoh, dan bahasa apa yang dipakai.

Kalau hal hal itu dibiarkan terbuka, spec macOS akan memilih bentuk yang paling enak untuk Apple, dan spec Windows serta Linux kelak mewarisi atau menabraknya. Tata tulis placeholder dan aturan fallback paling rawan: tiga OS punya tiga kebiasaan, dan perbedaan kecil di sana baru terlihat saat teks kedua masuk.

Bahasa yang didukung dan teks terjemahan yang disetujui adalah keputusan produk yang hanya user tahu. Spec ini hanya bisa mengusulkan.

## Options considered

Keputusan utama adalah di mana teks ditulis dan bagaimana ia sampai ke tiap OS. Keputusan lain yang lebih kecil ada di tabel sesudah bagian ini.

### Option 1: Satu katalog JSON netral untuk semua bahasa, resource native dibuat dari katalog itu

Satu berkas `catalog.json` memuat setiap kunci dengan deskripsi dan teksnya dalam semua bahasa. Alat sumber bersama memeriksa berkas itu dan menulis resource native tiap OS.

**Pros**:
* Menambah kunci adalah satu perubahan di satu tempat, dan kelengkapan semua bahasa terlihat dalam satu berkas.
* Bentuknya tidak memihak satu OS, jadi aturan placeholder dan fallback ditulis sekali dan berlaku sama di tiga emitter.
* Sama dengan pola sumber token di spec `_root/0003`: JSON, tipe tertutup, skema hasil generator. Tidak ada alat atau dependensi baru.

**Cons**:
* Setiap OS butuh satu emitter kecil dan satu berkas turunan.
* Semua bahasa di satu berkas menyulitkan pembagian kerja ke banyak penerjemah dan menambah peluang konflik merge saat bahasa bertambah.
* JSON tidak punya komentar, jadi catatan penerjemah harus menjadi anggota `description`.

### Option 2: Satu berkas JSON per bahasa

`shared/localization/en.json`, `id.json`, dan seterusnya, masing masing peta dari kunci ke teks. Deskripsi dan daftar bahasa tinggal di berkas bahasa sumber atau di berkas manifest terpisah.

**Pros**:
* Tiap penerjemah memegang satu berkas, dan menambah bahasa adalah menambah berkas.
* Konflik merge antar bahasa hilang.

**Cons**:
* Menambah satu kunci menyentuh sebanyak berkas bahasa yang ada, dan kelengkapan hanya terlihat lewat alat.
* Butuh berkas ketiga atau aturan khusus untuk deskripsi, daftar bahasa, dan versi, jadi "satu sumber" menjadi beberapa berkas yang harus cocok.
* Untuk satu kunci dan dua bahasa, pemecahan ini belum punya masalah untuk dipecahkan.

### Option 3: Format native satu OS sebagai sumber bersama

Sumber ditulis sebagai String Catalog Apple (`.xcstrings`) atau berkas gettext (`.po`), lalu dikonversi untuk dua OS lain.

**Pros**:
* Satu OS memakai sumber langsung tanpa emitter, dan editor bawaan OS itu bisa dipakai.
* Format itu sudah punya tempat untuk komentar, status terjemahan, dan bentuk jamak.

**Cons**:
* Tata tulis placeholder dan bentuk jamak milik satu OS menjadi aturan bersama, padahal dokumen resource bersama meminta keduanya mengikuti OS masing masing.
* Sumber bersama akan tinggal di bawah `shared/` tetapi dimiliki kebiasaan satu toolkit. Perubahan format oleh vendor itu menjadi perubahan untuk tiga aplikasi.
* Dua OS lain tetap butuh emitter, jadi penghematannya hanya satu emitter.

### Option 4: Pustaka lokalisasi Rust yang dipakai ketiga aplikasi saat berjalan

Teks ditulis dalam format pustaka Rust (misalnya Fluent) dan dibaca saat aplikasi berjalan lewat satu crate bersama. macOS memanggilnya lewat jembatan UniFFI.

**Pros**:
* Satu jalur kode untuk memilih bahasa dan mengisi placeholder, jadi perilaku tiga aplikasi sama persis.
* Tidak ada berkas turunan per OS.

**Cons**:
* Bertentangan dengan tabel target di dokumen resource bersama: macOS memakai resource lokalisasi Apple dan Linux memakai resource yang sesuai GTK.
* Pilihan bahasa, bentuk jamak, dan format angka berpindah dari OS ke pustaka, sehingga aplikasi tidak lagi mengikuti pengaturan dan kebiasaan OS dengan sendirinya.
* Menambah crate, dependensi, dan API jembatan baru sebelum ada teks kedua. Itu layanan untuk kebutuhan mendatang (kriteria 32).

### Keputusan yang lebih kecil

Kolom tanda memakai istilah koordinasi: `berat` berarti mahal dibalik sesudah dibangun.

| Pertanyaan | Diambil | Mengapa | Pilihan kedua | Tanda |
| --- | --- | --- | --- | --- |
| Teks mana yang menjadi contoh? | Label tombol, `greeting.submit` | Dokumen kontrak menyebut label tombol sebagai contoh teks yang boleh memakai katalog. Judul jendela adalah nama produk, dan spec `_root/0004` menetapkan nama produk sebagai nilai konfigurasi publik yang tidak diterjemahkan. | Dua teks, `Name` dan `Greet` (baris Follow-up spec macos/0001 menyebut tiga, tetapi judul adalah nama produk). Kolom dan tombol menjadi satu bahasa, tetapi berkas fitur dan dokumen resource bersama meminta satu teks. Bila ini yang dipilih, katalog, tabel Nilai hasil, AC-1, AC-2, dan OS-1 serta OS-4 berubah. | biasa |
| Bahasa apa yang dipakai? | Sumber `en`, contoh kedua `id` | Teks yang ada sekarang berbahasa Inggris, jadi bahasa sumber tidak mengubah tampilan yang sudah diverifikasi. Dokumen repo berbahasa Indonesia, jadi `id` adalah bahasa kedua yang paling mungkin dibutuhkan. | Sumber `id`, contoh kedua `en`. Cocok bila pengguna utama berbahasa Indonesia, tetapi mengubah teks bawaan yang sudah lulus verify. Bila ini yang dipilih, katalog, AC-1, kasus uji, dan OS-4 berubah. | biasa |
| Format berkas katalog? | JSON dengan `$schema` dan `schemaVersion`, sama dengan sumber token | Satu pola untuk semua sumber bersama, tanpa parser baru di alat. | TOML. Lebih enak ditulis tangan dan punya komentar, tetapi butuh dependensi baru di `desktop-codegen` dan menjadi pola kedua. Bila ini yang dipilih, isi berkas, AC-1, AC-8, dan semua kasus uji berubah bentuk. | berat |
| Bentuk kunci? | Segmen huruf kecil dan angka dipisah titik, tanpa garis bawah | Titik bisa diganti garis bawah untuk format yang tidak menerima titik (L-11 butir 2) tanpa dua kunci menjadi sama. Aturan yang lebih sempit mudah dilonggarkan kelak, sebaliknya tidak. | Segmen `snake_case` dipisah titik, ditambah pemeriksaan tabrakan seperti aturan T-3 spec `_root/0003`. Kunci lebih enak dibaca, tetapi butuh satu aturan dan satu pesan lagi. Bila ini yang dipilih, L-3, skema, dan kasus 7 sampai 11 berubah. | berat |
| Kunci menyebut apa? | Peran elemen (`greeting.submit`) | Kata kata boleh berubah tanpa mengganti kunci. | Isi teks bahasa sumber sebagai kunci, seperti kebiasaan gettext. Tanpa penamaan, tetapi setiap perbaikan ejaan memutus semua terjemahan. | berat |
| Tata tulis placeholder? | Bernama, `{nama}`, nilai selalu teks, tanpa escape | Nama membuat penerjemah bebas mengubah urutan dan tetap tahu arti tiap bagian. Nilai yang selalu teks membuat angka dan tanggal diformat OS. Tanpa escape berarti tanpa aturan yang harus ditiru tiga emitter. | Bernomor urut gaya `printf` (`%1$s`). Dekat dengan Apple dan gettext, tetapi penerjemah tidak tahu arti tiap nomor dan tanda `%` literal menjadi masalah di semua bahasa. Bila ini yang dipilih, L-7, K-1, dan kasus 23 sampai 30 berubah. | berat |
| Nama locale seluas apa? | Bahasa, lalu wilayah bila perlu | Cukup untuk `en` dan `id`, dan setiap nama punya padanan yang jelas di Apple, Windows, dan gettext. Aksara dan wilayah berupa angka tidak punya padanan satu ke satu di gettext. | Bahasa, aksara, dan wilayah (`zh-Hant-TW`, `es-419`). Lebih lengkap, tetapi pemetaan ke nama OS bisa bertabrakan dan belum ada bahasa yang membutuhkannya. Bila ini yang dipilih, L-4, skema, dan kasus 14 berubah. | biasa |
| Apakah terjemahan yang hilang menggagalkan `check`? | Tidak. Hilang dihitung dan disebut di ringkasan, resource tetap ditulis dengan teks bahasa sumber, dan uji gerbang menolak katalog repo yang belum lengkap | Developer bisa membangun aplikasi sebelum terjemahan datang, tetapi `main` tetap lengkap. Polanya sama dengan nilai `placeholder` di token: dilaporkan, tidak menggagalkan alat. | Terjemahan yang hilang adalah masalah seperti yang lain, jadi `check` dan `generate` gagal. Satu aturan lebih sedikit, tetapi fallback per teks tidak pernah terpakai dan aplikasi tidak bisa dibangun dengan katalog yang belum lengkap. Bila ini yang dipilih, L-8, L-9, tipe `Origin` dan `Missing`, AC-6, AC-7, dan K-2 berubah. | biasa |
| Di mana fallback per teks dikerjakan? | Di alat, di dalam `parse` | Tiga OS punya tiga perilaku saat sebuah kunci tidak ada di satu bahasa, dan sebagian menampilkan kunci mentah. `Catalog` yang selalu lengkap membuat perilakunya sama. | Diserahkan ke mekanisme fallback tiap OS. Resource lebih kecil, tetapi perilaku berbeda antar OS dan sulit diuji tanpa jendela. Bila ini yang dipilih, L-9, L-11, dan AC-7 berubah. | biasa |
| Kapan sebuah teks dihitung hilang? | Saat locale yang terdaftar tidak punya teks sendiri | Aturan paling sederhana, dan belum ada bahasa wilayah yang perlu mewarisi teks induknya. Menambah pewarisan kelak tidak memutus katalog yang sudah lengkap. | Teks baru dihitung hilang bila locale itu dan induknya yang terdaftar sama sama tidak punya teks (`pt-BR` mewarisi `pt`). Hemat untuk bahasa wilayah, tetapi menambah rantai induk di alat dan di aturan sebelum ada yang memakainya. Bila ini yang dipilih, L-8, L-9, dan K-2 berubah. | biasa |
| Siapa yang memilih bahasa untuk pengguna? | OS, dengan hasil minimum yang tertulis. Bentuk resource tanpa negosiasi menerapkan langkah minimum itu sendiri. | Aplikasi native mengikuti pengaturan bahasa OS. Hasil minimum cukup untuk diuji sama di tiga OS, dan menutup kasus gettext dan adapter Rust yang tidak punya bahasa bawaan sendiri. | Satu algoritme pemilihan yang wajib dipakai ketiga aplikasi. Perilaku sama persis, tetapi mengabaikan aturan OS seperti bahasa per aplikasi di macOS. | biasa |
| Apa arti versi? | `schemaVersion` untuk bentuk berkas, riwayat isi lewat git dan tabel di spec | Sama dengan sumber token. Alat perlu tahu bentuk berkas sebelum membacanya, dan git sudah mencatat isi. | Satu nomor versi SemVer untuk bentuk dan isi sekaligus. Satu angka untuk dikutip, tetapi alat tidak bisa membedakan "bentuk baru" dari "kunci dihapus". Bila ini yang dipilih, L-10 dan kasus 1 berubah. | berat |
| Apakah deskripsi wajib? | Ya, dengan pola deskripsi spec `_root/0003` | Penerjemah yang tidak melihat layar butuh tahu peran teks. Pola yang sama membuat deskripsi aman ditulis sebagai komentar di berkas hasil emitter. | Opsional. Lebih ringan untuk developer, tetapi kebanyakan entri akan kosong. | biasa |
| Apakah ada berkas skema untuk katalog? | Ya, `catalog.schema.json` sebagai keluaran generator | Sama dengan token: editor memeriksa bentuk saat mengetik, dan skema tidak bisa berbeda dari `parse` karena dibangun dari konstanta yang sama. | Tanpa skema. Satu berkas lebih sedikit, tetapi `$schema` yang diwajibkan pola sumber tidak punya tujuan. | biasa |
| Bagaimana aturan dibuktikan tanpa OS? | `parse` mengembalikan `Catalog` yang sudah diisi fallback, dan uji kontrak memeriksanya | Fallback dan deteksi bisa dibuktikan di Mac tanpa emitter OS mana pun, tanpa aksi ketiga di alat dan tanpa berkas turunan tambahan. Tiga emitter kelak memakai `Catalog` yang sama. | Tabel hasil sebagai keluaran `generate` yang dilacak git (saran sesi scope 008). Peninjau bisa melihat teks tiap bahasa di satu berkas, tetapi itu berkas turunan yang tidak dipakai aplikasi mana pun. Bila ini yang dipilih, AC-2, AC-9, dan angka `1 outputs` berubah. | biasa |
| Di mana aturan ditulis untuk pembaca? | Di spec ini, ditautkan dari `shared/localization/README.md` dan dokumen resource bersama | Satu tempat untuk aturan. Dokumen rujukan tetap pendek. | Menyalin aturan ke `docs/shared-resources/README.md`. Lebih mudah ditemukan, tetapi dua salinan akan berbeda. | biasa |

## Rationale

Option 1 dipilih karena empat hal dari Context.

Pertama, dokumen resource bersama sudah menetapkan tiga bentuk keluaran yang berbeda dan meminta bentuk jamak serta format mengikuti OS. Itu menyingkirkan Option 4, yang memindahkan semuanya ke satu pustaka, dan melemahkan Option 3, yang menjadikan kebiasaan satu OS sebagai aturan bersama. Sumber yang netral dengan tata tulis yang sempit (placeholder bernama, nilai selalu teks, tanpa bentuk jamak) adalah satu satunya bentuk yang bisa diterjemahkan ke tiga format native tanpa kehilangan makna.

Kedua, Core harus tetap tanpa dependensi dan `hello` tidak boleh berubah. Katalog yang hanya dibaca alat saat build tidak menyentuh Core sama sekali. Option 4 butuh crate dan API jembatan baru.

Ketiga, hanya ada satu teks dan dua bahasa. Option 2 memecahkan masalah banyak penerjemah yang belum ada, dengan harga beberapa berkas yang harus cocok. Bila masalah itu datang, pemecahan per bahasa adalah perubahan `schemaVersion` dan satu konversi otomatis, karena kunci dan aturannya tidak berubah.

Keempat, spec `_root/0003` sudah menyediakan alat dengan pola yang pas: sumber JSON, tipe tertutup, `check` dan `generate`, serta uji gerbang di `cargo test --workspace`. Katalog cukup menjadi modul kedua di alat itu. Karena itu spec ini tidak menambah alat, aksi, atau folder fixture, dan semua bukti bersama berjalan di Mac tanpa jendela.

Deteksi terjemahan yang hilang dan fallback sengaja dipisah. Baris Done when meminta keduanya, dan dokumen resource bersama menyebut "fallback bila teks bahasa tujuan belum ada" di samping "deteksi entri hilang". Kalau terjemahan yang hilang menggagalkan `generate`, fallback per teks tidak pernah terjadi dan developer tidak bisa menjalankan aplikasi sebelum terjemahan datang. Kalau tidak ada yang menolaknya, terjemahan yang hilang sampai ke pengguna tanpa ada yang tahu. Jadi alat tetap menulis resource yang lengkap, ringkasannya menyebut setiap pasangan yang hilang, dan satu uji menolak katalog repo yang belum lengkap. Uji itu berjalan di gerbang yang sudah dipakai lead.

Hanya satu teks yang dipindahkan, walau spec macos/0001 mencatat tiga. Berkas fitur dan dokumen resource bersama sama sama meminta satu contoh, dan brief koordinasi menegaskannya. Judul `Desktop Foundation` memang bukan calon: spec `_root/0004` menjadikannya nama produk yang sama untuk semua bahasa. Harga yang dibayar adalah layar yang setengah diterjemahkan, dan itu ditulis terang di Consequences. Menambah `Name` kelak adalah satu entri katalog, tanpa rancangan baru.

Aturan nama locale dan fallback dibuat sesempit yang dibutuhkan `en` dan `id`. Draf pertama spec ini memuat aksara dan pewarisan teks dari bahasa induk. Cross check menunjukkan keduanya tidak punya padanan satu ke satu di gettext dan belum dipakai siapa pun, jadi keduanya dicabut dan dicatat sebagai perluasan kelak yang tidak memutus katalog yang sudah lengkap.

Pasangan `en` dan `id` serta teks `Sapa` adalah usulan. Pilihannya dibuat supaya tampilan bawaan yang sudah diverifikasi tidak berubah dan bahasa kedua masuk akal untuk repo ini. Nilai akhirnya milik user, dan menggantinya tidak menyentuh aturan.

Pada 2026-11-15 batas waktu AC-9 dan AC-11 diperjelas, karena spec `macos/0006` menaikkan jumlah keluaran dari `1 outputs` ke `3 outputs` dan spec `_root/0007` memindahkan uji `workspace_layout` ke `tooling/codegen/tests/`. Angka, perintah yang tercatat di `verify.md`, aturan, dan keputusan tidak berubah.

## Bukti percobaan

Urutan pemeriksaan, katalog uji K-1 dan K-2, dan 31 kasus sumber rusak sudah dijalankan pada prototipe Python yang mengikuti bagian **Urutan pemeriksaan** dan **Aturan sumber** (2026-11-15). Setiap kasus memberi pesan yang tertulis di tabel, kasus 16 memberi dua masalah, dan kasus lain memberi tepat satu. Perintah `python3` di AC-1 sudah dijalankan pada salinan katalog. Isi `catalog.schema.json` dibuat dengan penulis JSON yang mengurutkan kunci, memakai indentasi dua spasi, dan menambah satu baris baru di akhir, sama dengan aturan `to_canonical_json` di spec `_root/0003`.

Yang belum dicoba: kode Rust. Teks pesan `serde_json` di kasus 2, 4, dan 6 mengikuti bentuk pesan di tabel kasus spec `_root/0003`. Bila teksnya berbeda saat dibangun, tabel kasus di spec ini yang diperbarui.
