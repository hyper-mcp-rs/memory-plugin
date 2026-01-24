fn main() {
    // Ensure git information is available during build
    // This is a no-op file but helps document that git-version is used
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
}
