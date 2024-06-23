pub const LUMINANCE_QUANT_TABLE: [[u8; 8]; 8] = [
    [16, 11, 10, 16, 24, 40, 51, 61],
    [12, 12, 14, 19, 26, 58, 60, 55],
    [14, 13, 16, 24, 40, 57, 69, 56],
    [14, 17, 22, 29, 51, 87, 80, 62],
    [18, 22, 37, 56, 68, 109, 103, 77],
    [24, 35, 55, 64, 81, 104, 113, 92],
    [49, 64, 78, 87, 103, 121, 120, 101],
    [72, 92, 95, 98, 112, 100, 103, 99]
];

pub const CHROMINANCE_QUANT_TABLE: [[u8; 8]; 8] = [
    [17, 18, 24, 47, 99, 99, 99, 99],
    [18, 21, 26, 66, 99, 99, 99, 99],
    [24, 26, 56, 99, 99, 99, 99, 99],
    [47, 66, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99]
];

pub fn quantize(block: &[[f32; 8]; 8], quant_table: &[[u8; 8]; 8]) -> [[i32; 8]; 8] {
    let mut quantized_block = [[0i32; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            quantized_block[i][j] = (block[i][j] / quant_table[i][j] as f32).round() as i32;
        }
    }
    quantized_block
}

pub fn dequantize(block: &[[i32; 8]; 8], quant_table: &[[u8; 8]; 8]) -> [[f32; 8]; 8] {
    let mut dequantized_block = [[0f32; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            dequantized_block[i][j] = block[i][j] as f32 * quant_table[i][j] as f32;
        }
    }
    dequantized_block
}