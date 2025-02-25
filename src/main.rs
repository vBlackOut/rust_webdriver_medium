use std::error::Error;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};
use std::collections::HashSet;

async fn get_page(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    let html = driver.source().await?;
    println!("{html}");
    Ok(())
}

async fn get_element_text_xpath(driver: &WebDriver, xpath_element: &str, author: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _element = driver.find_all(By::XPath(xpath_element)).await?; // Sélectionne le premier <h1>
    //dbg!(_element.html().await?);
    //let text = element.text().await?;
    //println!("Texte trouvé: {}", text);
    println!("Nombre d'éléments trouvés : {}", _element.len());

    let mut links_set = HashSet::new();

    for (i, item) in _element.iter().enumerate() {
        let text_html = item.text().await?;

        // find author in page
        let _element2 = item.find_all(By::XPath(author)).await?;
        for (i, item2) in _element2.iter().enumerate() {
          let link = &item2.attr("href").await?;

          if let Some(link) = link {
               let link_parts: Vec<&str> = link.split("/").collect();

               if link_parts[1].contains("@") {
                    let url_user: Vec<_> = link_parts[1].split("?").collect();
                    let formatted_link = format!("https://medium.com/{}/feed", url_user[0]);
                    links_set.insert(formatted_link);
               }
          } else {
               println!("Aucun lien trouvé !");
          }
        }

    }

    for link in &links_set {
        println!("{}", link);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // define the browser options
    let caps = DesiredCapabilities::chrome();

    // initialize a driver to control a Chrome instance
    // with the specified options
    let driver = WebDriver::new("http://localhost:34997", caps).await?;

    // visit the target page
    let search = "search?q=google";
    let number_scroll = 100;
    let _timesleep_scroll = 800; // in milli seconds

    let link_search = format!("{}/{}", "https://medium.com", search);
    driver.goto(link_search).await?;
    // retrieve the source HTML of the target page
    // and print it
    for i in 1..=number_scroll {
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);", vec![]).await?;
     sleep(Duration::from_millis(_timesleep_scroll)).await;
     match driver.find(By::XPath("//button[contains(text(), 'Show more')]")).await {
          Ok(button) => {
               println!("Bouton trouvé avec XPath, clique numéro: {}", i);
               button.click().await?;
          }
          Err(_) => println!("Bouton introuvable"),
     }
    }

    // get_page(&driver).await?;
    let page = "//div[@id='root']/div/div[3]/div[2]/div/main/div/div[2]/div";
    let search = "//div[@id='root']/div/div[3]/div[2]/div/main/div/div/div[2]/div";

    get_element_text_xpath(&driver, search, "//a[@rel='noopener follow']").await?;

    // close the browser and release its resources
    driver.quit().await?;

    Ok(())
}


