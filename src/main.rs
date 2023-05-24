fn main() {
    let url = "https://www.foxnews.com/";

    let response = reqwest::blocking::get(url).expect("Error loading the url");

    let raw_html = response.text().unwrap();

    let document = scraper::Html::parse_document(&raw_html);

    let title_selector = scraper::Selector::parse("h3.title>a").unwrap();

    let article_titles = document.select(&title_selector).map(|x| x.inner_html());

    for (number, item) in article_titles.enumerate() {
        println!("Article number: {}, Title: {}", number + 1, item);
    }

}