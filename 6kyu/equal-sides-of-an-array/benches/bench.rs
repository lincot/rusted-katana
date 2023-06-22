#![no_std]
#![feature(test)]

extern crate test;
use equal_sides_of_an_array::find_even_index;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        find_even_index(black_box(&[
            -2944, -8753, -4244, 3692, 3066, 6112, 5863, -9132, -3487, -9336, 5681, 918, -7659,
            410, 9577, -6443, 1418, -7443, 8638, 418, -3096, 2481, 8515, -460, -2509, 6849, 9127,
            -2006, -5717, -9884, 1369, 1940, -5664, 8742, 9643, 4620, 3091, -920, 1299, 6703,
            -2131, 2337, -9798, 6598, -5242, -6543, -7790, 4994, 5634, -272, -2423, -9323, -3485,
            -4328, -6043, 2095, -179, 5742, 3519, -4884, -2566, -3785, 109, 5526, -6827, -7266,
            7421, -826, -9395, 632, 4694, -7529, 3815, 5416, -6478, 5461, 9514, -1542, -8650,
            -9948, 8924, 9821, -3498, -4358, 5552, 1052, 8528, -1953, -5342, 5554, -6576, -5035,
            3193, -7968, -9645, 1736, -1734, -8110, 3593, -9562, 5842, 8269, 6167, 5040, -2551,
            4412, -8139, 2678, 6415, 7955, 3573, 7786, 3670, 1079, -317, 2753, 8185, 2656, -1078,
            722, -1362, -4655, -3810, 5887, 342, 4403, 2477, -9922, -9539, -1024, 1345, -8372,
            -391, -7294, -9667, 9840, 2342, 6390, 5256, 3400, -382, 2431, 2684, 9129, 5437, -8206,
            -1719, -2994, 3322, -7272, -7836, 3162, -4490, 5764, 4913, -3696, 5550, -3341, 0, 8225,
            2028, -8357, 5077, -3702, 434, -7581, 500, 6251, 6939, -8140, 1251, -3255, 4592, 642,
            -3134, -7736, 7827, 9847, 6843, 8964, -8561, 4221, 9779, 9436, 3511, 5741, -7654,
            -8506, -2839, -7978, 4977, 7407, 8017, 1122, 4926, 6713, 8719, 6540, -9681, -7871,
            -8851, -9601, 87, -101, 6212, 440, 4960, 1672, -2193, 8133, -2324, -1336, 940, -9199,
            1858, -3011, 273, -1392, 2796, 6736, 2696, -5129, -8994, -9529, -4355, 3435, 1065,
            -6217, -2237, -1639, -9546, 2776, 2927, -6769, 4742, -289, -124, -2847, 7022, -810,
            7478, 5290, -3075, 5255, 4888, -9448, 6511, -782, 9722, -5170, 851, -4236, 4828, 4286,
            -823, -120, -8275, -7560, 2635, -8661, 5562, -2986, -3825, -4048, -8469, -1548, -7022,
            -1738, 7286, 8516, -7716, 6645, 5942, 4756, -6990, 4685, 9814, 1211, 1438, -4989,
            -1145, 868, 9508, 8458, -8976, -9320, -4422, 1496, 1444, 5377, 2679, -1209, -5828,
            -6930, -9356, 3856, -7123, 8822, 8135, 9627, -566, 5178, 4875, 9908, 6527, -8441, 8278,
            -4063, 7602, -1724, -9733, -4085, -2142, 1042, -5047, -6874, -6012, -81, -5600, 6091,
            1172, 1454, -1318, 841, 2175, -9307, -7254, 5903, 9023, 111, 2607, -590, -5368, 7236,
            -481, 9628, -5055, -2416, 5116, 6837, -9594, -7135, -9406, 5966, -2662, 3540, -1896,
            -9862, 3302, 1498, 6710, 9724, -3470, -5506, -9889, -1625, -9315, 8481, -7385, -6477,
            -7765, -3859, 5453, -5104, 415, -1599, 3109, -884, 2991, 9681, 1318, 1680, 8072, 9584,
            -2693, 3693, 9142, -3105, -8267, -1217, 1678, 9938, -8003, 2159, 6826, -6367, 8545,
            1280, -8095, -932, 2348, 7127, 810, -6866, 6183, 9285, 2810, -6527, 3170, -610, -7757,
            2705, -7559, 7096, -1234, 1867, -5738, 6755, -6526, 4676, 6897, 6720, 6026, 2786, 6749,
            7007, -9290, 2351, -3307, -3346, -4373, -3455, -4687, -9527, 1447, -6868, -8440, -589,
            6565, -1424, -417, 8282, -3858, -9056, 5609, -9213, -6425, 7346, -5445, 9180, -9203,
            5380, -3687, -7253, -1558, -7986, 6135, 7875, 9045, 569, -6910, 8456, 2020, -4726,
            7758, -5187, -739, 4432, -5036, 7435,
        ]))
    });
}