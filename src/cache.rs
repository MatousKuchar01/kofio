use crate::Coffee;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// verze formátu cache souboru -> zvýší se, pokud se mění struktura Coffee
const CACHE_VERSION: u32 = 2;

/// Jak dlouho jsou data z cache považovaná za čerstvá (12 hodin).
pub const CACHE_MAX_AGE: Duration = Duration::from_secs(12 * 60 * 60);

/// Název souboru s cache uvnitř složky `~/.cache/kofio/`
const CACHE_FILE_NAME: &str = "coffees.json";

/// Obsah cache souboru pro zápis
#[derive(Serialize)]
struct CacheFileRef<'a> {
    version: u32,
    fetched_at: u64,
    coffees: &'a [Coffee],
}

/// Obsah cache souboru pro čtení
#[derive(Deserialize)]
struct CacheFile {
    version: u32,
    fetched_at: u64,
    coffees: Vec<Coffee>,
}

/// Kávy načtené z cache spolu se stářím dat
pub struct CachedCoffees {
    pub coffees: Vec<Coffee>,
    pub age: Duration,
}

impl CachedCoffees {
    pub fn is_fresh(&self) -> bool {
        self.age <= CACHE_MAX_AGE
    }
}

/// Vrátí cestu k cache souboru, na Ubuntu `~/.cache/kofio/coffees.json`
pub fn cache_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|dir| dir.join("kofio").join(CACHE_FILE_NAME))
}

/// Aktuální čas jako unix timestamp
fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Uloží kávy do cache souboru 
/// Zapisuje nejdřív do dočasného souboru a ten pak přejmenuje na finální název
pub fn save(coffees: &[Coffee]) -> io::Result<()> {
    let path = cache_path().ok_or_else(|| io::Error::other("Nepodařilo se najít cache složku"))?;
    
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let data = CacheFileRef {
        version: CACHE_VERSION,
        fetched_at: now_unix(),
        coffees
    };

    let tmp_path = path.with_extension("json.tmp");
    let file = fs::File::create(&tmp_path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &data)?;
    writer.flush()?;

    fs::rename(&tmp_path, &path)?;    

    Ok(())
}

/// Načte kávy z cache souboru
pub fn load() -> Option<CachedCoffees> {
    let path = cache_path()?;
    let content = fs::read_to_string(&path).ok()?;
    let data: CacheFile = serde_json::from_str(&content).ok()?;

    if data.version != CACHE_VERSION {
        return None;
    }

    let age_secs = now_unix().saturating_sub(data.fetched_at);

    Some(CachedCoffees {
        coffees: data.coffees,
        age: Duration::from_secs(age_secs),
    })
}
