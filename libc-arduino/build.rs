extern crate carguino_build;

use carguino_build::Config;

pub fn main() {
    let config = Config::new().unwrap();
    config.bindgen()
          .options(|options| {
              options.ctypes_prefix("")
                     .generate_comments(false)
          })
          .generate("src/libc.h")
          .unwrap();
}
