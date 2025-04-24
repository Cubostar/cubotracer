#![cfg_attr(
    target_os = "cuda",
    no_std,
    register_attr(nvvm_internal)
)]

#![allow(improper_ctypes_definitions)]

use cuda_std::*;
extern crate alloc;

pub mod objects;
pub mod materials;
pub mod world;
pub mod camera;
pub mod ray;
pub mod ppmhandler;
