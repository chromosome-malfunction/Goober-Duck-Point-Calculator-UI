fn main() {
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rustc-link-arg=app.res");
}
