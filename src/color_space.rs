use image::{DynamicImage, GenericImageView, Rgb, RgbImage};

pub fn rgb_to_ycbcr(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut ycbcr_img = RgbImage::new(width, height);
    
    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        
        let y_val = 0.299 * r + 0.587 * g + 0.114 * b;
        let cb = 128.0 - 0.168736 * r - 0.331264 * g + 0.5 * b;
        let cr = 128.0 + 0.5 * r - 0.418688 * g - 0.081312 * b;
        
        ycbcr_img.put_pixel(x, y, Rgb([y_val as u8, cb as u8, cr as u8]));
    }
    
    DynamicImage::ImageRgb8(ycbcr_img)
}

pub fn ycbcr_to_rgb(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rgb_img = RgbImage::new(width, height);
    
    for (x, y, pixel) in img.pixels() {
        let y_val = pixel[0] as f32;
        let cb = pixel[1] as f32 - 128.0;
        let cr = pixel[2] as f32 - 128.0;
        
        let r = y_val + 1.402 * cr;
        let g = y_val - 0.344136 * cb - 0.714136 * cr;
        let b = y_val + 1.772 * cb;
        
        rgb_img.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
    }
    
    DynamicImage::ImageRgb8(rgb_img)
}
