use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Cenový auditor kávy z Kofio.cz", long_about = None)]
pub struct CliArgs {
    // max cena za 100g v Kč (např. --max-price 180)
    #[arg(short, long)]
    pub max_price: Option<f64>,

    // filtr podle názvu pražírny (např. --roaster "Beansmith")
    #[arg(short, long)]
    pub roaster: Option<String>,

    // vyhledat v názvu kávy nebo chuti (např. --search "Etiopie")
    #[arg(short, long)]
    pub search: Option<String>,
}
