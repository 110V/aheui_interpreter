use std::{fs, path::Path};

use aheui_interpreter::execute::executor::{self, Executor};

fn main() {

    let test = 
r#"뷷우희어밍우여
아아아아아아아반받망희
먕오뱞오뱗오뵬"#;
    let input = r#"밯3"#;
    let mut executor = executor::Executor::new(test,input,true);
    executor.execute();
    println!("{}",executor.get_output());
    //println!("Hello, world!");
}

#[test]
fn run_tests() {
    let test_files = fs::read_dir("./tests/snippets/pi").unwrap();

    for test_file in test_files {

        let test_file = test_file.unwrap().path();
        if !test_file.file_name().unwrap().to_str().unwrap().ends_with(".aheui") {
            continue;
        }
        let test_file_name = test_file.file_stem().unwrap().to_str().unwrap();
        let input_file = Path::new(&test_file).with_extension("in");
        let expected_output_file = Path::new(&test_file).with_extension("out");

        let test = fs::read_to_string(&test_file).unwrap();
        println!("{:?}",&test_file);
        let input = match fs::read_to_string(&input_file) {
            
            Ok(s) => s,
            Err(_) => "".to_string(),
        };

        let mut executor = Executor::new(&test, &input,false);
        executor.execute();

        let output = executor.get_output();
        let expected_output = fs::read_to_string(&expected_output_file).unwrap();

        assert_eq!(output, expected_output.trim(), "{} test failed", test_file_name);
    }
}
