//! <https://www.codewars.com/kata/59530d2401d6039f8600001f/train/rust>

pub const fn how_many_measurements(n: u64) -> u32 {
    if n <= 3u64.pow(20) {
        if n <= 3u64.pow(10) {
            if n <= 3u64.pow(5) {
                if n <= 3u64.pow(2) {
                    if n <= 3u64.pow(1) {
                        if n <= 3u64.pow(0) {
                            0
                        } else {
                            1
                        }
                    } else {
                        2
                    }
                } else if n <= 3u64.pow(3) {
                    3
                } else if n <= 3u64.pow(4) {
                    4
                } else {
                    5
                }
            } else if n <= 3u64.pow(7) {
                if n <= 3u64.pow(6) {
                    6
                } else {
                    7
                }
            } else if n <= 3u64.pow(8) {
                8
            } else if n <= 3u64.pow(9) {
                9
            } else {
                10
            }
        } else if n <= 3u64.pow(15) {
            if n <= 3u64.pow(13) {
                if n <= 3u64.pow(11) {
                    11
                } else if n <= 3u64.pow(12) {
                    12
                } else {
                    13
                }
            } else if n <= 3u64.pow(14) {
                14
            } else {
                15
            }
        } else if n <= 3u64.pow(18) {
            if n <= 3u64.pow(16) {
                16
            } else if n <= 3u64.pow(17) {
                17
            } else {
                18
            }
        } else if n <= 3u64.pow(19) {
            19
        } else {
            20
        }
    } else if n <= 3u64.pow(30) {
        if n <= 3u64.pow(25) {
            if n <= 3u64.pow(23) {
                if n <= 3u64.pow(21) {
                    21
                } else if n <= 3u64.pow(22) {
                    22
                } else {
                    23
                }
            } else if n <= 3u64.pow(24) {
                24
            } else {
                25
            }
        } else if n <= 3u64.pow(28) {
            if n <= 3u64.pow(26) {
                26
            } else if n <= 3u64.pow(27) {
                27
            } else {
                28
            }
        } else if n <= 3u64.pow(29) {
            29
        } else {
            30
        }
    } else if n <= 3u64.pow(35) {
        if n <= 3u64.pow(33) {
            if n <= 3u64.pow(31) {
                31
            } else if n <= 3u64.pow(32) {
                32
            } else {
                33
            }
        } else if n <= 3u64.pow(34) {
            34
        } else {
            35
        }
    } else if n <= 3u64.pow(38) {
        if n <= 3u64.pow(36) {
            36
        } else if n <= 3u64.pow(37) {
            37
        } else {
            38
        }
    } else if n <= 3u64.pow(40) {
        if n <= 3u64.pow(39) {
            39
        } else {
            40
        }
    } else {
        41
    }
}
