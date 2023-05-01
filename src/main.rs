use std::process::Command;

fn main() {
    let ouput_path = "output";
    let cd_result = Command::new("cd")
        .arg(&ouput_path)
        .status()
        .expect("failed to execute cd command");

    if cd_result.success() {
        println!("Changed directory to output");
    } else {
        println!("Failed to change directory to output");
    }

    // let mut cmd = Command::new("pwd");
    let  go_mod_result = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg("output")
        .current_dir(&ouput_path)
        .status()
        .expect("failed to execute go mod init command");

    if go_mod_result.success() {
        println!("Initialized go module successfully");
    } else {
        println!("Failed to initialize go module");
    }
}
