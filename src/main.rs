use image::{DynamicImage, GenericImageView};
use reqwest::{self, blocking, Url};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img_bytes = reqwest::blocking::get(
        "https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx136800-WUh8ZeFNqzOv.jpg",
    )
    .unwrap()
    .bytes()
    .unwrap();
    let img = image::load_from_memory(&img_bytes).unwrap();
    // let img = getimg(
    //     "https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx136800-WUh8ZeFNqzOv.jpg",
    // );

    // // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();

    // fn a(name: &str, number: u64) {
    //     println!("{number}");
    //
    // }
    // println!("{}", img_bytes);

    // const NUM: u64 = 40;
    // const NAME: &str = "SubratGupta";
    // a(NAME, NUM);
}
