# kofio -> scraper kávy z kofio.cz

CLI nástroj, který stáhne nabídku zrnkové kávy z [kofio.cz](https://www.kofio.cz)

## Instalace

### Linux a macOS (doporučeno)

```sh
curl -fsSL https://github.com/MatousKuchar01/kofio.cz_scraper/releases/latest/download/kofio-cli-installer.sh | sh
```

Skript stáhne hotovou binárku pro tvůj systém do `~/.cargo/bin` (nebo `~/.local/bin`) a přidá ji do `PATH`.
Po instalaci otevři nový terminál a napiš `kofio`.

### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/MatousKuchar01/kofio.cz_scraper/releases/latest/download/kofio-cli-installer.ps1 | iex"
```

### Ručně

Na stránce [Releases](https://github.com/MatousKuchar01/kofio.cz_scraper/releases/latest) stáhni archiv pro svůj systém,
rozbal ho a binárku `kofio` zkopíruj kamkoliv do `PATH`.

### Z Rust zdrojáků

Pokud máš nainstalovaný Rust (přes [rustup](https://rustup.rs)):

```sh
cargo install --git https://github.com/MatousKuchar01/kofio.cz_scraper
```

## Použití

```sh
kofio                                  # celá nabídka seřazená podle ceny za 100 g
kofio --max-price 180                  # jen kávy do 180 Kč / 100 g
kofio --roaster beansmith              # jen kávy od konkrétní pražírny
kofio --search etiopie                 # hledá v názvu kávy i v chuťovém profilu
kofio -m 200 -r kmen -s geisha         # filtry jde kombinovat, krátké přepínače fungují taky
kofio --on-sale                        # jen zlevněné kávy, sleva je v tabulce červeně u ceny
kofio --refresh                        # ignoruje 12hodinovou cache a stáhne čerstvá data
kofio --no-pager                       # vypíše tabulku přímo, bez procházení v less
kofio --help
```
