use std::io::{self, IsTerminal, Write};
use std::process::{Command, Stdio};

/// Vypíše text přes stránkovač `less`, aby výstup začínal nahoře u menu a dal se procházet
pub fn show(text: &str, enabled: bool) {
    if enabled && io::stdout().is_terminal() && show_in_less(text) {
        return;
    }

    print!("{text}");
}

/// Spustí `less` a pošle mu text. Vrací `false`, když se `less` nepodařilo spustit.
///
/// Přepínače: `-R` propustí barvy a odkazy, `-F` skončí hned, když se text vejde na obrazovku,
/// `-X` nechá výstup po ukončení v terminálu, `-S` nezalamuje dlouhé řádky tabulky.
fn show_in_less(text: &str) -> bool {
    let Ok(mut child) = Command::new("less")
        .args(["-R", "-F", "-X", "-S"])
        .stdin(Stdio::piped())
        .spawn()
    else {
        return false;
    };

    if let Some(mut stdin) = child.stdin.take() {
        // Když uživatel ukončí less dřív, než se zapíše celý text, zápis selže. To není chyba.
        let _ = stdin.write_all(text.as_bytes());
    }

    let _ = child.wait();
    true
}
