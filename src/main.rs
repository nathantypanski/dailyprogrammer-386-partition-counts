enum PartitionState {
    AllIntegers,
    OddIntegers,
}

enum AddSub {
    Add,
    Subtract,
}

// _
// Add

// Add
// Add

// Add
// Subtract

// Subtract
// Subtract

// Subtract
// Add



struct AddSubCounter {
    last: AddSub,
    this: AddSub,
}

impl AddSubCounter {
    fn new() -> AddSubCounter {
        AddSubCounter {
            last: Subtract,
            this: Add,
        }
    }
}

impl Iterator for AddSubCounter {
    type Item = AddSub;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last, self.this) {
            (Add, Add) => {
                self.this = Subtract;
                Subtract
            }
            (Add, Subtract) => {
                self.last = Subtract;
                Subtract
            }
            (Subtract, Subtract) => {
                self.this = Add;
                Add
            }
            (Subtract, Add) => {
                self.last = Add;
                Add
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct AllIntegers {
    i: usize,
}

impl Iterator for AllIntegers {
    type Item = usize;

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
    i: usize,
}

impl Iterator for OddIntegers {
    type Item = usize;

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
    i: usize,
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
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        //print!("{:?}.next() -> ", self);
        self.i += 1;
        match self.state() {
            PartitionState::AllIntegers => {
                self.odd.next().and_then(|value| {
                    //println!("{}", value);
                    Some(value)
                })
            },
            PartitionState::OddIntegers => {
                self.all.next().and_then(|value| {
                    //println!("{}", value);
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
    i: usize,
    value: usize,
    dps: Box<DifferencePartitionSequence>,
}

impl Iterator for PartitionSequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            0 => {
                self.i = 1;
                Some(self.value)
            },
            _ => {
                self.i += 1;
                self.value += self.dps.next().expect("the sequence must go on");
                println!("{:?}", self);
                Some(self.value)
            },
        }
    }
}

impl PartitionSequence {
    fn new() -> PartitionSequence {
        let mut dps = DifferencePartitionSequence::new();
        PartitionSequence {
            i: 0,
            value: 1,
            dps: Box::new(dps),
        }
    }
}

fn main() {
    let mut ps = PartitionSequence::new();
    for _ in 0..15 {
        let result = ps.next().expect("The main sequence must go on!");
        println!("{:?} -> {}", ps, result);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn addsub() {
        let asc = super::AddSubCounter::new();
        let expect = [super::AddSub::Add, super::AddSub::Add, super::AddSub::Subtract, super::AddSub::Subtract, super::AddSub::Add, super::AddSub::Add, super::AddSub::Subtract, super::AddSub::Subtract, super::AddSub::Add, super::AddSub::Add];

        for (c, e) in expect.zip(all_int) {
            assert_eq!(c, e);
        }
    }

    #[test]
    fn all_integers() {
        let mut all_int = super::AllIntegers::new();
        let mut range = 1..100;

        for (r, a) in range.zip(all_int) {
            assert_eq!(r, a);
        }
    }

    #[test]
    fn odd_integers() {
        let mut odd_int = super::OddIntegers::new();
        let mut expect = (1..100).step_by(2);

        for (a, r) in expect.zip(odd_int) {
            assert_eq!(r, a);
        }
    }

    #[test]
    fn dps() {
        let mut dps = super::DifferencePartitionSequence::new();
        let expect = [1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7].iter();

        for (a, r) in expect.zip(dps) {
            //println!("got: {:?} expect: {:?}", r, a);
            assert_eq!(&r, a);
        }
    }

    #[test]
    fn ps() {
        let mut dps = super::PartitionSequence::new();
        let expect = [1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70].iter();

        for (a, r) in expect.zip(dps) {
            println!("got: {:?} expect: {:?}", r, a);
            //assert_eq!(&r, a);
        }
    }
}
