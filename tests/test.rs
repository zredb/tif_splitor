use tif_splitor::{split_image};

#[test]
fn it_can_works() {
   let img = "./tests/images/austin1.tif";
   let out = "./tests/images/";
   split_image(img, out, 500, 500);
}