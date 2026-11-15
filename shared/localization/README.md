# Katalog bahasa bersama

`catalog.json` adalah satu satunya sumber teks antarmuka yang dilokalkan untuk aplikasi macOS, Windows, dan Linux. `catalog.schema.json` dan resource bahasa di tiap aplikasi dibuat dari berkas ini oleh `desktop-codegen`. Jangan menyunting hasil generator.

Cara mengubah teks: sunting `catalog.json`, jalankan `cargo run -p desktop-codegen --locked -- localization generate`, lalu `cargo run -p desktop-codegen --locked -- localization check`, dan commit katalog bersama berkas turunannya.

Aturan kunci, nama locale, placeholder, fallback, deteksi terjemahan yang hilang, dan versi ada di [spec 0006](../../docs/specs/_root/0006-satu-contoh-lokalisasi/index.md), bagian "Aturan lokalisasi bersama".
