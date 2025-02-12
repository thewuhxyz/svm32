use lazy_static::lazy_static;

mod solana_sbf_rust_128bit;
mod solana_sbf_rust_alloc;
mod solana_sbf_rust_alt_bn128;
mod solana_sbf_rust_alt_bn128_compression;
mod solana_sbf_rust_curve25519;
mod solana_sbf_rust_custom_heap;
mod solana_sbf_rust_iter;
mod solana_sbf_rust_many_args;
mod solana_sbf_rust_mem;
mod solana_sbf_rust_membuiltins;
mod solana_sbf_rust_noop;
mod solana_sbf_rust_param_passing;
mod solana_sbf_rust_poseidon;
mod solana_sbf_rust_rand;
mod solana_sbf_rust_remaining_compute_units;
mod solana_sbf_rust_sanity;
mod solana_sbf_rust_secp256k1_recover;
mod solana_sbf_rust_sha;

lazy_static! {
    static ref PROGRAM_128BIT: &'static [u8] = &solana_sbf_rust_128bit::PROGRAM_BYTES;
    static ref PROGRAM_ALLOC: &'static [u8] = &solana_sbf_rust_alloc::PROGRAM_BYTES;
    static ref PROGRAM_ALT_BN128: &'static [u8] = &solana_sbf_rust_alt_bn128::PROGRAM_BYTES;
    static ref PROGRAM_ALT_BN128_COMPRESSION: &'static [u8] =
        &solana_sbf_rust_alt_bn128_compression::PROGRAM_BYTES;
    static ref PROGRAM_CURVE25519: &'static [u8] = &solana_sbf_rust_curve25519::PROGRAM_BYTES;
    static ref PROGRAM_CUSTOM_HEAP: &'static [u8] = &solana_sbf_rust_custom_heap::PROGRAM_BYTES;
    static ref PROGRAM_ITER: &'static [u8] = &solana_sbf_rust_iter::PROGRAM_BYTES;
    static ref PROGRAM_MANY_ARGS: &'static [u8] = &solana_sbf_rust_many_args::PROGRAM_BYTES;
    static ref PROGRAM_MEM: &'static [u8] = &solana_sbf_rust_mem::PROGRAM_BYTES;
    static ref PROGRAM_MEMBUILTINS: &'static [u8] = &solana_sbf_rust_membuiltins::PROGRAM_BYTES;
    static ref PROGRAM_NOOP: &'static [u8] = &solana_sbf_rust_noop::PROGRAM_BYTES;
    static ref PROGRAM_PARAM_PASSING: &'static [u8] = &solana_sbf_rust_param_passing::PROGRAM_BYTES;
    static ref PROGRAM_POSEIDON: &'static [u8] = &solana_sbf_rust_poseidon::PROGRAM_BYTES;
    static ref PROGRAM_RAND: &'static [u8] = &solana_sbf_rust_rand::PROGRAM_BYTES;
    static ref PROGRAM_REMAINING_COMPUTE_UNITS: &'static [u8] =
        &solana_sbf_rust_remaining_compute_units::PROGRAM_BYTES;
    static ref PROGRAM_SANITY: &'static [u8] = &solana_sbf_rust_sanity::PROGRAM_BYTES;
    static ref PROGRAM_SECP256K1_RECOVER: &'static [u8] =
        &solana_sbf_rust_secp256k1_recover::PROGRAM_BYTES;
    static ref PROGRAM_SHA: &'static [u8] = &solana_sbf_rust_sha::PROGRAM_BYTES;
    pub static ref PROGRAMS: [&'static str; 18] = [
        "128bit",
        "alloc",
        "alt_bn128",
        "alt_bn128_compression",
        "curve25519",
        "custom_heap",
        "iter",
        "many_args",
        "mem",
        "membuiltins",
        "noop",
        "param_passing",
        "poseidon",
        "rand",
        "remaining_compute_units",
        "sanity",
        "secp256k1_recover",
        "sha",
    ];
}

pub fn load_program(name: &str) -> &'static [u8] {
    match name {
        "128bit" => &PROGRAM_128BIT,
        "alloc" => &PROGRAM_ALLOC,
        "alt_bn128" => &PROGRAM_ALT_BN128,
        "alt_bn128_compression" => &PROGRAM_ALT_BN128_COMPRESSION,
        "curve25519" => &PROGRAM_CURVE25519,
        "custom_heap" => &PROGRAM_CUSTOM_HEAP,
        "iter" => &PROGRAM_ITER,
        "many_args" => &PROGRAM_MANY_ARGS,
        "mem" => &PROGRAM_MEM,
        "membuiltins" => &PROGRAM_MEMBUILTINS,
        "noop" => &PROGRAM_NOOP,
        "param_passing" => &PROGRAM_PARAM_PASSING,
        "poseidon" => &PROGRAM_POSEIDON,
        "rand" => &PROGRAM_RAND,
        "remaining_compute_units" => &PROGRAM_REMAINING_COMPUTE_UNITS,
        "sanity" => &PROGRAM_SANITY,
        "secp256k1_recover" => &PROGRAM_SECP256K1_RECOVER,
        "sha" => &PROGRAM_SHA,
        _ => &[],
    }
}

pub fn programs() -> &'static [&'static str; 18] {
    &PROGRAMS
}
