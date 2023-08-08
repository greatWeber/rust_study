use std::{error::Error, fs, path::Path};
use reqwest::Url;
use tokio::{fs::File as AsyncFile, io::AsyncWriteExt};

const PATH: &'static str = "image";

/**
 * 通过链接下载图片到本地
 */
pub async fn download_img(url: &str) -> Result<(), Box<dyn Error>> {
  let url = Url::parse(url)?;
  let response = reqwest::get(url).await?;
  fs::create_dir_all(PATH)?;
  let fname = response
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) })
      .unwrap_or("tmp.bin");
  let file_path = Path::new(PATH).join(fname);

  if !file_path.exists() {
    println!("file download: {:?}", fname);
    let mut dest = AsyncFile::create(file_path).await?;
    let mut content = response.bytes().await?;
    dest.write_all(&mut content).await?;
  }else {
    println!("file exists: {:?}", fname);
  }
  
  
  Ok(())
}