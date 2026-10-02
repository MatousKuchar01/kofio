use colored::*;
use std::io::{self, Write};

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
        "v1.0".dimmed()
    );
    println!(
        "{}",
        "================================================================"
            .white()
    );
    println!("  Spusťte program s volitelnými parametry pro filtrování nabídek:\n");
    println!(
        "  {} {}",
        "-m, --max-price <KČ>".bold().yellow(),
        "│ Maximální cena za 100g kávy".dimmed()
    );
    println!(
        "  {} {}",
        "-r, --roaster <NÁZEV>".bold().yellow(),
        "│ Filtrovat pouze konkrétní pražírnu".dimmed()
    );
    println!(
        "  {} {}",
        "-s, --search <TEXT>  ".bold().yellow(),
        "│ Hledat text v názvu kávy nebo v chuťovém profilu".dimmed()
    );
    println!(
        "  {} cargo run -- -m 180",
        "•".bright_green()
    );
    println!(
        "  {} cargo run -- -r \"beansmith\" -m 200",
        "•".bright_green()
    );
    println!(
        "  {} cargo run -- -s \"jahody\"",
        "•".bright_green()
    );
    println!(
        "\n{}\n",
        "----------------------------------------------------------------"
            .white()
    );
}

/// Vyčistí obrazovku terminálu a posune kurzor na pozici 1x1
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}
