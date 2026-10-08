use super::*;

const ITEM_REGULAR: &str = include_str!("fixtures/item_regular.html");
const ITEM_DISCOUNTED: &str = include_str!("fixtures/item_discounted.html");

#[test]
fn parses_regular_coffee() {
    let coffees = parse_coffees(ITEM_REGULAR);

    assert_eq!(coffees.len(), 1);
    let coffee = &coffees[0];
    assert_eq!(coffee.name, "Kolumbie LA CABAÑA");
    assert_eq!(coffee.roaster, "Beansmith.s");
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
