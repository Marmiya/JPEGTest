use image::{DynamicImage, GenericImageView, open, save_buffer_with_format, ColorType};
use std::path::Path;

pub fn read_image(file_path: &str) -> DynamicImage {
    open(file_path).unwrap()
}

pub fn save_image(img: &DynamicImage, file_path: &str) {
    let (width, height) = img.dimensions();
    let path = Path::new(file_path);
    save_buffer_with_format(
        path,
        &img.to_rgb8(),
        width,
        height,
        ColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )
    .unwrap();
}

pub fn split_into_blocks(y_matrix: &Vec<Vec<u8>>, cb_matrix: &Vec<Vec<u8>>, 
    cr_matrix: &Vec<Vec<u8>>) 
-> (Vec<[[f32; 8]; 8]>, Vec<[[f32; 8]; 8]>, Vec<[[f32; 8]; 8]>) {
    let width = y_matrix[0].len() as u32;
    let height = y_matrix.len() as u32;
    let chroma_width = cb_matrix[0].len() as u32;
    let chroma_height = cb_matrix.len() as u32;

    let mut y_blocks = Vec::new();
    let mut cb_blocks = Vec::new();
    let mut cr_blocks = Vec::new();

    for y in (0..height).step_by(8) {
        for x in (0..width).step_by(8) {
            let mut y_block = [[0.0; 8]; 8];
            for j in 0..8 {
                for i in 0..8 {
                    let px = x + i;
                    let py = y + j;
                    y_block[j as usize][i as usize] = y_matrix[py as usize][px as usize] as f32;
                }
            }
            y_blocks.push(y_block);
        }
    }

    for y in (0..chroma_height).step_by(8) {
        for x in (0..chroma_width).step_by(8) {
            let mut cb_block = [[0.0; 8]; 8];
            let mut cr_block = [[0.0; 8]; 8];
            for j in 0..8 {
                for i in 0..8 {
                    let px = x + i;
                    let py = y + j;
                    cb_block[j as usize][i as usize] = cb_matrix[py as usize][px as usize] as f32;
                    cr_block[j as usize][i as usize] = cr_matrix[py as usize][px as usize] as f32;
                }
            }
            cb_blocks.push(cb_block);
            cr_blocks.push(cr_block);
        }
    }
    
    (y_blocks, cb_blocks, cr_blocks)
}

pub fn merge_blocks(y_blocks: &Vec<[[f32; 8]; 8]>, cb_blocks: &Vec<[[f32; 8]; 8]>,
    cr_blocks: &Vec<[[f32; 8]; 8]>, orig_width: u32, orig_height: u32) 
-> (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>) {
    
    let padded_width_y = (orig_width + 15) / 16 * 16;
    let padded_height_y = (orig_height + 15) / 16 * 16;
    let padded_width_c = padded_width_y / 2;
    let padded_height_c = padded_height_y / 2;

    let mut y_matrix = vec![vec![0; padded_width_y as usize]; padded_height_y as usize];
    let mut cb_matrix = vec![vec![0; padded_width_c as usize]; padded_height_c as usize];
    let mut cr_matrix = vec![vec![0; padded_width_c as usize]; padded_height_c as usize];

    for (idx, y_block) in y_blocks.iter().enumerate() {
        let x = (idx as u32 % (padded_width_y / 8)) * 8;
        let y = (idx as u32 / (padded_width_y / 8)) * 8;
        for j in 0..8 {
            for i in 0..8 {
                let px = x + i;
                let py = y + j;
                y_matrix[py as usize][px as usize] = y_block[j as usize][i as usize] as u8;
            }
        }
    }

    for (idx, cb_block) in cb_blocks.iter().enumerate() {
        let x = (idx as u32 % (padded_width_c / 8)) * 8;
        let y = (idx as u32 / (padded_width_c / 8)) * 8;
        for j in 0..8 {
            for i in 0..8 {
                let px = x + i;
                let py = y + j;
                cb_matrix[py as usize][px as usize] = cb_block[j as usize][i as usize] as u8;
            }
        }
    }

    for (idx, cr_block) in cr_blocks.iter().enumerate() {
        let x = (idx as u32 % (padded_width_c / 8)) * 8;
        let y = (idx as u32 / (padded_width_c / 8)) * 8;
        for j in 0..8 {
            for i in 0..8 {
                let px = x + i;
                let py = y + j;
                cr_matrix[py as usize][px as usize] = cr_block[j as usize][i as usize] as u8;
            }
        }
    }

    (y_matrix, cb_matrix, cr_matrix)
}
