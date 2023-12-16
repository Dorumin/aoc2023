use indexmap::IndexMap;

pub struct Sequence<'a> {
    pub steps: Vec<Step<'a>>
}

pub struct Step<'a> {
    s: &'a str
}

impl<'a> Sequence<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        let steps = input.split(',').map(|s| s.into()).collect();

        Some(Self {
            steps,
        })
    }

    pub fn fill_sluts(&self) -> [IndexMap<&str, u8>; 256] {
        let mut slots = [0; 256].map(|_| IndexMap::new());

        for step in self.steps.iter() {
            let slot = &mut slots[step.hash_label() as usize];

            if step.op() == Op::Add {
                slot.insert(step.label(), step.focal_length().unwrap());
            } else {
                slot.shift_remove(step.label());
            }
        }

        slots
    }
}

#[derive(PartialEq, Eq)]
pub enum Op {
    Remove,
    Add
}


fn mock_hash(s: &str) -> u8 {
    let mut v: u8 = 0;

    for byte in s.bytes() {
        v = v.wrapping_add(byte);
        v = v.wrapping_mul(17);
    }

    v
}

impl<'a> Step<'a> {
    pub fn label(&'a self) -> &'a str {
        self.s.split(['-', '=']).next().unwrap()
    }

    pub fn op(&'a self) -> Op {
        if self.s.ends_with('-') {
            Op::Remove
        } else {
            Op::Add
        }
    }

    pub fn focal_length(&'a self) -> Option<u8> {
        if matches!(self.op(), Op::Add) {
            Some(self.s.split_once('=').unwrap().1.parse().unwrap())
        } else {
            None
        }
    }

    pub fn hash_label(&self) -> u8 {
        mock_hash(self.label())
    }

    pub fn hash(&self) -> u8 {
        mock_hash(self.s)
    }
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            s: value
        }
    }
}
