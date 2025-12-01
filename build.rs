fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        commitment_issues::find_valid_git_root!()
    );
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}
