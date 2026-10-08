use colored::*;
use std::io::{self, Write};
use std::io::IsTerminal;
use std::time::Duration;

/// Název spustitelného souboru z Cargo.toml (`[[bin]] name`), použitý v příkladech.
const BIN: &str = env!("CARGO_BIN_NAME");

/// Šířka sloupce s přepínači, aby byly popisy za `│` pod sebou.
const OPT_WIDTH: usize = 22;

/// Vytiskne uvítací UI s popisem používání aplikace a ASCII logem
pub fn print_base_menu() {
    
    let ascii_logo = r#"
    %  %  %
   (  (  (
  ┌───────┐
  │ KOFIO │┐
  │  CLI  ││
  └───────┘┘
   └─────┘
    "#;

    println!("{}", ascii_logo.bright_green().bold());

    
    println!(
        "{}",
        "================================================================"
            .white()
    );
    println!(
        "  {} {}",
        "KOFIO CLI SCRAPER".bold().bright_green(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
    println!(
        "{}",
        "================================================================"
            .white()
    );
    println!("  Spusťte program s volitelnými parametry pro filtrování nabídek:\n");
    println!(
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-m, --max-price <KČ>").bold().yellow(),
        "│ Maximální cena za 100g kávy".dimmed()
    );
    println!(
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-r, --roaster <NÁZEV>").bold().yellow(),
        "│ Filtrovat pouze konkrétní pražírnu".dimmed()
    );
    println!(
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-s, --search <TEXT>").bold().yellow(),
        "│ Hledat text v názvu kávy nebo v chuťovém profilu".dimmed()
    );
    println!(
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-o, --on-sale").bold().yellow(),
        "│ Zobrazit jen zlevněné kávy".dimmed()
    );
    println!(
        "  {} {}",
        format!("{:<OPT_WIDTH$}", "-R, --refresh").bold().yellow(),
        "│ Stáhnout čerstvá data z webu místo cache".dimmed()
    );
    println!(
        "  {} {BIN} -m 180",
        "•".bright_green()
    );
    println!(
        "  {} {BIN} -r \"beansmith\" -m 200",
        "•".bright_green()
    );
    println!(
        "  {} {BIN} -s \"jahody\"",
        "•".bright_green()
    );
    println!(
        "\n{}\n",
        "----------------------------------------------------------------"
            .white()
    );
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

/// Vypíše informaci že data pochází z cache a jak jsou stará
pub fn print_cache_info(age: Duration) {
    let message = format!(
        "Data z cache ({}). Pro čerstvá data spusť {BIN} --refresh.",
        format_age(age)
    );

    eprintln!("{}\n", message.dimmed());
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
