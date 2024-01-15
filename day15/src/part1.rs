pub fn run(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(s: &str) -> usize {
    let mut hasher = Hasher::default();
    for c in s.chars() {
        hasher.advance(c);
    }
    hasher.into()
}

#[derive(Default)]
struct Hasher {
    current_value: u32,
}

impl Hasher {
    fn advance(&mut self, c: char) {
        if c == '\n' {
            return;
        }

        let code = u32::from(c);
        self.current_value += code;
        self.current_value *= 17;
        self.current_value %= 256;
    }
}

impl From<Hasher> for usize {
    fn from(value: Hasher) -> Self {
        value.current_value.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher_ignores_newline() {
        let mut hasher = Hasher::default();
        hasher.advance('\n');
        assert_eq!(usize::from(hasher), 0);
    }

    #[test]
    fn test_hash_func() {
        let s = "HASH";
        assert_eq!(hash(s), 52);
    }

    #[test]
    fn test_run() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run(input), 1320);
    }
}
