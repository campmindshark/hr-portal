use std::process::Command;

fn main() {
    read_compiler_info().ok();
    read_git_info().ok();
}

fn read_compiler_info() -> Result<(), Box<dyn std::error::Error>> {
    let profile = std::env::var("PROFILE").unwrap_or("unknown".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

    let compiler = std::env::var("RUSTC")?;
    let rust_version = run_command(&[&compiler, "-V"])?;
    println!("cargo:rustc-env=RUSTC_VERSION={}", rust_version);

    let target = std::env::var("TARGET")?;
    println!("cargo:rustc-env=RUST_TARGET={}", target);

    Ok(())
}

fn read_git_info() -> Result<(), Box<dyn std::error::Error>> {
    // Record what branch was being worked on when the build was made
    let current_branch = run_command(&["git", "rev-parse", "--abbrev-ref", "HEAD"])?;
    println!("cargo:rustc-env=GIT_BRANCH={}", current_branch);

    let describe = run_command(&["git", "describe", "--always", "--tags"])?;
    println!("cargo:rustc-env=GIT_DESCRIBE={}", describe);

    let is_dirty = run_command(&["git", "diff", "--exit-code", "--quiet"]).is_err();
    println!("cargo:rustc-env=GIT_DIRTY={}", is_dirty);

    let latest_revision = run_command(&["git", "rev-parse", "HEAD"])?;
    println!("cargo:rustc-env=GIT_REVISION={}", latest_revision);

    // Get the current tagged version. If the current commit isn't tagged this will give a format
    // indicating what the most recent tag was, how many commits suceeded it, and what the current
    // git short SHA is for the branch. Does not indicate whether or not the current current
    // repository is dirty or not (has uncommitted changes). If no tags have yet been made this
    // will produce an error (and in this case we want an empty string, so we'll just not set the
    // value but continue sucessfully).
    let commit_tag = run_command(&["git", "describe", "--tags"]).ok();
    if let Some(ref ct) = commit_tag {
        println!("cargo:rustc-env=GIT_TAG={}", ct);
    };

    Ok(())
}

fn run_command(args: &[&str]) -> Result<String, std::io::Error> {
    let result = Command::new(args[0]).args(&args[1..]).output()?;

    if !result.status.success() {
        use std::io::{Error, ErrorKind};

        if let Ok(dump_output) = std::env::var("BUILD_DIAGNOSTICS") {
            if dump_output == "true" {
                println!("--------------------------------------------------");
                println!("Dumping failed output for command: {}", args.join(" "));
                println!("--------------------------------------------------");
                println!("STDOUT:");
                println!("{}", String::from_utf8(result.stdout).unwrap().trim());
                println!("--------------------------------------------------");
                println!("STDERR:");
                println!("{}", String::from_utf8(result.stderr).unwrap().trim());
            }
        };

        return Err(Error::new(ErrorKind::Other, "Command failed to complete"));
    }

    Ok(String::from_utf8(result.stdout).unwrap().trim().to_string())
}
