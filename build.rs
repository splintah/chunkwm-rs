extern crate cc;

fn main() {
    #[cfg(feature = "border")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-objc")
            .flag("-framework")
            .flag("Cocoa")
            .include("./chunkwm-lib/border/")
            .file("./chunkwm-lib/border/border.mm")
            .compile("border");
    }
}