// wave height: class="quiver-surf-height"



use reqwest;
use scraper::{Html, Selector};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.surfline.com/surf-report/rockpiles/5842041f4e65fad6a7708b39")
        .await?
        .text()
        .await?;

    // println!("{:#?}", resp);

    let document = Html::parse_document(&resp);
    
    let infobox_selector = Selector::parse(".quiver-spot-forecast-summary__wrapper").unwrap();
    let infobox = document.select(&infobox_selector).next().unwrap();


    let waveheight = Selector::parse(".quiver-surf-height").unwrap();
    let tide = Selector::parse(".quiver-reading").unwrap();

    for element in infobox.select(&waveheight) {
        println!("wave height: {:#?}", element.inner_html() );
    }

    for element in infobox.select(&tide) {
        println!("tide: {:#?}", element.inner_html() );
    }

    // println!("wave height: {:#?}", wave_height);
    
    Ok(())
    
}
