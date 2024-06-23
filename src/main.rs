mod color_space;
mod downsampling;
mod dct;
mod quantization;
mod encoding_decoding;
mod utils;

use std::fs;
use std::path::Path;
use utils::{read_image, save_image, split_into_blocks, merge_blocks};
use encoding_decoding::{huffman_encode, LUMINANCE_AC_HUFFMAN_CODES, 
    LUMINANCE_DC_HUFFMAN_CODES, CHROMINANCE_AC_HUFFMAN_CODES, CHROMINANCE_DC_HUFFMAN_CODES};

fn main() {

    // Read input dir from args, if no args, use default "Kodak24"
    let args: Vec<String> = std::env::args().collect();
    let input_dir = if args.len() > 1 {
        &args[1]
    } else {
        "Kodak24"
    };
    
    // Process all PNG images in the input directory
    
    let output_dir = "CompressedImages";

    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir).unwrap();
    }

    for entry in fs::read_dir(input_dir).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().unwrap() == "png" {
            let img = read_image(file_path.to_str().unwrap());
            // 1. Convert image to YCbCr color space and downsample chroma channels
            let ycbcr_img = color_space::rgb_to_ycbcr(&img);

            // // Test:
            // // output the YCbCr image for demonstration
            // let output_file_path = format!("{}/{}_ycbcr.jpg", output_dir, 
            // file_path.file_stem().unwrap().to_str().unwrap());
            // save_image(&ycbcr_img, &output_file_path);

            // 2. Downsample chroma channels
            let (y_matrix, cb_matrix, 
                cr_matrix, orig_width, orig_height) 
                = downsampling::chroma_downsample(&ycbcr_img);
            
            // Test:
            // output the downsampled image for demonstration
            // let output_file_path = format!("{}/{}_downsampled.jpg", 
            // output_dir, file_path.file_stem().unwrap().to_str().unwrap());
            // let restored_img = downsampling::chroma_reshore
            // (&y_matrix, &cb_matrix, &cr_matrix, orig_width, orig_height);
            // let restored_img = color_space::ycbcr_to_rgb(&restored_img);
            // save_image(&restored_img, &output_file_path);

            // 3. Split image into 8x8 blocks and apply DCT to each block
            let (y_blocks, cb_blocks, cr_blocks) 
            = split_into_blocks(&y_matrix, &cb_matrix, &cr_matrix);
            let dct_y_blocks: Vec<_> = y_blocks.iter()
            .map(|block| dct::dct_transform(block)).collect();
            let dct_cb_blocks: Vec<_> = cb_blocks.iter()
            .map(|block| dct::dct_transform(block)).collect();
            let dct_cr_blocks: Vec<_> = cr_blocks.iter()
            .map(|block| dct::dct_transform(block)).collect();

            // 4. Quantize DCT coefficients
            let l_quant_table = quantization::LUMINANCE_QUANT_TABLE;
            let c_quant_table = quantization::CHROMINANCE_QUANT_TABLE;
            let quant_y_blocks: Vec<_> = dct_y_blocks.iter()
            .map(|block| quantization::quantize(&block, &l_quant_table)).collect();
            let quant_cb_blocks: Vec<_> = dct_cb_blocks.iter()
            .map(|block| quantization::quantize(&block, &c_quant_table)).collect();
            let quant_cr_blocks: Vec<_> = dct_cr_blocks.iter()
            .map(|block| quantization::quantize(&block, &c_quant_table)).collect();
            
            // 5. Encode quantized coefficients using run-length encoding and Huffman coding
            // change to zigzag scan
            let mut zigzag_y_blocks: Vec<_> = quant_y_blocks.iter()
            .map(|block| encoding_decoding::zigzag_scan(block)).collect();
            let mut zigzag_cb_blocks: Vec<_> = quant_cb_blocks.iter()
            .map(|block| encoding_decoding::zigzag_scan(block)).collect();
            let mut zigzag_cr_blocks: Vec<_> = quant_cr_blocks.iter()
            .map(|block| encoding_decoding::zigzag_scan(block)).collect();

           
            let encoded_y_data = huffman_encode(&mut zigzag_y_blocks,
                 &LUMINANCE_DC_HUFFMAN_CODES, &LUMINANCE_AC_HUFFMAN_CODES);
            let encoded_cb_data = huffman_encode(&mut zigzag_cb_blocks,
                 &CHROMINANCE_DC_HUFFMAN_CODES, &CHROMINANCE_AC_HUFFMAN_CODES);   
            let encoded_cr_data = huffman_encode(&mut zigzag_cr_blocks,
                 &CHROMINANCE_DC_HUFFMAN_CODES, &CHROMINANCE_AC_HUFFMAN_CODES);

            // 6. Decode and reconstruct image
            let decoded_y_data = encoding_decoding::huffman_decode(&encoded_y_data, 
                &LUMINANCE_DC_HUFFMAN_CODES, &LUMINANCE_AC_HUFFMAN_CODES);
            let decoded_cb_data = encoding_decoding::huffman_decode(&encoded_cb_data,
                 &CHROMINANCE_DC_HUFFMAN_CODES, &CHROMINANCE_AC_HUFFMAN_CODES);
            let decoded_cr_data = encoding_decoding::huffman_decode(&encoded_cr_data,
                 &CHROMINANCE_DC_HUFFMAN_CODES, &CHROMINANCE_AC_HUFFMAN_CODES);

            // unzigzag 
            let unzigzag_y_blocks: Vec<_> = decoded_y_data.iter()
            .map(|block| encoding_decoding::zigzag_unscan(block)).collect();
            let unzigzag_cb_blocks: Vec<_> = decoded_cb_data.iter()
            .map(|block| encoding_decoding::zigzag_unscan(block)).collect();
            let unzigzag_cr_blocks: Vec<_> = decoded_cr_data.iter()
            .map(|block| encoding_decoding::zigzag_unscan(block)).collect();

            // dequantize
            let dequant_y_blocks: Vec<_> = unzigzag_y_blocks.iter()
            .map(|block| quantization::dequantize(block, &l_quant_table)).collect();
            let dequant_cb_blocks: Vec<_> = unzigzag_cb_blocks.iter()
            .map(|block| quantization::dequantize(block, &c_quant_table)).collect();
            let dequant_cr_blocks: Vec<_> = unzigzag_cr_blocks.iter()
            .map(|block| quantization::dequantize(block, &c_quant_table)).collect();

            // idct
            let idct_y_blocks: Vec<_> = dequant_y_blocks.iter()
            .map(|block| dct::idct_transform(block)).collect();
            let idct_cb_blocks: Vec<_> = dequant_cb_blocks.iter()
            .map(|block| dct::idct_transform(block)).collect();
            let idct_cr_blocks: Vec<_> = dequant_cr_blocks.iter()
            .map(|block| dct::idct_transform(block)).collect();

            // merge blocks
            let (restored_y_matrix, restored_cb_matrix,
                 restored_cr_matrix) = 
                 merge_blocks(&idct_y_blocks, &idct_cb_blocks, &idct_cr_blocks,orig_width, orig_height);
            let restored_img = downsampling::chroma_reshore(
                &restored_y_matrix, &restored_cb_matrix, &restored_cr_matrix, orig_width, orig_height);

            let final_img = color_space::ycbcr_to_rgb(&restored_img);

            let output_file_path = format!("{}/{}_compressed.jpg", 
            output_dir, file_path.file_stem().unwrap().to_str().unwrap());
            save_image(&final_img, &output_file_path);
        }
    }
}
