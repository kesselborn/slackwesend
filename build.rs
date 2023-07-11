fn main() {
    #[cfg(feature = "lambda")]
    {
        use std::process::Command;
        let command = "docker build --tag rust-lambda-artifact-builder --output type=local,dest=./lambda-artifacts .";
        println!("cargo:rerun-if-changed=src/");
        println!("cargo:rerun-if-changed=Dockerfile");
        println!("cargo:rerun-if-changed=lambda-artifacts");

        eprintln!("executing {command}");

        let _ = Command::new("sh")
            .arg("-xci")
            .arg(command)
            .spawn()
            .expect("error executing '{command}'");
    }
}
