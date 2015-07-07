fn main() {
    println!("cargo:rustc-link-lib=dylib=zookeeper_mt");
    println!("cargo:rustc-link-search=native=/usr/local/opt/zookeeper/lib");
}
