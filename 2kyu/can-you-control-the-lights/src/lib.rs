//! <https://www.codewars.com/kata/633cb47c1ec6ac0064ec242d/train/rust>

use core::ops::BitXorAssign;

pub struct LightController {
    switches: Vec<(BitSet, BitSet)>,
    first: Vec<usize>,
}

impl LightController {
    pub fn new(n: usize, corresponding_lights_list: &[Vec<usize>]) -> Self {
        assert!(n <= BitSet::LEN && corresponding_lights_list.len() <= BitSet::LEN);

        let mut switches: Vec<_> = corresponding_lights_list
            .iter()
            .enumerate()
            .map(|(ind, value)| {
                let mut ind_set = BitSet::default();
                ind_set.set(ind);
                (ind_set, BitSet::new(value))
            })
            .collect();

        let mut row = 0;

        for col in 0..n {
            if let Some((i, _)) = switches
                .iter()
                .enumerate()
                .skip(row)
                .find(|(_, (_, bs))| bs.get(col))
            {
                switches.swap(row, i);

                let (le, ri) = switches.split_at_mut(row + 1);

                for (ind, value) in ri {
                    if value.get(col) {
                        *value ^= &le.last().unwrap().1;
                        *ind ^= &le.last().unwrap().0;
                    }
                }

                row += 1;
            }
        }

        switches.truncate(row);
        let first = switches
            .iter()
            .map(|(_, row)| row.find_first().unwrap())
            .collect();

        Self { switches, first }
    }

    pub fn solve(&self, lights: &[usize]) -> Option<Vec<usize>> {
        if lights.is_empty() {
            return Some(vec![]);
        }

        let mut lights = BitSet::new(lights);
        let mut target = lights.find_first().unwrap();
        let mut res = BitSet::default();

        for i in 0..self.switches.len() {
            if *unsafe { self.first.get_unchecked(i) } == target {
                lights ^= &self.switches[i].1;
                res ^= &self.switches[i].0;

                if let Some(new_target) = lights.find_first() {
                    target = new_target;
                } else {
                    return Some(res.one_positions().collect());
                }
            }
        }

        None
    }
}

#[derive(Default, Clone, Debug)]
struct BitSet {
    mem: [u64; Self::MEM_LEN],
}

impl BitSet {
    const MEM_LEN: usize = 8;
    pub const LEN: usize = Self::MEM_LEN * u64::BITS as usize;

    pub fn new(incides: &[usize]) -> Self {
        let mut res = Self::default();
        for &i in incides {
            res.set(i);
        }
        res
    }

    pub const fn get(&self, index: usize) -> bool {
        (self.mem[index / 64] & (1 << (index % 64))) != 0
    }

    pub fn set(&mut self, index: usize) {
        self.mem[index / 64] |= 1 << (index % 64);
    }

    pub fn find_first(&self) -> Option<usize> {
        self.mem.iter().enumerate().find_map(|value| match value {
            (_, 0) => None,
            (ind, x) => Some(64 * ind + x.trailing_zeros() as usize),
        })
    }

    pub fn find_next(&self, mut index: usize) -> Option<usize> {
        if index >= Self::LEN {
            return None;
        }

        if index % 64 != 0 {
            let head = self.mem[index / 64] & !((1 << (index % 64)) - 1);
            if head != 0 {
                return Some((index & !63) + head.trailing_zeros() as usize);
            }
            index = (index & !63) + 64;
        }

        self.mem
            .iter()
            .enumerate()
            .skip(index / 64)
            .find_map(|value| match value {
                (_, 0) => None,
                (ind, x) => Some(ind * 64 + x.trailing_zeros() as usize),
            })
    }

    pub const fn one_positions(&self) -> BitsetOnePositions {
        BitsetOnePositions {
            bitset: self,
            index: 0,
        }
    }
}

impl BitXorAssign<&Self> for BitSet {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for (x, y) in self.mem.iter_mut().zip(rhs.mem) {
            *x ^= y;
        }
    }
}

pub struct BitsetOnePositions<'a> {
    bitset: &'a BitSet,
    index: usize,
}

impl Iterator for BitsetOnePositions<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.bitset.find_next(self.index);
        self.index = next.map_or(BitSet::LEN, |x| x + 1);
        next
    }
}
