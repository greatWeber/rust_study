use std::error::Error;

use scraper::{Html, Selector};


/**
 * 页面爬虫，但是单页面不适用
 */
pub async fn reptile(url: &str)-> Result<Vec<String>, Box<dyn Error>>{
  // 获取页面内容
  let body = reqwest::get(url).await?.text().await?;
  let document = Html::parse_document(&body);

  let selector = Selector::parse("img").unwrap();

  let mut result: Vec<String> = Vec::new();

  for el in document.select(&selector) {
      // println!("el:{:#?}", el);
      let img = el.value().attr("src")
                              .filter(|&src| !src.is_empty())
                              .or_else(|| el.value().attr("data-src").filter(|&data_src| !data_src.is_empty()))
                              .unwrap_or("")
                              .to_string();
      if !img.is_empty() {
        // println!("image: {:#?}", img);
        result.push(img);
      }
      
  };
  // println!("image: {:#?}", &result);
  Ok(result)
}