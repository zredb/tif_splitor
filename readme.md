此项目提供了split_image方法，把一个大的tiff图片，按照指定的大小切成小的tiff图片。
```rust
fn main(){
   let img = "./tests/images/austin1.tif";
   let out = "./tests/images/";
   split_image(img, out, 500, 500);
}
```
