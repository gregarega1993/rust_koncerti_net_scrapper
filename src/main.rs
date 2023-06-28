use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    let response = get("https://www.koncerti.net").unwrap().text().unwrap();
    let document = Html::parse_document(&response);
    let concert_block_selector = parse_selector("div.vrstica");
    let date_selector = parse_selector("div.datum2");
    let title_selector = parse_selector("div.naziv");

    document
        .select(&concert_block_selector)
        .filter_map(|concert| {
            let concert_html = Html::parse_document(&concert.inner_html());
            concert_html
                .select(&date_selector)
                .next()
                .zip(concert_html.select(&title_selector).next())
                .map(|(datum_2, naziv)| (datum_2.inner_html(), naziv.inner_html()))
        })
        .filter(|(datum, _)| datum.contains("sobota"))
        .for_each(|(datum, naziv)| println!("Date: {} | Title: {}", datum, naziv));
}

fn parse_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}
