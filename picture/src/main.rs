mod image;
mod reptile;

use std::error::Error;

    
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let img_list = reptile::reptile("https://wallhaven.cc/search?categories=010&purity=100&topRange=1M&sorting=toplist&order=desc&page=2").await?;
    
    // 通过多线程下载图片
    let handles: Vec<_> = img_list.into_iter().filter(|img| !img.is_empty()).map(|img| {
        println!("img: {}", img);
        tokio::spawn(async move {
            match image::download_img(&img).await {
                Ok(_) => println!("Downloaded: {}", img),
                Err(e) => eprintln!("Error downloading {}: {}", img, e),
            }
        })
    }).collect();
    
    for handle in handles {
        let _ = handle.await;
    }
    
    
   
    Ok(())
}

