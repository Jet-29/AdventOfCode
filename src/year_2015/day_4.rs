use crate::{DayResult, PuzzleInput};

const PRE_COMP_TABLE_S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const PRE_COMP_TABLE_K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

pub fn solution(mut puzzle: PuzzleInput) -> DayResult {
    let puzzle = puzzle.remove(0);
    let input = puzzle.iter().collect::<String>();
    let mut counter: u32 = 0;
    let mut five_zeros_value = 0;

    loop {
        let hash_input = format!("{input}{counter}").as_bytes().to_vec();
        let hash = calculate_md5(hash_input);
        let formatted_hash = format!("{hash:x}");
        let leading_zeros = 32 - formatted_hash.len();
        if leading_zeros == 5 && five_zeros_value == 0 {
            five_zeros_value = counter;
        } else if leading_zeros == 6 {
            break;
        }
        counter += 1;
    }

    let mut result = DayResult::new();
    result.set_star1(five_zeros_value.to_string());
    result.set_star2(counter.to_string());
    result
}

fn calculate_md5(mut input: Vec<u8>) -> u128 {
    // Padding
    let length: u64 = (input.len() * 8) as u64;
    input.push(0x80);

    while input.len() % 64 != 56 {
        input.push(0);
    }
    input.append(&mut split_u64_to_byte_array(length).to_vec());

    let data: Vec<u32> = input
        .chunks(4)
        .map(|x| u32::from_ne_bytes(x[0..4].try_into().unwrap()))
        .collect();

    let mut hash_value_a: u32 = 0x6745_2301;
    let mut hash_value_b: u32 = 0xefcd_ab89;
    let mut hash_value_c: u32 = 0x98ba_dcfe;
    let mut hash_value_d: u32 = 0x1032_5476;

    for chunks in data.chunks(16) {
        let mut temp_value_a: u32 = hash_value_a;
        let mut temp_value_b: u32 = hash_value_b;
        let mut temp_value_c: u32 = hash_value_c;
        let mut temp_value_d: u32 = hash_value_d;

        for idx in 0..64 {
            let mut f: u32;
            let g: u32;

            if (0..16).contains(&idx) {
                f = (temp_value_b & temp_value_c) | ((!temp_value_b) & temp_value_d);
                g = idx;
            } else if (16..32).contains(&idx) {
                f = (temp_value_d & temp_value_b) | ((!temp_value_d) & temp_value_c);
                g = (5 * idx + 1) % 16;
            } else if (32..48).contains(&idx) {
                f = temp_value_b ^ temp_value_c ^ temp_value_d;
                g = (3 * idx + 5) % 16;
            } else if (48..64).contains(&idx) {
                f = temp_value_c ^ (temp_value_b | (!temp_value_d));
                g = (7 * idx) % 16;
            } else {
                unreachable!("UHhh oh");
            }

            f = f
                .wrapping_add(temp_value_a)
                .wrapping_add(PRE_COMP_TABLE_K[idx as usize])
                .wrapping_add(chunks[g as usize]);
            temp_value_a = temp_value_d;
            temp_value_d = temp_value_c;
            temp_value_c = temp_value_b;
            temp_value_b = temp_value_b.wrapping_add(f.rotate_left(PRE_COMP_TABLE_S[idx as usize]));
        }

        hash_value_a = hash_value_a.wrapping_add(temp_value_a);
        hash_value_b = hash_value_b.wrapping_add(temp_value_b);
        hash_value_c = hash_value_c.wrapping_add(temp_value_c);
        hash_value_d = hash_value_d.wrapping_add(temp_value_d);
    }

    let result: u128 = (u128::from(hash_value_a.swap_bytes()) << 96)
        + (u128::from(hash_value_b.swap_bytes()) << 64)
        + (u128::from(hash_value_c.swap_bytes()) << 32)
        + u128::from(hash_value_d.swap_bytes());

    result
}

#[allow(clippy::cast_possible_truncation)]
fn split_u64_to_byte_array(input: u64) -> [u8; 8] {
    [
        input as u8,
        (input >> 8) as u8,
        (input >> 16) as u8,
        (input >> 24) as u8,
        (input >> 32) as u8,
        (input >> 40) as u8,
        (input >> 48) as u8,
        (input >> 56) as u8,
    ]
}
