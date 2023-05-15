const SBOX: [u32; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

fn read_cleartext() -> Vec<Vec<u8>> {
    let input = include_str!("../data/cleartext.txt");

    // Read the file line by line, split by spaces, and store in a vector
    let mut data: Vec<Vec<_>> = Vec::new();
    for line in input.lines() {
        let tokens: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        data.push(tokens);
    }

    return data;
}

// Fun
fn read_trace(trace_number: u8) -> Vec<Vec<f64>> {
    if trace_number > 15 {
        panic!("Trace number must be between 0 and 15");
    }

    let input = std::fs::read_to_string(format!("./data/trace{}.txt", trace_number)).unwrap();

    // Read the file line by line, split by spaces, and store in a vector
    let mut data: Vec<Vec<f64>> = Vec::new();
    for line in input.lines() {
        let tokens: Vec<f64> = line
            .split_whitespace()
            .map(|s: &str| s.parse().unwrap())
            .collect();

        data.push(tokens);
    }

    return data;
}

fn generate_hw_model(plaintext_bytes: &Vec<u8>) -> Vec<Vec<u32>> {
    let mut hw_model: Vec<Vec<_>> = Vec::new();

    for plaintext in plaintext_bytes.iter() {
        let mut hw_line: Vec<_> = Vec::new();

        for key in 0..=255 {
            let xor = plaintext ^ key;
            let hamming_weight = SBOX[xor as usize].count_ones();
            hw_line.push(hamming_weight);
        }

        hw_model.push(hw_line);
    }

    return hw_model;
}

fn pearson_correlation_coefficient(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    // Compute the mean of x
    let mean_x: f64 = x.iter().sum::<f64>() / x.len() as f64;

    // Compute the mean of y
    let mean_y: f64 = y.iter().sum::<f64>() / y.len() as f64;

    // Compute the numerator
    let mut numerator: f64 = 0.0;
    for i in 0..x.len() {
        numerator += (x[i] - mean_x) * (y[i] - mean_y);
    }

    // Compute the denominator
    let mut denominator_x: f64 = 0.0;
    let mut denominator_y: f64 = 0.0;
    for i in 0..x.len() {
        denominator_x += (x[i] - mean_x).powi(2);
        denominator_y += (y[i] - mean_y).powi(2);
    }

    // Compute the denominator
    let denominator: f64 = (denominator_x * denominator_y).sqrt();

    // Compute the pearson correlation coefficient
    let pcc: f64 = numerator / denominator;

    return pcc;
}

fn compute_key(hw_model: &Vec<Vec<u32>>, trace: &Vec<Vec<f64>>) -> usize {
    let mut max_correlation = 0.0;
    let mut key = 0;

    // for time_instant in 0..trace[0].len() {
    for time_instant in 0..4960 {
        // for time_instant in 0.. {
        // Construct time instant vector
        let mut plaintext_vector: Vec<f64> = Vec::new();
        for i in 0..trace.len() {
            plaintext_vector.push(trace[i][time_instant] as f64);
        }

        // Iterate over all keys 0-255
        for i in 0..=255 {
            let mut hw_vector: Vec<f64> = Vec::new();
            for j in 0..hw_model.len() {
                hw_vector.push(hw_model[j][i] as f64);
            }

            let correlation = pearson_correlation_coefficient(&hw_vector, &plaintext_vector);
            if correlation > max_correlation {
                max_correlation = correlation;
                key = i;
            }
        }
    }

    println!(" ===> Key byte {}: {}", key, max_correlation);

    return key;
}

fn main() {
    let cleartex = read_cleartext();

    // Each column represents 150 plaintext bytes
    let mut plaintext_bytes: Vec<Vec<u8>> = Vec::new();
    for i in 0..cleartex[0].len() {
        let mut column: Vec<u8> = Vec::new();
        for line in &cleartex {
            column.push(line[i]);
        }
        plaintext_bytes.push(column);
    }

    // Range 16 times
    let mut keys: Vec<_> = Vec::new();
    for i in 0..16 {
        let hw_model = generate_hw_model(&plaintext_bytes[i]);
        let trace = read_trace(i as u8);

        println!("[*] Computing key for key byte {} ...", i);
        let key = compute_key(&hw_model, &trace);
        keys.push(key);
        println!("[*] K[{}]: {}", i, key)
    }

    // Print the 16 byte keys
    println!("[*] Keys: {:?}", keys);
}
