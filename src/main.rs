use clap::Parser;

mod cli;
mod scraper;
mod table;
mod ui;

#[derive(Debug)]
pub struct Coffee {
    pub name: String,
    pub roaster: String,
    pub weight_g: u32,
    pub price_czk: f64,
    pub stock: String,
    pub flavors: String,
    pub url: String,
}

impl Coffee {
    fn price_per_100g(&self) -> f64 {
        (self.price_czk / self.weight_g as f64) * 100.0
    }
}

fn main() {
    let args = cli::CliArgs::parse();

    let has_filters_applied = args.max_price.is_some() ||
        args.roaster.is_some() ||
        args.search.is_some();

    if !has_filters_applied {
        ui::clear_screen();
        ui::print_base_menu();
    }
    
    match scraper::fetch_coffees() {
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
            println!("Error fetching data: {}", err);
        }
    }
}
