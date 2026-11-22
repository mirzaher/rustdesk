# EngineX Podrška / Tehničar (zasnovano na RustDesk-u)

Izmjena RustDesk-a (AGPL-3.0). Cijela izmjena: `src/enginex.rs` + po jedan red u `src/lib.rs`, `src/core_main.rs` i
`Cargo.toml`. Bez ijednog `enginex_*` featurea ovo je običan RustDesk.

| Izdanje | Feature | Šta radi |
|---|---|---|
| **Podrška** (kod korisnika) | `enginex_podrska` | samo prima veze; bez postavki, računa i adresara |
| **Tehničar** (vi) | `enginex_tehnicar` | samo uspostavlja veze; bez računa i adresara |

Server (`api.enginex.ba`) i javni ključ su ugrađeni i korisnik ih ne može promijeniti. Server se ne mijenja: vrijede
gotovi `hbbs`/`hbbr` i skripta `Server/Instaliraj-RustDeskServer.ps1` iz EngineX-Manager.

## Build

Potrebne varijable okruženja (build pada ako nema ključa):

```
ENGINEX_RS_PUB_KEY   sadržaj id_ed25519.pub sa servera (javan je, nije tajna)
ENGINEX_SERVER       zadano api.enginex.ba
ENGINEX_APP_NAME     zadano Podrska / Tehnicar
```

Cargo feature se doda uz uobičajeni RustDesk build (`flutter`, `hwcodec`...), npr.
`cargo build --release --features "flutter enginex_podrska"`. RustDeskov build (vcpkg, Flutter, Windows) je opisan u
`docs/` i `.github/workflows/flutter-build.yml`.

Provjereno: `src/enginex.rs` se prevodi protiv `hbb_common` (podrška, tehničar, bez featurea); odabir oba izdanja i
build bez ključa namjerno pada. **Cijeli Windows build nije pokretan.**

## Stari klijenti (Connecta)

Bez starog `id_ed25519` svaki klijent mora dobiti novi ključ. Za nastavak rada postojećih klijenata koristite
stari javni ključ pri buildu.

## AGPL-3.0

Ko dobije program ima pravo na izvorni kod ove verzije. Ovaj repozitorij ostaje javan, a program treba imati vidljiv
link na njega (npr. u opisu ili o programu).
