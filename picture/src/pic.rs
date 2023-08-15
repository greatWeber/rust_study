use std::{error::Error, fs::{self, read_dir}, path::Path};
use image::{GenericImageView, DynamicImage, ImageBuffer};
use reqwest::Url;
use tokio::{fs::File as AsyncFile, io::AsyncWriteExt};
use rand::Rng;

const PATH: &'static str = "image";

#[derive(Debug)]
  struct Pic {
    width:u32, height:u32
  }

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

  if !dir.exists() {
    return Err(From::from("没有该目录"));
  }

  // 获取目录下的所有文件
  let files = read_dir(dir)?;

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
  let binding = Path::new(PATH).join("pic.jpg");
  let save_path =  binding.to_str();
  synthesis_pic(&result,3,&save_path.unwrap());

  Ok(result)
}
/**
 * 多张图片合成一个
 */
pub fn synthesis_pic(img_list: &Vec<String> ,x:i32,path : &str) -> Result<(),Box<dyn Error>> {
  let count = img_list.len() ;
  let count = count as i32 ;
  // let count = 13;
  let y = (count as f64 / x as f64).ceil() as i32;
  println!("x:{x}; y:{y}");

  

  let (info_list, rect_list) = get_img_info(&img_list).unwrap();

  let (max_width,max_height) = get_max_value(& rect_list,count,x,y);

  let mut new_pic = ImageBuffer::new(max_width,max_height);

  let mut width = 0;
  let mut height = 0;


  for j in 0..y {
    for i in 0..x {
      let index = j * x +i;
      if index >= count {
        break;
      }
      let last_index = index - 1;
      let rect = if last_index >=0 {
        &rect_list[last_index as usize]
      }else {
        &Pic{width:0,height:0}
      };

      width+=rect.width;
      if index % x == 0 && index!=0 {
        width = 0;
        height+=rect.height;
      }


      let info = &info_list[index as usize];
      // println!("index: {:?}; width:{:?}; height:{:?}",index,width,height);
      // 将两张图片复制到新的图像缓冲区中
      for (x, y, pixel) in info.pixels() {
        new_pic.put_pixel(x+width, y+height, pixel);
      }
    }
  }

  new_pic.save(path).unwrap();

  Ok(())
}

/**
 * 获取图片信息
 */
fn get_img_info(img_list: &Vec<String>)-> Result<((Vec<DynamicImage>, Vec<Pic>)), Box<dyn Error>> {
  let mut info_list = Vec::new();
  let mut rect_list = Vec::new();
  for img in img_list.iter() {
    let info = image::open(&img).unwrap();
    let (width, height) = info.dimensions();
    rect_list.push(Pic{width: width, height: height});
    info_list.push(info);
  }
 Ok((info_list, rect_list)) 
}
/**
 * 获取最大的宽高
 */
fn get_max_value (source: &Vec<Pic>,count:i32,x:i32,y:i32)-> (u32,u32) {
  let mut max_width = 0;
  let mut max_height = 0;

  let mut width = 0;
  let mut height = 0;


    for i in 0..count {
      width += source[i as usize].width;
      if i % (x-1) ==0 && i != 0 {
        max_width = if width > max_width {
          width
        }else {
          max_width
        };
        width = 0;
      }
    }

    for i in 0..x {
      height = 0;
      for j in 0..y {
        let index = (j * x +i) as usize;
        if index as i32 >= count {
          break;
        }
        height += source[index].height;
        max_height = if height > max_height {
          height
        }else {
          max_height
        };
      }
    }
    println!("max_width: {max_width}; max_height:{max_height}" );

    (max_width, max_height)    
}