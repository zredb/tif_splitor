use std::fs;
use std::fs::{File};
use std::path::{Path, PathBuf};
use tiff::decoder::Decoder;
use tiff::decoder::DecodingResult::U8;
use tiff::encoder::{colortype, TiffEncoder};
use tiff::tags::Tag;
use ndarray::Array;
use tiff::ColorType;
use walkdir::{WalkDir};


pub fn split_images(input: &str, output: &str, height: u32, width: u32) {
   fs::create_dir_all(output).unwrap();
   let walker = WalkDir::new(input).into_iter();
   for entry in walker {
      let entry = entry.unwrap();
      println!("entry path: {}", entry.path().display());
      let full_file_path = entry.path().as_os_str().to_str().unwrap();
      if full_file_path.ends_with(".tif") {
         display(&full_file_path);
         split_image(full_file_path, output, height, width);
      }
   }
}

fn display(full_file_path: &str) {
   let img_file = File::open(full_file_path).expect("Cannot find test image!");
   println!("file: {:?}", &img_file);
   let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
   println!("color type: {:?}", decoder.colortype().unwrap());
   println!("dimension: {:?}", decoder.dimensions().unwrap());
   println!("byte order: {:?}", decoder.byte_order());
   println!("chunk_dimensions: {:?}", decoder.chunk_dimensions());
   println!("more images: {:?}", decoder.more_images());
   println!("strip_count: {:?}", decoder.strip_count().unwrap());
   // let image = decoder.read_image().unwrap();
   // println!("image: {:?}", image);
   println!("___________________________________________________", );
}

///image file's color type is RGB(8);
/// file dimension: (5000, 5000), need split to (500,500)
pub fn split_image(full_file_path: &str, out_path: &str, height: u32, width: u32) {
   let img_file = File::open(full_file_path).expect("Cannot find test image!");
   println!("file: {:?}", &img_file);
   let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
   let image_data = decoder.read_image().expect("read image error");
   let (h, w) = decoder.dimensions().unwrap();
   let color_type = decoder.colortype().unwrap();
   let image_data = match image_data {
      U8(data) => data,
      _ => panic!("not support type")
   };
   let out_file = Path::new(full_file_path).file_stem().unwrap();
   let ch: u32 = match color_type {
      ColorType::RGB(_) => 3,
      ColorType::Gray(_) => 1,
      _ => unimplemented!()
   };
   let shape = (h as usize, (w * ch) as usize);
   let image_data = Array::from_shape_vec(shape, image_data).expect("to ndarray error");
   let mut vec = Vec::new();
   let h_count = h / height;
   let w_count = w / width;
   for x in 0..w_count {
      let xs = width * x;
      let xe = xs + width;
      for y in 0..h_count {
         let ys = height * ch * y;
         let ye = ys + height * ch;
         for i in xs..xe {
            for j in ys..ye {
               vec.push(image_data[[i as usize, j as usize]]);
            }
         }
         let fname = format!("{}_{}_{}.tif", out_file.to_str().unwrap(), x, y);
         //println!("fname: {:?}", fname);
         let full_path = PathBuf::from(out_path).join(fname);
         println!("write: {:?}", full_path);
         println!("data length: {}", vec.len());
         write_splited_image(full_path, color_type, &vec, height, width);
         vec.clear();
      }
   }
}

fn write_splited_image(file: PathBuf, color_type: ColorType, image_data: &Vec<u8>, height: u32, width: u32) {
   let mut file = match File::create(file) {
      Err(why) => panic!("couldn't create file: {:?}", why),
      Ok(file) => file,
   };
   let mut tiff = TiffEncoder::new_big(&mut file).unwrap();

   match color_type {
      ColorType::RGB(_) => {
         let mut image = tiff.new_image::<colortype::RGB8>(width, height).unwrap();
         image
            .encoder()
            .write_tag(Tag::Artist, "Image-tiff")
            .unwrap();
         image.write_data(&image_data).unwrap();
      }
      ColorType::Gray(_) => {
         let mut image = tiff.new_image::<colortype::Gray8>(width, height).unwrap();
         image
            .encoder()
            .write_tag(Tag::Artist, "Image-tiff")
            .unwrap();
         image.write_data(&image_data).unwrap();
      }
      _ => unimplemented!()
   };
}
