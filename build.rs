#[cfg(not(feature = "nolibsnark"))]
extern crate gcc;

fn main() {
    #[cfg(not(feature = "nolibsnark"))]
    {
        println!("cargo:rustc-link-search=/usr/local/lib");
        gcc::Config::new()
            .cpp(true)
            .include("/usr/local/include")
            .include("/usr/local/include/libsnark")
            .opt_level(2)
            .define("NO_PROCPS", None)
            .define("STATIC", None)
            .define("MONTGOMERY_OUTPUT", None)
            .define("USE_ASM", None)
            .define("NO_PT_COMPRESSION", None)
            .define("BINARY_OUTPUT", None)
            .define("CURVE_ALT_BN128", None)
            .flag("-std=c++11")
            .file("lib/wraplibsnark.cpp")
            .compile("libwraplibsnark.a");
    }
}
