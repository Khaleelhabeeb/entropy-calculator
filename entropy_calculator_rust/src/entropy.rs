pub const BUFFER_SIZE: usize = 256;

pub fn count_bits_set(byte: u8) -> u8 {
    byte.count_ones() as u8
}

pub fn calculate_byte_level_entropy(counts: &[u32; 256], total_bytes: u64) -> f64 {
    if total_bytes == 0 {
        return 0.0;
    }
    
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let prob = count as f64 / total_bytes as f64;
            entropy -= prob * prob.log2();
        }
    }
    entropy
}

pub fn calculate_bit_level_entropy(counts: &[u32; 256], total_bytes: u64) -> f64 {
    if total_bytes == 0 {
        return 0.0;
    }
    
    let mut bit_counts = [0u32; 9]; // 0 to 8 bits set
    
    for (byte_value, &count) in counts.iter().enumerate() {
        if count > 0 {
            let bits_set = count_bits_set(byte_value as u8);
            bit_counts[bits_set as usize] += count;
        }
    }

    let mut bit_entropy = 0.0;
    for &bit_count in bit_counts.iter() {
        if bit_count > 0 {
            let prob = bit_count as f64 / total_bytes as f64;
            bit_entropy -= prob * prob.log2();
        }
    }
    bit_entropy
}

