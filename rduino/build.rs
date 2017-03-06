extern crate carguino_build;

use carguino_build::Config;

pub fn main() {
    let config = Config::new().unwrap();
    let core = config.core();
    if core != "arduino" && !core.ends_with(":arduino") {
        println!("cargo:warning=core '{}' might not be supported", core);
    }

    config.builder()
          .core_sources()
          .build("core")
          .unwrap();

    config.builder()
          .source("src/ffi/rduino.cpp")
          .build("rduino")
          .unwrap();

    config.bindgen()
          .options(|options| {
              options.ctypes_prefix("::platform::raw")
                     .whitelisted_type("Rduino.+")
                     .whitelisted_function("rduino_.+")
                     .whitelisted_var("RDUINO_.+")
          })
          .generate("src/ffi/rduino.hpp")
          .unwrap();
}
