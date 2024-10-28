fn main() {
    let git_hash_main_branch = ".git/refs/heads/main";

    if let Ok(hash) = std::env::var("GITHUB_SHA") {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    } else if std::path::Path::new(git_hash_main_branch).exists() {
        println!(
            "cargo:rustc-env=GIT_HASH={}",
            include_str!(".git/refs/heads/main")
        );
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown");
    }
}
