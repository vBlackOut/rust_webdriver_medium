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
    let element = driver.find_all(By::XPath(xpath_element)).await?;
    
    println!("Nombre d'éléments trouvés : {}", element.len());

    let mut links_set = HashSet::new();
    // iter for all parent element
    for (i, item) in element.iter().enumerate() {
        let text_html = item.text().await?;

        // find link author in parent (this is search child) 
        let element2 = item.find_all(By::XPath(author)).await?;

        // iter child element
        for (i, item2) in element2.iter().enumerate() {
          // give attrib link
          let link = &item2.attr("href").await?;
            
          // get link inside the Some it's small particularity rust use if let.
          if let Some(link) = link {
               let link_parts: Vec<&str> = link.split("/").collect();
              
               // compare is contains author in url @....
               if link_parts[1].contains("@") {
                    let url_user: Vec<_> = link_parts[1].split("?").collect();
                    // build url
                    let formatted_link = format!("https://medium.com/{}/feed", url_user[0]);
                    
                    // inject url in Set list.
                    links_set.insert(formatted_link);
               }
          } else {
               println!("Aucun lien trouvé !");
          }
        }

    }
    
    // print output set list
    for link in &links_set {
        println!("{}", link);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // define the browser options
    let caps = DesiredCapabilities::chrome();
    
    // launch chromdriver 
    let driver = WebDriver::new("http://localhost:34997", caps).await?;

    // visit the target page
    let search = "search?q=google";
    let number_scroll = 100;
    let timesleep_scroll = 800; // in milli seconds

    let link_search = format!("{}/{}", "https://medium.com", search);
    driver.goto(link_search).await?;

    for i in 1..=number_scroll {
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);", vec![]).await?;
     // wait minor time for detect button after every loop for
     sleep(Duration::from_millis(timesleep_scroll)).await;
     
     // find element button "Show more"
     match driver.find(By::XPath("//button[contains(text(), 'Show more')]")).await {
          Ok(button) => {
               // if exist click on button for show more information...
               println!("Bouton trouvé avec XPath, clique numéro: {}", i);
               button.click().await?;
          }
          Err(_) => println!("Bouton introuvable"),
     }
    }
    
    // get full page sources
    // get_page(&driver).await?;
    
    // define xpath if it's page or articles search.
    let page = "//div[@id='root']/div/div[3]/div[2]/div/main/div/div[2]/div";
    let search = "//div[@id='root']/div/div[3]/div[2]/div/main/div/div/div[2]/div";
    
    // function for give parent element and give the children element extract link.
    get_element_text_xpath(&driver, search, "//a[@rel='noopener follow']").await?;

    // close the browser and release its resources
    driver.quit().await?;

    Ok(())
}


