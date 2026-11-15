# Fondasi Windows

Target pertama adalah jendela WinUI 3 tempat pengguna mengetik nama, menekan tombol, lalu melihat sapaan dari Rust Core. Ketika tersedia host Windows, tiap spec yang berlaku untuk Windows dapat menggunakan nomor fitur dari [scope bersama](../../scope/scope.md) dan ditempatkan di `docs/specs/windows/`.

## Alur dan kepemilikan

| Bagian | Tanggung jawab yang direncanakan |
| --- | --- |
| Presentasi di `apps/windows/` | Kontrol WinUI 3, navigasi, fokus, pintasan, aksesibilitas dan state tampilan. Komponen Rust memakai pesan bertipe dan model pembaruan yang didukung `windows-reactor`. |
| Adapter Windows | Akses WinRT dan API Windows melalui `windows-rs`, izin, aktivasi jendela dan pemuatan resource lokal. Adapter menginisialisasi runtime Windows App SDK dan melepas resource saat proses berakhir. |
| Rust Core | Memiliki kontrak dan implementasi `hello(name: String) -> String`. Tombol memanggil Core lewat dependensi Cargo langsung. Contohnya, masukan `Ayu` menghasilkan `Hello, Ayu!`. |

Seluruh kode aplikasi Windows menggunakan Rust. Runtime native WinUI dan Windows App SDK termasuk dependensi yang diizinkan. Lapisan aplikasi C#, .NET, XAML code behind, WebView dan FFI khusus untuk memanggil Rust Core berada di luar rancangan. Dependensi UI tetap berada di aplikasi Windows.

## Percobaan WinUI 3 di host Windows

`windows-reactor` adalah kandidat eksperimental dalam rencana proyek. Sebelum memperluas aplikasi, percobaan kecil di desktop Windows perlu menjawab tiga pertanyaan.

1. Apakah Rust dapat menginisialisasi Windows App SDK, membuka jendela native berisi judul, input, tombol dan label, memuat resource, lalu menutup aplikasi dengan bersih?
2. Apakah klik tombol menghasilkan pesan bertipe, memanggil Rust Core dan memperbarui label melalui state komponen? Apakah kontrol mengikuti tema terang dan gelap?
3. Apakah dependensi native dapat dipasang dan dikompilasi pada x64? Sejauh mana compiler, metadata dan runtime yang tersedia mendukung ARM64?

Matriks ini akan memuat satu baris untuk setiap kombinasi yang dicoba.

| Target | Versi Windows | Rust dan target Cargo | `windows-reactor` dan `windows-rs` | Windows App SDK, metadata dan runtime | Build | Uji jendela |
| --- | --- | --- | --- | --- | --- | --- |
| x86_64 | Belum dipilih | Belum dipilih | Belum dipilih | Belum dipilih | Belum diuji | Belum diuji |
| ARM64 | Belum dipilih | Belum dipilih | Belum dipilih | Belum dipilih | Kelayakan belum diuji | Belum diuji |

Hasil percobaan nantinya dicatat di `docs/platforms/windows/windows-reactor-evaluation.md` dan ditautkan dari spec Windows terkait. Integrasi dapat dilanjutkan bila kemampuan minimum terbukti pada kombinasi versi yang dicatat. Untuk kemampuan yang gagal, catat batasnya, periksa apakah API di `windows-rs` cukup untuk adapter Rust yang tipis, lalu rekam dampak dan keputusan dalam Architecture Decision Record (ADR). Perubahan pendekatan UI memerlukan keputusan arsitektur eksplisit. Selama bukti belum tersedia, status Windows tetap belum terverifikasi.

## Tema, resource dan perilaku native

* Tema Fluent memakai token warna semantik, tipografi dan jarak dari sumber bersama. Generator atau pemetaan yang dapat diulang menghasilkan nilai untuk Rust. API tema yang tersedia pada versi `windows-reactor` terpilih menentukan penerapannya.
* Fokus, keyboard, skala tampilan, tema sistem dan aksesibilitas mengikuti perilaku WinUI. Dukungan kontras tinggi perlu diperiksa sejauh API mendukungnya. Event dan perubahan tampilan mengikuti thread UI. Pekerjaan Core yang mahal perlu dijalankan di luar penangan event agar jendela tetap responsif.
* Aplikasi membaca konfigurasi publik bersama dan menyimpan pengaturan khusus Windows di sisi native. Adapter lokalisasi Rust memakai kunci bersama dengan format lokal dan pluralisasi native. Aset merek dikemas sebagai resource WinUI dari sumber bersama.
* Adapter menggunakan izin Windows sesuai kebutuhan, dependensi native tepercaya dan ketentuan distribusi runtime. Elevasi dan pemuatan pustaka dinamis dibatasi pada kebutuhan yang jelas.

## Bukti build, runtime dan distribusi

Runner Windows di GitHub Actions direncanakan untuk memeriksa toolchain, resolusi dependensi, kecocokan Windows App SDK, build WinUI dan integrasi Core. Perintah build yang direncanakan berada di `scripts/build-windows.ps1`. Sesi desktop Windows memberikan bukti terpisah untuk startup, interaksi, hasil Core, tema, resource dan shutdown. Jika runner hanya dapat melakukan kompilasi, hasil uji UI tetap terbuka.

Untuk distribusi kelak, pilihan yang perlu dinilai mencakup executable native dengan runtime terpasang atau disertakan jika didukung, MSIX, installer native, penandatanganan dan Microsoft Store. Pada tahap fondasi, cukup mencatat kebutuhan runtime, opsi yang kompatibel dan batas target. Kredensial penandatanganan disimpan di luar repositori.
