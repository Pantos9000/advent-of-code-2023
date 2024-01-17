pub fn run(_input: &str) -> usize {
    0 // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....";

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 46);
    }
}
