//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
  
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 设置 TEST_FOO 环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 设置 pass feature flag
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
