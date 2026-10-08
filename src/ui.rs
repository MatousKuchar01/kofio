use colored::*;
use std::fmt::Write as _;
use std::io::{self, Write};
use std::io::IsTerminal;
use std::time::Duration;

/// Název spustitelného souboru z Cargo.toml (`[[bin]] name`), použitý v příkladech.
const BIN: &str = env!("CARGO_BIN_NAME");

/// Šířka sloupce s přepínači, aby byly popisy za `│` pod sebou.
const OPT_WIDTH: usize = 22;

/// Vrátí uvítací UI s popisem používání aplikace a ASCII logem jako text.
pub fn base_menu() -> String {
    let mut menu = String::new();

    let ascii_logo = r#"
    %  %  %
   (  (  (
  ┌───────┐
  │ KOFIO │┐
  │  CLI  ││
  └───────┘┘
   └─────┘
    "#;

    let _ = writeln!(menu, "{}", ascii_logo.bright_green().bold());

    
    let _ = writeln!(
        menu,
        "{}",
        separator('=').white()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        "KOFIO CLI SCRAPER".bold().bright_green(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
    let _ = writeln!(
        menu,
        "{}",
        separator('=').white()
    );
    let _ = writeln!(menu, "  Spusťte program s volitelnými parametry pro filtrování nabídek:\n");
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-m, --max-price <KČ>").bold().yellow(),
        "│ Maximální cena za 100g kávy".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-r, --roaster <NÁZEV>").bold().yellow(),
        "│ Filtrovat pouze konkrétní pražírnu".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-s, --search <TEXT>").bold().yellow(),
        "│ Hledat text v názvu kávy nebo v chuťovém profilu".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-o, --on-sale").bold().yellow(),
        "│ Zobrazit jen zlevněné kávy".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-R, --refresh").bold().yellow(),
        "│ Stáhnout čerstvá data z webu místo cache".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "    --no-pager").bold().yellow(),
        "│ Vypsat tabulku přímo, bez procházení v less".dimmed()
    );
    let _ = writeln!(
        menu,
        "  {} {BIN} -m 180",
        "•".bright_green()
    );
    let _ = writeln!(
        menu,
        "  {} {BIN} -r \"beansmith\" -m 200",
        "•".bright_green()
    );
    let _ = writeln!(
        menu,
        "  {} {BIN} -s \"jahody\"",
        "•".bright_green()
    );
    let _ = writeln!(
        menu,
        "\n  {} {}",
        "Ovládání:".bold(),
        "↑ ↓ mezerník posun, ← → široká tabulka, / hledat, q konec".dimmed()
    );
    let _ = writeln!(
        menu,
        "\n{}\n",
        separator('-').white()
    );

    menu
}

/// Vrátí čáru ze znaku `ch` přes celou šířku terminálu
fn separator(ch: char) -> String {
    let width = crossterm::terminal::size()
        .map(|(cols, _)| cols as usize)
        .unwrap_or(64);

    ch.to_string().repeat(width)
}

/// Vyčistí obrazovku terminálu a posune kurzor na pozici 1x1.
///
/// Pokud výstup nesměřuje do terminálu (např. `kofio > kavy.txt` nebo pipe),
/// nedělá nic, aby se do souboru nezapsaly escape sekvence.
pub fn clear_screen() {
    if !io::stdout().is_terminal() {
        return;
    }

    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// Vrátí informaci, že data pochází z cache a jak jsou stará
pub fn cache_info(age: Duration) -> String {
    let message = format!(
        "Data z cache ({}). Pro čerstvá data spusť {BIN} --refresh.",
        format_age(age)
    );

    format!("{}\n\n", message.dimmed())
}

/// převede stáří dat na číselný text
fn format_age(age: Duration) -> String {
    let total_minutes = age.as_secs() / 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    if total_minutes == 0 {
        "staženo před chvílí".to_string()
    } else if hours == 0 {
        format!("staženo před {minutes} min")
    } else {
        format!("staženo před {hours} h {minutes} min")
    }
}
