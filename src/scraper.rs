use crate::{Coffee, RATING_MAX};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub fn fetch_coffees() -> Result<Vec<Coffee>, Error> {
    let mut all_coffees = Vec::new();
    let mut seen_keys = HashSet::new();
    let mut offset = 0; // (?s=0, ?s=24, ?s=48...)

    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("|/-\\ ")
            .template("{spinner:.bold.green} {msg}")
            .unwrap(),
    );

    pb.enable_steady_tick(Duration::from_millis(80));

    loop {
        pb.set_message(format!(
            "Stahuji Kofio.cz | Načteno {} káv...",
            all_coffees.len()
        ));

        let url = format!("https://www.kofio.cz/kava/filtr?s={}", offset);

        let response = reqwest::blocking::get(&url)?;
        let html_content = response.text()?;

        let coffees_on_page = parse_coffees(&html_content);

        let mut new_coffees_count = 0;

        for coffee in coffees_on_page {
            // (např. "Kolumbie LA CABAÑA-Beansmith.s-250")
            let unique_key = format!("{}-{}-{}", coffee.name, coffee.roaster, coffee.weight_g);

            if seen_keys.insert(unique_key) {
                all_coffees.push(coffee);
                new_coffees_count += 1;
            }
        }

        if new_coffees_count == 0 {
            break;
        }

        offset += 24;

        // pro vývoj
        /*if offset > 48 {
            break;
        }*/
    }

    fetch_roaster_ratings(&mut all_coffees, &pb);

    pb.finish_with_message(format!(
        " Staženo celkem {} káv z Kofio.cz\n",
        all_coffees.len()
    ));

    Ok(all_coffees)
}

/// Stáhne stránku každé pražírny (jednou, i když má víc káv) a doplní kávám hodnocení pražírny
/// Když se stránku pražírny nepodaří stáhnout, zůstane u jejích káv `roaster_rating = None`
fn fetch_roaster_ratings(coffees: &mut [Coffee], pb: &ProgressBar) {
    let roaster_urls: Vec<String> = {
        let mut seen = HashSet::new();
        coffees
            .iter()
            .map(|c| c.roaster_url.clone())
            .filter(|url| !url.is_empty() && seen.insert(url.clone()))
            .collect()
    };

    let mut ratings: HashMap<String, Option<f64>> = HashMap::new();

    for (index, url) in roaster_urls.iter().enumerate() {
        pb.set_message(format!(
            "Stahuji hodnocení pražíren {}/{}...",
            index + 1,
            roaster_urls.len()
        ));

        let rating = reqwest::blocking::get(url)
            .and_then(|response| response.text())
            .ok()
            .and_then(|html| parse_roaster_rating(&html));

        ratings.insert(url.clone(), rating);
    }

    for coffee in coffees.iter_mut() {
        coffee.roaster_rating = ratings.get(&coffee.roaster_url).copied().flatten();
    }
}

/// Vytáhne průměrné hodnocení pražírny ze stránky pražírny (např. `https://www.kofio.cz/beansmiths`)
/// Hodnocení je v elementu `div.merchant_rating span.total_nums` jako `3.9<small> / 5</small>`
fn parse_roaster_rating(html: &str) -> Option<f64> {
    let document = Html::parse_document(html);
    let rating_selector = Selector::parse("div.merchant_rating span.total_nums").unwrap();

    document
        .select(&rating_selector)
        .next()
        .and_then(|el| extract_rating(&direct_text(&el)))
}

/// Parsuje HTML dokument a extrahuje z něj seznam produktů kávy.
///
/// Funkce prochází HTML strukturu pomocí CSS selektorů a vyhledává elementy reprezentující
/// jednotlivé kávy (`div.category_item`). U každé položky se pokouší extrahovat název,
/// odkaz na detail, pražírnu, hmotnost, cenu, dostupnost a chuťový profil.
///
/// # Chování a fallbacky
///
/// * **Přeskakování:** Pokud položka nemá název, má název prázdný, nebo je její výsledná cena `0.0`, do výsledného seznamu se **nepřidá**.
/// * **Extrakce hmotnosti:** Hmotnost se primárně hledá v elementu `div.price small`. Pokud tam není, zkusí se extrahovat přímo z názvu kávy. Pokud selžou obě možnosti, použije se výchozí hodnota **250 g**.
/// * **Neznámá pražírna:** Pokud se nepodaří dohledat název pražírny z atributu `title`, nastaví se text `"Neznámá pražírna"`.
///
/// # Arguments
///
/// * `html` - Vstupní textový řetězec obsahující HTML kód stránky (např. stažený e-shop).
///
/// # Returns
///
/// Vrací `Vec<Coffee>` obsahující všechny úspěšně na parsované kávy. Pokud HTML neobsahuje žádné odpovídající prvky, vrátí prázdný vektor.
fn parse_coffees(html: &str) -> Vec<Coffee> {
    let document = Html::parse_document(html);

    // Selektory z přiloženého HTML
    let item_selector = Selector::parse("div.category_item").unwrap();
    let name_selector = Selector::parse("div.category_item_footer h3 a").unwrap();
    let roaster_selector = Selector::parse("div.category_item_merchant a").unwrap();
    let rating_selector = Selector::parse("a.category_rating_box div.category_rating_box_avg").unwrap();
    let price_selector = Selector::parse("div.price").unwrap();
    let old_price_selector = Selector::parse("div.price span.price_old").unwrap();
    let small_selector = Selector::parse("div.price small").unwrap();
    let stock_selector = Selector::parse("div.stock_availability span").unwrap();
    let flavors_selector = Selector::parse("div.category_item_flavors").unwrap();

    let mut coffees = Vec::new();

    for item in document.select(&item_selector) {
        // extrakce názvu kávy a odkazu na detail
        let (name, url) = match item.select(&name_selector).next() {
            Some(el) => {
                let text = el.text().collect::<String>().trim().to_string();
                let href = el.value().attr("href").unwrap_or("").trim();

                // relativní cestu doplníme o doménu
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else if !href.is_empty() {
                    format!("https://www.kofio.cz{}", href)
                } else {
                    String::new()
                };

                (text, full_url)
            }
            None => continue, // Pokud káva nemá název, přeskočíme
        };

        if name.is_empty() {
            continue;
        }

        // extrakce pražírny a odkazu na její stránku
        let (roaster, roaster_url) = match item.select(&roaster_selector).next() {
            Some(el) => (
                el.value()
                    .attr("title")
                    .unwrap_or("Neznámá pražírna")
                    .trim()
                    .to_string(),
                el.value().attr("href").unwrap_or("").trim().to_string(),
            ),
            None => ("Neznámá pražírna".to_string(), String::new()),
        };

        // extrakce hodnocení kávy (průměr hvězd), element obsahuje i <small> s počtem recenzí
        let rating = item
            .select(&rating_selector)
            .next()
            .and_then(|el| extract_rating(&direct_text(&el)));

        // extrakce gramáže
        let mut weight_g: u32 = 0;

        if let Some(small_el) = item.select(&small_selector).next() {
            let small_text = small_el.text().collect::<String>();
            weight_g = extract_weight(&small_text);
        }

        if weight_g == 0 {
            weight_g = extract_weight(&name);
        }

        if weight_g == 0 {
            weight_g = 250; // Fallback na 250g
        }

        // extrakce celkové ceny
        let mut price_czk: f64 = 0.0;

        if let Some(price_el) = item.select(&price_selector).next() {
            let direct_text: String = price_el
                .children()
                .filter_map(|node| {
                    if let Some(el) = node.value().as_element() {
                        if el.name() == "div" || el.classes().any(|c| c == "price_old") {
                            return None;
                        }
                        return ElementRef::wrap(node).map(|e| e.text().collect::<String>());
                    }
                    node.value().as_text().map(|t| t.to_string())
                })
                .collect();

            price_czk = extract_price(&direct_text);
        }

        // extrakce původní ceny u zlevněné kávy
        let old_price_czk = item
            .select(&old_price_selector)
            .next()
            .map(|el| extract_price(&el.text().collect::<String>()))
            .filter(|old| *old > price_czk);

        // extrakce dostupnosti
        let stock = match item.select(&stock_selector).next() {
            Some(el) => el.text().collect::<String>().trim().to_string(),
            None => "Neuvedeno".to_string(),
        };

        // extrakce chuti
        let flavors_raw = match item.select(&flavors_selector).next() {
            Some(el) => el.text().collect::<String>(),
            None => "".to_string(),
        };

        let flavors = flavors_raw
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        if price_czk > 0.0 {
            coffees.push(Coffee {
                name,
                rating,
                roaster,
                roaster_url,
                roaster_rating: None,
                weight_g,
                price_czk,
                old_price_czk,
                stock,
                flavors,
                url,
            });
        }
    }

    coffees
}

/// Vrátí jen přímé textové uzly elementu, bez textu vnořených elementů
/// např. `<div>1,8<small>3x</small></div>` -> `"1,8"`
fn direct_text(el: &ElementRef) -> String {
    el.children()
        .filter_map(|node| node.value().as_text().map(|t| t.to_string()))
        .collect()
}

/// Extrahuje průměrné hodnocení (počet hvězd) z textu, např. `"4,5"` nebo `"3.9"`.
fn extract_rating(text: &str) -> Option<f64> {
    text.trim()
        .replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|rating| *rating > 0.0 && *rating <= RATING_MAX)
}

/// Extrahuje hmotnost v gramech z textového řetězce.
///
/// Funkce v textu vyhledá číselnou hodnotu, která reprezentuje hmotnost v gramech (končí znakem 'g' nebo 'G').
/// Aby byla hmotnost úspěšně extrahována, musí splňovat rozsah **od 50 g do 5000 g** (včetně).
///
/// Pokud text neobsahuje žádné platné číslo s jednotkou gramů, nebo pokud nalezená hmotnost
/// leží mimo povolený rozsah, funkce **nevyvolá panic** a vrátí `0`.
///
/// # Arguments
///
/// * `text` - Vstupní textový řetězec (např. "Balení kávy 250g mletá" nebo "Hmotnost: 500 G").
fn extract_weight(text: &str) -> u32 {
    let cleaned: String = text
        .chars()
        .map(|c| {
            if c.is_ascii_digit() || c == 'g' || c == 'G' {
                c
            } else {
                ' '
            }
        })
        .collect();

    for word in cleaned.split_whitespace() {
        if word.ends_with('g') || word.ends_with('G') {
            let num_str = &word[..word.len() - 1];
            if let Ok(num) = num_str.parse::<u32>() {
                if (50..=5000).contains(&num) {
                    return num;
                }
            }
        }
    }
    0
}

/// Extrahuje číselnou hodnotu ceny z textového řetězce.
///
/// Funkce vezme text před prvním výskytem zkratky "Kč" (pokud tam je),
/// odstraní z něj všechny znaky kromě číslic, teček a čárek,
/// převede čárky na tečky a pokusí se výsledek převést na `f64`.
///
/// Pokud se v textu žádné číslo nenajde nebo parsování selže,
/// funkce **nevyvolá panic**, ale vrátí hodnotu `0.0`.
///
/// # Arguments
///
/// * `text` - Vstupní textový řetězec, který obsahuje cenu (např. "Cena: 1 250,50 Kč").
fn extract_price(text: &str) -> f64 {
    let text_before_kc = text.split("Kč").next().unwrap_or(text);

    let cleaned: String = text_before_kc
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
        .collect();

    let cleaned = cleaned.replace(',', ".");
    cleaned.parse::<f64>().unwrap_or(0.0)
}

#[cfg(test)]
mod tests;
