use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};

// This function divides the ycrcb image into 3 matrixes 
// and then we downsample the chroma channels
// Input: DynamicImage
// Output: matrixes of u8. For y channel, the size is the same as the input image. 
// For cb and cr channels, the size is half of the input image.

pub fn chroma_downsample(img: &DynamicImage) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>, u32, u32) {
    let (width, height) = img.dimensions();
    // For the input image whose size is not multiple of 16, we pad the image with 0s.
    let padded_width = (width + 15) / 16 * 16;
    let padded_height = (height + 15) / 16 * 16;

    let mut y_matrix = vec![vec![0; padded_width as usize]; padded_height as usize];
    let mut cb_matrix = vec![vec![0; padded_width as usize / 2]; padded_height as usize / 2];
    let mut cr_matrix = vec![vec![0; padded_width as usize / 2]; padded_height as usize / 2];

    for y in (0..padded_height).step_by(2) {
        for x in (0..padded_width).step_by(2) {

            let pixel1 = get_pixel_or_default(&img, x, y, width, height);
            let pixel2 = get_pixel_or_default(&img, x + 1, y, width, height);
            let pixel3 = get_pixel_or_default(&img, x, y + 1, width, height);
            let pixel4 = get_pixel_or_default(&img, x + 1, y + 1, width, height);

            let avg_cb = (pixel1[1] as u16 + pixel2[1] as u16 + pixel3[1] as u16 + pixel4[1] as u16) / 4;
            let avg_cr = (pixel1[2] as u16 + pixel2[2] as u16 + pixel3[2] as u16 + pixel4[2] as u16) / 4;

            y_matrix[y as usize][x as usize] = pixel1[0];
            y_matrix[y as usize][x as usize + 1] = pixel2[0];
            y_matrix[y as usize + 1][x as usize] = pixel3[0];
            y_matrix[y as usize + 1][x as usize + 1] = pixel4[0];
            cb_matrix[y as usize / 2][x as usize / 2] = avg_cb as u8;
            cr_matrix[y as usize / 2][x as usize / 2] = avg_cr as u8;
        }
    }

    (y_matrix, cb_matrix, cr_matrix, width, height)
}

// This function merges the 3 matrixes into a single image
// Input: 3 matrixes of u8 and the original width and height of the image
// Output: DynamicImage
pub fn chroma_reshore(y_matrix: &Vec<Vec<u8>>, cb_matrix: &Vec<Vec<u8>>, 
    cr_matrix: &Vec<Vec<u8>>, orig_width: u32, orig_height: u32) -> DynamicImage {
    
    let mut img = RgbImage::new(orig_width, orig_height);

    for y in (0..orig_width).step_by(2) {
        for x in (0..orig_height).step_by(2) {
            let y1 = y_matrix[y as usize][x as usize];
            let y2 = y_matrix[y as usize][x as usize + 1];
            let y3 = y_matrix[y as usize + 1][x as usize];
            let y4 = y_matrix[y as usize + 1][x as usize + 1];
            let cb = cb_matrix[y as usize / 2][x as usize / 2];
            let cr = cr_matrix[y as usize / 2][x as usize / 2];

            img.put_pixel(x, y, Rgb([y1, cb, cr]));
            img.put_pixel(x + 1, y, Rgb([y2, cb, cr]));
            img.put_pixel(x, y + 1, Rgb([y3, cb, cr]));
            img.put_pixel(x + 1, y + 1, Rgb([y4, cb, cr]));
        }
    }

    DynamicImage::ImageRgb8(img)
}

fn get_pixel_or_default(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Rgb<u8> {
    if x < width && y < height {
        img.get_pixel(x, y).to_rgb()
    } else {
        Rgb([0, 0, 0])
    }
}