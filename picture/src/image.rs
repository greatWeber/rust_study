use std::{error::Error, fs::{self, read_dir}, path::Path};
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
/**
 * 查找目录下的所有图片
 */
pub fn find_img(path : &str)-> Result<Vec<String>,Box<dyn Error>> {
  // 定义要搜索的目录
  let dir = Path::new(path);

  // 获取目录下的所有文件
  let files = read_dir(dir).unwrap();

  let images: Vec<_> = files.filter_map(|file| file.ok())
  .filter(|file| {
    if let Some(ext) = file.path().extension() {
      ext == "jpg" || ext == "png" || ext == "jpeg"
    }else {
      false
    }
  }).collect();

  let mut result = Vec::new();

  // 打印所有的图片路径
  for img in images {
    let img_path =img.path().display().to_string();
    // println!("img path: {}",img_path);
    
    result.push(img_path);
  }

  Ok(result)
}