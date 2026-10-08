use clap::Parser;
use serde::{Deserialize, Serialize};

mod cli;
mod scraper;
mod table;
mod ui;
mod cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coffee {
    pub name: String,
    pub roaster: String,
    pub weight_g: u32,
    pub price_czk: f64,
    pub old_price_czk: Option<f64>,
    pub stock: String,
    pub flavors: String,
    pub url: String,
}

impl Coffee {
    fn price_per_100g(&self) -> f64 {
        (self.price_czk / self.weight_g as f64) * 100.0
    }

    /// Sleva v procentech spočítaná z původní a aktuální ceny, `None` u kávy bez slevy.
    fn discount_percent(&self) -> Option<f64> {
        self.old_price_czk
            .map(|old| (old - self.price_czk) / old * 100.0)
    }
}

/// hlavní vstupní funkce programu
fn main() {
    let args = cli::CliArgs::parse();

    let has_filters_applied = args.max_price.is_some() ||
        args.roaster.is_some() ||
        args.search.is_some() ||
        args.on_sale;

    if !has_filters_applied {
        ui::clear_screen();
        ui::print_base_menu();
    }
    
    match load_coffees(args.refresh) {
        Ok(mut coffees) => {
            // filtry podle zadání uživatele
            if let Some(max_p) = args.max_price {
                coffees.retain(|c| c.price_per_100g() <= max_p);
            }

            if let Some(ref r_filter) = args.roaster {
                let lower_r = r_filter.to_lowercase();
                coffees.retain(|c| c.roaster.to_lowercase().contains(&lower_r));
            }

            if let Some(ref search_item) = args.search {
                let lower_s = search_item.to_lowercase();
                coffees.retain(|c| {
                    c.name.to_lowercase().contains(&lower_s)
                        || c.flavors.to_lowercase().contains(&lower_s)
                });
            }

            if args.on_sale {
                coffees.retain(|c| c.old_price_czk.is_some());
            }

            // seřadíme vzestupně podle výhodnosti
            coffees.sort_by(|a, b| {
                a.price_per_100g()
                    .partial_cmp(&b.price_per_100g())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            if coffees.is_empty() {
                println!("\nŽádná káva neodpovídá zadaným filtrům.");
                return;
            }

            table::print_coffee_table(&coffees);
        }
        Err(err) => {
            eprintln!("Nepodařilo se stáhnout data z Kofio.cz: {err}");
        }
    }
}

/// vrátí kávy z cache, nebo je stáhne z webu
fn load_coffees(refresh: bool) -> Result<Vec<Coffee>, reqwest::Error> {
    if !refresh
        && let Some(cached) = cache::load()
        && cached.is_fresh()
    {
        ui::print_cache_info(cached.age);
        return Ok(cached.coffees);
    }

    let coffees = scraper::fetch_coffees()?;

    if !coffees.is_empty()
        && let Err(err) = cache::save(&coffees)
    {
        eprintln!("Varování: nepodařilo se uložit cache: {err}");
    }
    
    Ok(coffees)
}
