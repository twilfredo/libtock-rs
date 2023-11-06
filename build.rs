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
    println!("cargo:rustc-link-arg=-lspdm_transport_pcidoe_lib");

    println!("cargo:rustc-link-arg=/scratch/alistair/software/tock/libtock-c/newlib/rv32/rv32imac/libc.a");
    println!("cargo:rustc-link-arg=/scratch/alistair/software/tock/libtock-c/newlib/rv32/rv32imac/libm.a");
    println!("cargo:rustc-link-arg=/scratch/alistair/software/tock/libtock-c/libtock/build/rv32imac/libtock.a");

    println!("cargo:rustc-link-search=/scratch/alistair/software/tier4/SPDM-Utils/third-party/libspdm/build/lib/");
}
