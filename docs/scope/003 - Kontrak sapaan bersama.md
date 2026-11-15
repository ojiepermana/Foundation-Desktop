# 003 - Kontrak sapaan bersama

Dasar · planned · needs a decision

Tetapkan satu fungsi publik dengan masukan nama dan hasil sapaan. Bagian spec tiap OS menjelaskan bagaimana UI memanggil fungsi itu. [Kontrak Rust](../contracts/README.md)

**Done when:** Uji Core tanpa UI membuktikan `hello("Ayu".to_string())` menghasilkan `Hello, Ayu!` dari satu implementasi.

* [ ] `/architect scope 003 kontrak sapaan untuk _root`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
