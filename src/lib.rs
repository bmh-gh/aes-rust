const S_BOX: [[u8; 16]; 16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

const UN_S_BOX: [[u8; 16]; 16] = [
    [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d],
];

pub fn split_data(data: u128) -> [u8; 16] {
    let mut byte_array: [u8; 16] = [0; 16];
    for i in 0..byte_array.len() {
        byte_array[15 - i] = (data >> (i * 8)) as u8
    }
    byte_array
}

pub fn unsplit(byte_array: [u8; 16]) -> u128 {
    let mut sum: u128 = 0;
    for i in 0..byte_array.len() {
        sum += (byte_array[i] as u128) << ((15 - i) * 8);
    }
    sum
}

pub fn sub_byte(data: [u8; 16]) -> [u8; 16] {
    let mut new: [u8; 16] = [0; 16];
    for i in 0..data.len() {
        let higher: usize = ((data[i] & 0xF0) >> 4) as usize;
        let lower: usize = (data[i] & 0xF) as usize;
        new[i] = S_BOX[higher][lower];
    }
    new
}

pub fn unsub_byte(data: [u8; 16]) -> [u8; 16] {
    let mut new: [u8; 16] = [0; 16];
    for i in 0..data.len() {
        let higher: usize = ((data[i] & 0xF0) >> 4) as usize;
        let lower: usize = (data[i] & 0xF) as usize;
        new[i] = UN_S_BOX[higher][lower];
    }
    new
}

pub fn shift_rows(data: [u8; 16]) -> [u8; 16] {

    
    [
        // Column 1
        data[0], data[5], data[10], data[15],
        // Column 2
        data[4], data[9], data[14], data[3],
        // Column 3
        data[8], data[13], data[2], data[7],
        // Column 4
        data[12], data[1], data[6], data[11]
    ]

}

fn unshift_rows(data: [u8; 16]) -> [u8; 16] {
        
    [
        // Column 1
        data[0], data[13], data[10], data[7],
        // Column 2
        data[4], data[1], data[14], data[11],
        // Column 3
        data[8], data[5], data[2], data[15],
        // Column 4
        data[12], data[9], data[6], data[3]
    ]
    
}

pub fn mix_columns(data: [u8; 16]) -> [u8; 16] {
    let b0: [u8; 4] = mix_column([data[0], data[1], data[2], data[3]]);
    let b1: [u8; 4] = mix_column([data[4], data[5], data[6], data[7]]);
    let b2: [u8; 4] = mix_column([data[8], data[9], data[10], data[11]]);
    let b3: [u8; 4] = mix_column([data[12], data[13], data[14], data[15]]);
    
    let mut result: [u8; 16] =  [0; 16];
    for i in 0..4 {
        result[i] = b0[i];
        result[i + 4] = b1[i];
        result[i + 8] = b2[i];
        result[i + 12] = b3[i];
    }
    result
}

fn mix_column(data: [u8; 4]) -> [u8; 4] {

    let mul02 = |i: u8| -> u8 {
        // 0x02 = 0b0010
        if i >= 128 {
            let p: u8 = 0b00011011;
            (i << 1) ^ p
        }
        else {
            i << 1
        }
    };

    let mul03 = |i: u8| -> u8 {
        if i >= 128 {
            let p: u8 = 0b00011011;
            ((i << 1) ^ i) ^ p
        }
        else {
            (i << 1) ^ i
        }
    };

    let c0: u8 = mul02(data[0]) ^ mul03(data[1]) ^ data[2] ^ data[3];
    let c1: u8 = data[0] ^ mul02(data[1]) ^ mul03(data[2]) ^ data[3];
    let c2: u8 = data[0] ^ data[1] ^ mul02(data[2]) ^ mul03(data[3]);
    let c3: u8 =  mul03(data[0]) ^ data[1] ^ data[2] ^ mul02(data[3]);

    [c0, c1, c2, c3]
}

fn unmix_columns(data: [u8; 16]) -> [u8; 16] {
    let b0: [u8; 4] = unmix_column([data[0], data[1], data[2], data[3]]);
    let b1: [u8; 4] = unmix_column([data[4], data[5], data[6], data[7]]);
    let b2: [u8; 4] = unmix_column([data[8], data[9], data[10], data[11]]);
    let b3: [u8; 4] = unmix_column([data[12], data[13], data[14], data[15]]);
    
    let mut result: [u8; 16] =  [0; 16];
    for i in 0..4 {
        result[i] = b0[i];
        result[i + 4] = b1[i];
        result[i + 8] = b2[i];
        result[i + 12] = b3[i];
    }
    result
}

fn unmix_column(data: [u8; 4]) -> [u8; 4] {
    fn mod_p(mut pol: u16) -> u8 {
        let p: u16 = 0b100011011;
        let p9: u16 = 0b1000110110;
        let p10: u16 = 0b10001101100;
    
        if pol >> 10 & 1 == 1 {
            pol ^= p10;
        }
        if pol >> 9 & 1 == 1  {
            pol ^= p9; 
            
        }
        if pol >> 8 & 1 == 1 {
            pol ^= p;  
        }
        pol as u8
    }

    fn mul0_e (i: u8) -> u8 {
        //0x0E = 0b1110
        mod_p(((i as u16) << 3) ^ ((i as u16) << 2) ^ ((i as u16) << 1))
    }
    fn mul0_b (i: u8) -> u8 {
        //0x0B = 1011
        mod_p(((i as u16) << 3) ^ ((i as u16) << 1) ^ ((i as u16)))
    }
    fn mul0_d (i: u8) -> u8 {
        //0x0D = 0b1101
        mod_p(((i as u16) << 3) ^ ((i as u16) << 2) ^ ((i as u16)))
    }
    fn mul0_9 (i: u8) -> u8 {
        // 0x09 = 0b0000_1001
        mod_p(((i as u16) << 3) ^ ((i as u16)))
    }

    let b0: u8 = mul0_e(data[0]) ^ mul0_b(data[1]) ^ mul0_d(data[2]) ^ mul0_9(data[3]);
    let b1: u8 = mul0_9(data[0]) ^ mul0_e(data[1]) ^ mul0_b(data[2]) ^ mul0_d(data[3]);
    let b2: u8 = mul0_d(data[0]) ^ mul0_9(data[1]) ^ mul0_e(data[2]) ^ mul0_b(data[3]);
    let b3: u8 = mul0_b(data[0]) ^ mul0_d(data[1]) ^ mul0_9(data[2]) ^ mul0_e(data[3]);
    
    [b0, b1, b2, b3]
}

pub fn key_addition(key: u128, data: u128) -> u128 {
    key ^ data
}

pub fn key_schedule(key: u128) -> [u128; 11] {
    let mut rc: u8 = 0b1;
    let mut keys: [u128; 11] = [0; 11];
    let mut words: [u32; 4] = [
       (key >> 96) as u32,
       (key >> 64) as u32,
       (key >> 32) as u32,
       (key >> 0) as u32,
    ];
    fn rot_word(word: u32) -> u32 {
        word.rotate_left(8)
    }
    let mut g = |word: u32| -> u32 {
        let mut new_word: u32 = 0;
        for i in 0..4 {
            let byte: u8 = (word >> (i * 8)) as u8;
            let higher: usize = ((byte & 0xF0) >> 4) as usize;
            let lower: usize = (byte & 0xF) as usize;
            let sub = S_BOX[higher][lower];
            new_word += (sub as u32) << (i * 8);
        }
        
        new_word ^= (rc as u32) << 24;
        rc = increase(rc);
        new_word
    };
    fn increase(rc: u8) -> u8 {
        let new_rc = rc << 1;
        if rc >= 128 {
            let p: u8 = 0b00011011;
            new_rc ^ p
        }
        else {
            new_rc
        }
    }
    let mut add_key = |w: [u32; 4], index: usize| {
        keys[index] = ((w[0] as u128) << 96) + ((w[1] as u128) << 64) + ((w[2] as u128) << 32) + (w[3] as u128)
    };
    add_key(words, 0);
    for i in 1..11 {
        let func = g(rot_word(words[3]));
        let w0 = words[0] ^ func;
        let w1 = words[1] ^ w0;
        let w2 = words[2] ^ w1;
        let w3 = words[3] ^ w2;
        words = [w0, w1, w2, w3];
        add_key(words, i);
    }
    keys
}

fn encrypt(data: u128, key: u128) -> u128 {
    let keys: [u128; 11] = key_schedule(key);
    let mut data = key_addition(keys[0], data);
    for i in 1..10 {
        data = key_addition(keys[i], unsplit(mix_columns(shift_rows(sub_byte(split_data(data))))));
    }
    key_addition(keys[10], unsplit(shift_rows(sub_byte(split_data(data)))))
}

fn decrypt(data: u128, key: u128) -> u128 {
    let keys: [u128; 11] = key_schedule(key);
    let mut data = key_addition(keys[10], unsplit(unshift_rows(unsub_byte(split_data(data)))));
    for i in (1..10).rev() {
        data = key_addition(
            keys[i], 
            unsplit(sub_byte(unshift_rows(unmix_columns(split_data(data)))))
        )
    }
    key_addition(keys[0], data)
}

#[cfg(test)]
mod tests {
    use crate::{split_data, unsplit, shift_rows, unshift_rows, key_addition, sub_byte, unsub_byte, mix_column, key_schedule, encrypt, unmix_column, decrypt};
    use rand::Rng;

    #[test]
    fn test_split() {
        let data: u128 = u128::MAX;
        let result = split_data(data);
        let should: [u8; 16] = [0b11111111; 16];
        assert_eq!(result, should);

        let data: u128 = 0;
        let result = split_data(data);
        let should: [u8; 16] = [0; 16];
        assert_eq!(result, should);

        let data: u128 = 0x00010203040506070809101112131415;
        let result: [u8; 16] = split_data(data);
        let should: [u8; 16] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15];
        assert_eq!(result, should)
    }

    #[test]
    fn test_unsplit() {
        let data: [u8; 16] = [0b11111111; 16];
        let result = unsplit(data);
        let should:u128  = u128::MAX;
        assert_eq!(result, should);
    }

    #[test]
    fn test_shift() {
        let data: [u8; 16] = [
            0, 1, 2, 3 ,4 ,5 ,6 ,7, 8, 9, 10, 11, 12, 13, 14, 15
        ];
        let result: [u8; 16] = shift_rows(data);
        let should: [u8; 16] = [
                0, 5, 10, 15,
                4, 9, 14, 3,
                8, 13, 2, 7,
                12, 1, 6, 11
        ];
        assert_eq!(result, should);
        //unshift
        let invert_shift = unshift_rows(result);
        assert_eq!(invert_shift, data);
    }

    // Uses Test Cases from the official NIST

    #[test]
    fn test_key_add() {
        //Round: 0
        let data: u128 = 0x3243f6a8885a308d313198a2e0370734;
        let key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
        let result = key_addition(key, data);
        let should: u128 = 0x193de3bea0f4e22b9ac68d2ae9f84808;
        assert_eq!(result, should);

        //Round: 1
        let first_subkey = key_schedule(key)[1];
        println!("{:x}", first_subkey);
        let data: u128 = 0x046681e5e0cb199a48f8d37a2806264c;
        let result: u128 = 0xa49c7ff2689f352b6b5bea43026a5049;
        let actual: u128 = key_addition(first_subkey, data);
        assert_eq!(result, actual)
    }

    #[test]
    fn test_sub_bytes() {
        //Round: 1
        let data: u128 = 0x193de3bea0f4e22b9ac68d2ae9f84808;
        let bytes = split_data(data);
        let result = sub_byte(bytes);
        let actual:[u8; 16] = [
            0xd4, 0x27, 0x11, 0xae,
            0xe0, 0xbf, 0x98, 0xf1,
            0xb8, 0xb4, 0x5d, 0xe5,
            0x1e, 0x41, 0x52, 0x30
        ];
        assert_eq!(result, actual);

        let result = unsub_byte(actual);
        assert_eq!(data, unsplit(result))
    }

    fn test_shift_rows() {
        //Round: 1
        let data:[u8; 16] = [
            0xd4, 0x27, 0x11, 0xae,
            0xe0, 0xbf, 0x98, 0xf1,
            0xb8, 0xb4, 0x5d, 0xe5,
            0x1e, 0x41, 0x52, 0x30
        ];
        let result = shift_rows(data);
        let actual: [u8; 16] = [
             0xd4, 0xbf, 0x5d, 0x30, 
             0xe0, 0xb4, 0x52, 0xae,
             0xb8, 0x41, 0x11, 0xf1,
             0x1e, 0x27, 0x98, 0xe5
        ];
        assert_eq!(result, actual)
    }
    
    #[test]
    fn test_mix_columns() {
        //Round: 1
        let data: [u8; 16] = [
             0xd4, 0xbf, 0x5d, 0x30, 
             0xe0, 0xb4, 0x52, 0xae,
             0xb8, 0x41, 0x11, 0xf1,
             0x1e, 0x27, 0x98, 0xe5
        ];
        let b0: [u8; 4] = [data[0], data[1], data[2], data[3]];
        let b1: [u8; 4] = [data[4], data[5], data[6], data[7]];
        let b2: [u8; 4] = [data[8], data[9], data[10], data[11]];
        let b3: [u8; 4] = [data[12], data[13], data[14], data[15]];

        let c0 = mix_column(b0);
        let actual_c0: [u8; 4] = [0x04, 0x66, 0x81, 0xe5];
        assert_eq!(c0, actual_c0);

        let new_b0: [u8; 4] = unmix_column(c0);
        assert_eq!(b0, new_b0);

        let c1 = mix_column(b1);
        let actual_c1: [u8; 4] = [0xe0, 0xcb, 0x19, 0x9a];
        assert_eq!(c1, actual_c1);

        let c2 = mix_column(b2);
        let actual_c2: [u8; 4] = [0x48, 0xf8, 0xd3, 0x7a];
        assert_eq!(c2, actual_c2);

        let c3 = mix_column(b3);
        let actual_c3: [u8; 4] = [0x28, 0x06, 0x26, 0x4c];
        assert_eq!(c3, actual_c3);
        
    }
    #[test]
    fn test_encryption() {
        let input: u128 = 0x3243f6a8885a308d313198a2e0370734;
        let main_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
        let result: u128 = encrypt(input, main_key);
        println!("{:x}", result);
        let actual: u128 = 0x3925841d02dc09fbdc118597196a0b32;
        println!("{:x}", actual);

        assert_eq!(result, actual);
    }

    fn test_decryption() {
        let input: u128 = 0x3925841d02dc09fbdc118597196a0b32;
        let main_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
        let result: u128 = decrypt(input, main_key);
        println!("{:x}", result);
        let actual: u128 = 0x3243f6a8885a308d313198a2e0370734;
        println!("{:x}", actual);

        assert_eq!(result, actual);
    }

    fn test_cryptosys() {
        for _ in 0..=1000 {
            let input: u128 = rand::thread_rng().gen_range(0 .. u128::MAX);
            let key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
            assert_eq!(input, encrypt(decrypt(input, key), key))
        }
    }
}
