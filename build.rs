fn main() {
    libtock_build_scripts::auto_layout();

    println!("cargo:rustc-link-arg=-lmemlib");
    println!("cargo:rustc-link-arg=-lmalloclib");
    println!("cargo:rustc-link-arg=-ldebuglib");
    println!("cargo:rustc-link-arg=-lplatform_lib");
    println!("cargo:rustc-link-arg=-lcryptlib_mbedtls");
    println!("cargo:rustc-link-arg=-lrnglib");

    println!("cargo:rustc-link-arg=-lmbedtls");
    println!("cargo:rustc-link-arg=-lmbedx509");
    println!("cargo:rustc-link-arg=-lmbedcrypto");

    println!("cargo:rustc-link-arg=-lspdm_common_lib");
    println!("cargo:rustc-link-arg=-lspdm_requester_lib");
    println!("cargo:rustc-link-arg=-lspdm_responder_lib");
    println!("cargo:rustc-link-arg=-lspdm_secured_message_lib");
    println!("cargo:rustc-link-arg=-lspdm_secured_message_lib");
    println!("cargo:rustc-link-arg=-lspdm_crypt_lib");
    println!("cargo:rustc-link-arg=-lspdm_crypt_ext_lib");

    println!("cargo:rustc-link-arg=/home/twilfred/wdc/libtock-c/newlib/cortex-m/v7e-m/libc.a");
    println!("cargo:rustc-link-arg=/home/twilfred/wdc/libtock-c/newlib/cortex-m/v7e-m/libm.a");
    println!("cargo:rustc-link-arg=/home/twilfred/wdc/libtock-c/libtock/build/cortex-m4/libtock.a");

    println!(
        "cargo:rustc-link-search=/home/twilfred/wdc/spdm-utils/third-party/libspdm/build/lib/"
    );
}
