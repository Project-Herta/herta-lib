pub mod math;

fn calculate_level_multiplier(level: u8) -> f32 {
    match level {
        1 => 54.0000,
        2 => 58.0000,
        3 => 62.0000,
        4 => 67.5264,
        5 => 70.5094,
        6 => 73.5228,
        7 => 76.5660,
        8 => 79.6385,
        9 => 82.7395,
        10 => 85.8684,
        11 => 91.4944,
        12 => 97.0680,
        13 => 102.5892,
        14 => 108.0579,
        15 => 113.4743,
        16 => 118.8383,
        17 => 124.1499,
        18 => 129.4091,
        19 => 134.6159,
        20 => 139.7703,
        21 => 149.3323,
        22 => 158.8011,
        23 => 168.1768,
        24 => 177.4594,
        25 => 186.6489,
        26 => 195.7452,
        27 => 204.7484,
        28 => 213.6585,
        29 => 222.4754,
        30 => 231.1992,
        31 => 246.4276,
        32 => 261.1810,
        33 => 275.4733,
        34 => 289.3179,
        35 => 302.7275,
        36 => 315.7144,
        37 => 328.2905,
        38 => 340.4671,
        39 => 352.2554,
        40 => 363.6658,
        41 => 408.1240,
        42 => 451.7883,
        43 => 494.6798,
        44 => 536.8188,
        45 => 578.2249,
        46 => 618.9172,
        47 => 658.9138,
        48 => 698.2325,
        49 => 736.8905,
        50 => 774.9041,
        51 => 871.0599,
        52 => 964.8705,
        53 => 1056.4206,
        54 => 1145.7910,
        55 => 1233.0585,
        56 => 1318.2965,
        57 => 1401.5750,
        58 => 1482.9608,
        59 => 1562.5178,
        60 => 1640.3068,
        61 => 1752.3215,
        62 => 1861.9011,
        63 => 1969.1242,
        64 => 2074.0659,
        65 => 2176.7983,
        66 => 2277.3904,
        67 => 2375.9085,
        68 => 2472.4160,
        69 => 2566.9739,
        70 => 2659.6406,
        71 => 2780.3044,
        72 => 2898.6022,
        73 => 3014.6029,
        74 => 3128.3729,
        75 => 3239.9758,
        76 => 3349.4730,
        77 => 3456.9236,
        78 => 3562.3843,
        79 => 3665.9099,
        80 | _ => 3767.5533,
    }
}
