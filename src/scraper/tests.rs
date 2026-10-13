use super::*;

const ITEM_REGULAR: &str = include_str!("fixtures/item_regular.html");
const ITEM_DISCOUNTED: &str = include_str!("fixtures/item_discounted.html");
const MERCHANT_PAGE: &str = include_str!("fixtures/merchant_page.html");

#[test]
fn parses_regular_coffee() {
    let coffees = parse_coffees(ITEM_REGULAR);

    assert_eq!(coffees.len(), 1);
    let coffee = &coffees[0];
    assert_eq!(coffee.name, "Kolumbie LA CABAÑA");
    assert_eq!(coffee.roaster, "Beansmith.s");
    assert_eq!(coffee.roaster_url, "https://www.kofio.cz/beansmiths");
    assert_eq!(coffee.roaster_rating, None, "hodnocení pražírny se doplňuje až ze stránky pražírny");
    assert_eq!(coffee.rating, Some(1.8));
    assert_eq!(coffee.weight_g, 250);
    assert_eq!(coffee.price_czk, 379.0);
    assert_eq!(coffee.old_price_czk, None);
    assert_eq!(coffee.stock, "Skladem > 5ks");
    assert_eq!(coffee.flavors, "Lesní jahody, Nektarinka, Oolong");
    assert_eq!(
        coffee.url,
        "https://www.kofio.cz/kava/kolumbie-la-caba-a-beansmiths/21556"
    );
}

#[test]
fn parses_discounted_coffee() {
    let coffees = parse_coffees(ITEM_DISCOUNTED);

    assert_eq!(coffees.len(), 1);
    let coffee = &coffees[0];
    assert_eq!(coffee.price_czk, 1490.0);
    assert_eq!(coffee.old_price_czk, Some(1990.0));
    assert_eq!(coffee.weight_g, 480);
    assert_eq!(coffee.rating, None, "káva bez recenzí nemá hodnocení");

    let percent = coffee.discount_percent().unwrap();
    assert!((percent - 25.1).abs() < 0.1, "sleva má být cca 25 %, je {percent}");
}

#[test]
fn takes_weight_from_name_when_price_has_none() {
    let html = r#"
        <div class="category_item">
            <div class="category_item_footer">
                <h3><a href="/kava/test/1">Etiopie TEST - 200g</a></h3>
                <div class="price">300 Kč</div>
            </div>
        </div>
    "#;

    let coffees = parse_coffees(html);

    assert_eq!(coffees[0].weight_g, 200);
    assert_eq!(coffees[0].url, "https://www.kofio.cz/kava/test/1");
}

#[test]
fn skips_item_without_price() {
    let html = r#"
        <div class="category_item">
            <div class="category_item_footer">
                <h3><a href="/kava/test/1">Etiopie TEST</a></h3>
            </div>
        </div>
    "#;

    assert!(parse_coffees(html).is_empty());
}

#[test]
fn parses_roaster_rating_from_merchant_page() {
    assert_eq!(parse_roaster_rating(MERCHANT_PAGE), Some(3.9));
}

#[test]
fn roaster_rating_is_none_when_merchant_page_has_no_rating() {
    let html = r#"
        <div class="merchant_title_content">
            <h1>Nová pražírna</h1>
        </div>
    "#;

    assert_eq!(parse_roaster_rating(html), None);
}

#[test]
fn extracts_rating_with_comma_or_dot_and_rejects_out_of_range() {
    assert_eq!(extract_rating("4,5"), Some(4.5));
    assert_eq!(extract_rating(" 3.9 "), Some(3.9));
    assert_eq!(extract_rating("0"), None);
    assert_eq!(extract_rating("5,1"), None);
    assert_eq!(extract_rating("abc"), None);
}

#[test]
fn rating_ignores_review_count_in_nested_small() {
    let html = r#"
        <div class="category_item">
            <a class="category_rating_box"><i class="fa fa-star"></i> <div class="category_rating_box_avg">4,7<small class="category_rating_box_total">12x</small></div></a>
            <div class="category_item_footer">
                <h3><a href="/kava/test/1">Etiopie TEST</a></h3>
                <div class="price">300 Kč</div>
            </div>
        </div>
    "#;

    assert_eq!(parse_coffees(html)[0].rating, Some(4.7));
}
