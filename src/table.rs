use crate::Coffee;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

/// Štítek zobrazený ve sloupci "Odkaz". Sloupec má `ColumnConstraint::ContentWidth`, takže se štítek nikdy nezalomí.
const LINK_LABEL: &str = "otevřít ↗";

/// Vytiskne přehlednou tabulku káv do standardního výstupu (STDOUT).
///
/// Funkce využívá formátování sady znaků UTF-8 (`UTF8_FULL`) pro vykreslení tabulky.
/// Automaticky počítá pořadí (rank) jednotlivých položek od jedničky.
///
/// # Vizuální formátování
///
/// * **Zarovnání:** Sloupce pro balení (hmotnost), celkovou cenu a cenu za 100g jsou zarovnány **doprava** pro lepší čitelnost číselných hodnot.
/// * **Zvýraznění TOP 3:** První **tři položky** (indexy 0, 1, 2) jsou v tabulce zvýrazněny **zelenou barvou a tučným písmem**, což je ideální pro zobrazení nejvýhodnějších nebo nejlépe hodnocených káv.
/// * **Výpočet ceny:** Pro sloupec "Cena / 100g" funkce interně volá metodu `.price_per_100g()` na struktuře `Coffee`.
/// * **Odkaz:** Sloupec "Odkaz" obsahuje krátký štítek, pod kterým je schovaná URL detailu kávy
///   jako terminálový hyperlink (OSC 8). V terminálech s podporou hyperlinků (GNOME Terminal, Kitty,
///   WezTerm, iTerm2, Windows Terminal, VS Code…) jde kliknout (obvykle s Ctrl), jinde se zobrazí jen štítek.
///   Sloupec má pevnou šířku podle obsahu, aby se štítek nikdy nezalomil a odkaz zůstal celistvý.
///
/// # Arguments
///
/// * `coffees` - Slice (pohled na pole) struktur `Coffee`, které se mají v tabulce zobrazit. Pokud je předán prázdný slice, vytiskne se pouze hlavička tabulky.
pub fn print_coffee_table(coffees: &[Coffee]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("#").add_attribute(Attribute::Bold),
        Cell::new("Název").add_attribute(Attribute::Bold),
        Cell::new("Pražírna").add_attribute(Attribute::Bold),
        Cell::new("Balení").add_attribute(Attribute::Bold),
        Cell::new("Cena").add_attribute(Attribute::Bold),
        Cell::new("Cena / 100g").add_attribute(Attribute::Bold),
        Cell::new("Chuťový profil").add_attribute(Attribute::Bold),
        Cell::new("Skladem").add_attribute(Attribute::Bold),
        Cell::new("Odkaz").add_attribute(Attribute::Bold),
    ]);

    if let Some(column) = table.column_mut(3) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    if let Some(column) = table.column_mut(4) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    if let Some(column) = table.column_mut(5) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    if let Some(column) = table.column_mut(8) {
        column.set_constraint(ColumnConstraint::ContentWidth);
    }

    for (index, coffee) in coffees.iter().enumerate() {
        let rank = index + 1;
        let is_top3 = rank <= 3;

        let flavors_display = if coffee.flavors.trim().is_empty() {
            "-".to_string()
        } else {
            coffee.flavors.clone()
        };

        let mut row = vec![
            Cell::new(rank.to_string()),
            Cell::new(&coffee.name),
            Cell::new(&coffee.roaster),
            Cell::new(format!("{}g", coffee.weight_g)),
            Cell::new(price_cell(coffee)),
            Cell::new(format!("{:.2} Kč", coffee.price_per_100g())),
            Cell::new(flavors_display),
            Cell::new(&coffee.stock),
            Cell::new(link_cell(&coffee.url)),
        ];

        if is_top3 {
            row = row
                .into_iter()
                .map(|cell| cell.fg(Color::Green).add_attribute(Attribute::Bold))
                .collect();
        }

        table.add_row(row);
    }

    println!("{table}");
}

/// Vrátí text buňky s cenou, u zlevněné kávy doplněný o červené procento slevy, např. "1490 Kč -25 %".
fn price_cell(coffee: &Coffee) -> String {
    match coffee.discount_percent() {
        Some(percent) => format!(
            "{:.0} Kč {}",
            coffee.price_czk,
            format!("-{percent:.0} %").red().bold()
        ),
        None => format!("{:.0} Kč", coffee.price_czk),
    }
}

/// Vrátí text buňky s odkazem: krátký štítek obalený OSC 8 hyperlinkem na `url`.
fn link_cell(url: &str) -> String {
    if url.is_empty() {
        return "-".to_string();
    }

    hyperlink(url, LINK_LABEL)
}

/// Obalí `label` terminálovým hyperlinkem (OSC 8): `ESC ] 8 ; ; url ESC \ label ESC ] 8 ; ; ESC \`.
fn hyperlink(url: &str, label: &str) -> String {
    format!("\x1b]8;;{url}\x1b\\{label}\x1b]8;;\x1b\\")
}
