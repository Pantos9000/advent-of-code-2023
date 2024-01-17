use crate::part1::hash;

use linked_hash_map::LinkedHashMap;

pub fn run(input: &str) -> usize {
    let mut boxes = vec![LensBox::default(); 256];
    input
        .split(',')
        .map(Instruction::parse)
        .for_each(|instruction| instruction.execute(&mut boxes));
    boxes
        .into_iter()
        .enumerate()
        .map(|(box_id, lens_box)| lens_box.focusing_power(box_id))
        .sum()
}

#[derive(Debug, Default, Clone)]
struct LensBox<'a> {
    lenses: LinkedHashMap<&'a str, Lens>,
}

impl<'a> LensBox<'a> {
    fn remove_lens(&mut self, label: &str) {
        self.lenses.remove(label);
    }

    fn add_lens(&mut self, label: &'a str, focal_length: usize) {
        let lens = Lens::new(focal_length);
        if let Some(existing_lens) = self.lenses.get_mut(label) {
            existing_lens.replace(lens);
            return;
        }
        let existing = self.lenses.insert(label, lens);
        assert!(existing.is_none());
    }

    fn focusing_power(self, box_id: usize) -> usize {
        let box_mult = box_id + 1;
        let lens_mult: usize = self
            .lenses
            .into_iter()
            .enumerate()
            .map(|(i, (_label, lens))| lens.focal_length * (i + 1))
            .sum();
        lens_mult * box_mult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lens {
    focal_length: usize,
}
impl Lens {
    fn new(focal_length: usize) -> Self {
        Self { focal_length }
    }
    fn replace(&mut self, other: Self) {
        self.focal_length = other.focal_length;
    }
}

struct Instruction<'a> {
    box_id: usize,
    label: &'a str,
    operation: Operation,
}

impl<'a> Instruction<'a> {
    fn parse(s: &'a str) -> Self {
        let s = Self::remove_newline(s);
        let operation = Operation::parse(s);
        let label = Self::parse_label(s, operation);
        let box_id = hash(label);
        Self {
            box_id,
            label,
            operation,
        }
    }

    fn remove_newline(s: &str) -> &str {
        let Some(last_char) = s.chars().last() else {
            return s;
        };
        if last_char != '\n' {
            return s;
        }
        let len_without_newline = s.len() - 1;
        &s[..len_without_newline]
    }

    fn parse_label(s: &str, operation: Operation) -> &str {
        let label_len = match operation {
            Operation::RemoveLens => s.len() - 1,
            Operation::NewLens { .. } => s.len() - 2,
        };
        &s[..label_len]
    }

    fn execute(self, boxes: &mut [LensBox<'a>]) {
        let target_box = &mut boxes[self.box_id];
        match self.operation {
            Operation::RemoveLens => target_box.remove_lens(self.label),
            Operation::NewLens { focal_length } => target_box.add_lens(self.label, focal_length),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    RemoveLens,
    NewLens { focal_length: usize },
}

impl Operation {
    fn parse(s: &str) -> Self {
        let last_char = s.chars().last().expect("parse failed - str is empty");
        match last_char {
            '-' => Self::RemoveLens,
            x @ '1'..='9' => {
                let focal_length = x.to_digit(10).unwrap().try_into().unwrap();
                Self::NewLens { focal_length }
            }
            _ => panic!("parse failed - last char is {last_char}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show_lenses(boxes: &[LensBox]) -> String {
        let mut ret = String::new();
        let mut is_first = true;
        'boxloop: for (box_id, lens_box) in boxes.iter().enumerate() {
            if lens_box.lenses.is_empty() {
                continue 'boxloop;
            }
            if is_first {
                is_first = false;
            } else {
                ret.push('\n');
            }
            ret.push_str(&format!("Box {box_id}:"));
            for (label, lens) in &lens_box.lenses {
                let focal_length = lens.focal_length;
                ret.push_str(&format!(" [{label} {focal_length}]"));
            }
        }
        ret
    }

    #[test]
    fn test_example() {
        let mut boxes = vec![LensBox::default(); 256];

        Instruction::parse("rn=1").execute(&mut boxes);
        let expectation = "Box 0: [rn 1]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("cm-").execute(&mut boxes);
        let expectation = "Box 0: [rn 1]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("qp=3").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1]\n\
            Box 1: [qp 3]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("cm=2").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 1: [qp 3]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("qp-").execute(&mut boxes);
        let expectation = "Box 0: [rn 1] [cm 2]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("pc=4").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [pc 4]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("ot=9").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [pc 4] [ot 9]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("ab=5").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [pc 4] [ot 9] [ab 5]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("pc-").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [ot 9] [ab 5]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("pc=6").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [ot 9] [ab 5] [pc 6]";
        assert_eq!(show_lenses(&boxes), expectation);

        Instruction::parse("ot=7").execute(&mut boxes);
        let expectation = "\
            Box 0: [rn 1] [cm 2]\n\
            Box 3: [ot 7] [ab 5] [pc 6]";
        assert_eq!(show_lenses(&boxes), expectation);
    }
}
