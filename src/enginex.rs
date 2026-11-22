//! EngineX "Podrska" / "Tehnicar" izdanje RustDesk-a.
//!
//! Umjesto RustDeskovog potpisanog `custom.txt` (potpisuje ga samo RustDesk), vrijednosti se zadaju pri buildu
//! i upisuju u globalne postavke odmah po pokretanju (vidi `core_main::core_main`).
//!
//! Dva izdanja, ista baza koda (cargo feature):
//!   * `enginex_podrska`  - klijent kod korisnika: samo prima veze, bez postavki, racuna i adresara.
//!   * `enginex_tehnicar` - tehnicar: samo uspostavlja veze.
//!
//! Potrebne varijable okruzenja pri buildu (build pada ako ih nema, da ne nastane klijent bez ispravnog servera):
//!   ENGINEX_RS_PUB_KEY  javni kljuc servera (sadrzaj `id_ed25519.pub`; javan je, nije tajna)
//!   ENGINEX_SERVER      ID/relay server, zadano `api.enginex.ba`
//!   ENGINEX_APP_NAME    ime programa, zadano `Podrska` / `Tehnicar`
//!
//! Licenca: RustDesk je AGPL-3.0; izvorni kod ove verzije mora biti dostupan svakome ko dobije program.

use hbb_common::config;

#[cfg(all(feature = "enginex_podrska", feature = "enginex_tehnicar"))]
compile_error!("Odaberi samo jedno izdanje: enginex_podrska ILI enginex_tehnicar.");

#[cfg(any(feature = "enginex_podrska", feature = "enginex_tehnicar"))]
const PUB_KEY: &str = env!("ENGINEX_RS_PUB_KEY");

#[cfg(any(feature = "enginex_podrska", feature = "enginex_tehnicar"))]
const SERVER: &str = match option_env!("ENGINEX_SERVER") {
    Some(s) => s,
    None => "api.enginex.ba",
};

#[cfg(feature = "enginex_podrska")]
const APP_NAME: &str = match option_env!("ENGINEX_APP_NAME") {
    Some(s) => s,
    None => "Podrska",
};

#[cfg(feature = "enginex_tehnicar")]
const APP_NAME: &str = match option_env!("ENGINEX_APP_NAME") {
    Some(s) => s,
    None => "Tehnicar",
};

/// Poziva se jednom, odmah poslije `load_custom_client()`. Bez ijednog enginex featurea ne radi nista,
/// pa obican RustDesk build ostaje nepromijenjen.
pub fn apply() {
    #[cfg(any(feature = "enginex_podrska", feature = "enginex_tehnicar"))]
    {
        *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();

        // server i kljuc: nametnuto, korisnik ih ne moze promijeniti
        {
            let mut o = config::OVERWRITE_SETTINGS.write().unwrap();
            o.insert("custom-rendezvous-server".to_owned(), SERVER.to_owned());
            o.insert("relay-server".to_owned(), SERVER.to_owned());
            o.insert("key".to_owned(), PUB_KEY.to_owned());
        }

        // zakljucane opcije (HARD_SETTINGS): "Y" = ukljuceno
        let mut h = config::HARD_SETTINGS.write().unwrap();
        h.insert("disable-account".to_owned(), "Y".to_owned());
        h.insert("disable-ab".to_owned(), "Y".to_owned());
        #[cfg(feature = "enginex_podrska")]
        {
            h.insert("conn-type".to_owned(), "incoming".to_owned());
            h.insert("disable-settings".to_owned(), "Y".to_owned());
        }
        #[cfg(feature = "enginex_tehnicar")]
        {
            h.insert("conn-type".to_owned(), "outgoing".to_owned());
        }
    }
    #[cfg(not(any(feature = "enginex_podrska", feature = "enginex_tehnicar")))]
    {
        let _ = &config::APP_NAME; // bez featurea nema izmjena
    }
}
