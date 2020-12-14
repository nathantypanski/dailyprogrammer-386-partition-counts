use std::env;
use std::collections::HashMap;
use num_bigint::{BigInt,ToBigInt};

#[derive(Copy, Debug, Clone)]
enum PartitionState {
    AllIntegers,
    OddIntegers,
}

#[derive(Copy, Debug, Clone, PartialEq)]
enum AddSub {
    Add,
    Sub,
}

impl Eq for AddSub {}

#[derive(Copy, Debug, Clone, PartialEq)]
struct AddSubCounter {
    last: AddSub,
    this: AddSub,
}

impl Eq for AddSubCounter {}

impl AddSubCounter {
    fn new() -> AddSubCounter {
        AddSubCounter {
            last: AddSub::Sub,
            this: AddSub::Sub,
        }
    }
}

impl Iterator for AddSubCounter {
    type Item = AddSub;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last, self.this) {
            (AddSub::Add, AddSub::Add) => {
                self.this = AddSub::Sub;
                Some(AddSub::Sub)
            }
            (AddSub::Add, AddSub::Sub) => {
                self.last = AddSub::Sub;
                Some(AddSub::Sub)
            }
            (AddSub::Sub, AddSub::Sub) => {
                self.this = AddSub::Add;
                Some(AddSub::Add)
            }
            (AddSub::Sub, AddSub::Add) => {
                self.last = AddSub::Add;
                Some(AddSub::Add)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct AllIntegers {
    i: i64,
}

impl Iterator for AllIntegers {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        Some(self.i)
    }
}

impl AllIntegers {
    fn new() -> AllIntegers {
        AllIntegers { i: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
struct OddIntegers {
    i: i64,
}

impl Iterator for OddIntegers {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            1 => {
                self.i += 1;
                Some(1)
            },
            2 => {
                self.i += 1;
                Some(self.i)
            },
            _ => {
                self.i += 2;
                Some(self.i)
            }
        }
    }
}

impl OddIntegers {
    fn new() -> OddIntegers {
        OddIntegers { i: 1 }
    }
}

#[derive(Debug, Clone)]
struct DifferencePartitionSequence {
    i: i64,
    all: Box<AllIntegers>,
    odd: Box<OddIntegers>,
}

impl DifferencePartitionSequence {
    fn state(&self) -> PartitionState {
        if self.i % 2 == 0 {
            PartitionState::AllIntegers
        } else {
            PartitionState::OddIntegers
        }
    }
}

impl Iterator for DifferencePartitionSequence {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        match self.state() {
            PartitionState::AllIntegers => {
                self.odd.next().and_then(|value| {
                    Some(value)
                })
            },
            PartitionState::OddIntegers => {
                self.all.next().and_then(|value| {
                    Some(value)
                })
            },
        }
    }
}

impl DifferencePartitionSequence {
    fn new() -> DifferencePartitionSequence {
        let i = 2;
        let all = Box::new(AllIntegers::new());
        let mut odd = Box::new(OddIntegers::new());
        &odd.next().expect("Should always be next");
        DifferencePartitionSequence {
            i,
            all,
            odd,
        }
    }
}

#[derive(Debug, Clone)]
struct PartitionSequence {
    i: i64,
    value: i64,
    dps: Box<DifferencePartitionSequence>,
}

impl Iterator for PartitionSequence {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            0 => {
                self.i = 1;
                Some(self.value)
            },
            _ => {
                self.i += 1;
                self.value += self.dps.next().expect("the sequence must go on");
                Some(self.value)
            },
        }
    }
}

impl PartitionSequence {
    fn new() -> PartitionSequence {
        let dps = DifferencePartitionSequence::new();
        PartitionSequence {
            i: 0,
            value: 1,
            dps: Box::new(dps),
        }
    }
}

fn calculate_p(cache: &mut HashMap<i64, BigInt>, n: i64) -> BigInt {
    match cache.get(&n) {
        Some(result) => {
            result.clone()
        },
        None => {
            if n == 0 {
                return 1.to_bigint().unwrap();
            }
            let mut pos_neg = AddSubCounter::new();
            let mut ps = PartitionSequence::new();
            let mut calls = Vec::new();

            let mut s = ps.next().expect("must always go on");
            while (n - s) >= 0 {
                calls.push((pos_neg.next(), s));
                s = ps.next().expect("must always go on");
            }

            let result = calls.iter().fold(ToBigInt::to_bigint(&0).expect(""), |acc, (pn, s)| {
                match pn {
                    Some(AddSub::Add) => {
                        acc + calculate_p(cache, n - s)
                    },
                    Some(AddSub::Sub) => {
                        acc - calculate_p(cache, n - s)
                    },
                    _ => acc,
                }
            });

            cache.insert(n, result.clone());
            result
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[args.len() - 1].parse::<i64>().unwrap();
    let mut h = HashMap::new();
    println!("{}", calculate_p(&mut h, n));
}

#[cfg(test)]
mod tests {
    #[test]
    fn addsub() {
        let mut asc = super::AddSubCounter::new();
        let expect = Box::new([super::AddSub::Add, super::AddSub::Add, super::AddSub::Sub, super::AddSub::Sub, super::AddSub::Add, super::AddSub::Add, super::AddSub::Sub, super::AddSub::Sub, super::AddSub::Add, super::AddSub::Add].iter());

        for e in expect {
            let got = asc.next().expect("must be able to count");
            assert_eq!(got, *e);
        }
    }

    #[test]
    fn all_integers() {
        let all_int = super::AllIntegers::new();
        let range = 1..100;

        for (r, a) in range.zip(all_int) {
            assert_eq!(r, a);
        }
    }

    #[test]
    fn odd_integers() {
        let odd_int = super::OddIntegers::new();
        let expect = (1..100).step_by(2);

        for (a, r) in expect.zip(odd_int) {
            assert_eq!(r, a);
        }
    }

    #[test]
    fn dps() {
        let dps = super::DifferencePartitionSequence::new();
        let expect = [1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7].iter();

        for (a, r) in expect.zip(dps) {
            assert_eq!(&r, a);
        }
    }

    #[test]
    fn ps() {
        let dps = super::PartitionSequence::new();
        let expect = [1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70].iter();

        for (a, r) in expect.zip(dps) {
            println!("got: {:?} expect: {:?}", r, a);
        }
    }
}
