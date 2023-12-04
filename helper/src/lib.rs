pub mod util;
pub fn easy_input(input_name: &str) -> String {
    use std::fs::read_to_string;
    use std::path::Path;
    //input file should be the same name as crate for aoc
    //so crate "day1" should have input file name be "day1"
    let input_name = "/input/".to_owned() + input_name;

    let input_path = Path::new(".")
        .canonicalize()
        .unwrap()
        .ancestors()
        .find(|path| path.to_str().unwrap().ends_with("aoc2023"))
        .unwrap()
        .to_string_lossy()
        .to_string()
        + &input_name;

    read_to_string(input_path)
        .expect("Could not read input file")
        .trim()
        .to_owned()
}
pub fn easy_aoc_input() -> String {
    let run_arg = std::env::args().next().unwrap();
    let day = run_arg.split('/').last().unwrap();
    easy_input(day)
}

#[cfg(test)]
mod input {
    #[test]
    fn input_test() {
        let result = crate::easy_input("test.txt");
        assert_eq!(result, "foo and bar");
    }
}
