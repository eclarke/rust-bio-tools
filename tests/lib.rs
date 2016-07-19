use std::process::Command;
use std::fs;


fn test_output(result: &str, expected: &str) {
    assert!(Command::new("cmp")
            .arg(result)
            .arg(expected)
            .spawn().unwrap().wait().unwrap().success());
    fs::remove_file(result).unwrap();
}


#[test]
fn fastq_split() {
    assert!(Command::new("bash")
            .arg("-c")
            .arg("target/debug/rbt fastq-split tests/A.fastq tests/B.fastq < tests/test.fastq")
            .spawn().unwrap().wait().unwrap().success());
    test_output("tests/A.fastq", "tests/expected/A.fastq");
    test_output("tests/B.fastq", "tests/expected/B.fastq");
}

#[test]
fn bam_depth() {
    assert!(Command::new("bash")
            .arg("-c")
            .arg("target/debug/rbt bam-depth tests/test.bam < tests/pos.txt > tests/depth.txt")
            .spawn().unwrap().wait().unwrap().success());
    test_output("tests/depth.txt", "tests/expected/depth.txt");
}
