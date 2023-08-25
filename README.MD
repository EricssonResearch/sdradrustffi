# Secure Rewind & Discard of Isolated Domains for Foreign Function Interface in Rust

This repository contains the source code for the paper [Friend or Foe Inside? Exploring In-Process Isolation to Maintain Memory Safety for Unsafe Rust](https://arxiv.org/pdf/2306.08127.pdf)

Accepted at [IEEE-SECDEV 2023](https://secdev.ieee.org/2023/home). 

## Abstract

Rust is a popular memory-safe systems programming language. In order to interact with hardware or call into non- Rust libraries, Rust provides unsafe language features that shift responsibility for ensuring memory safety to the developer. Failing to do so, may lead to memory safety violations in unsafe code which can violate safety of the entire application. In this work we explore in-process isolation with Memory Protection Keys as a mechanism to shield safe program sections from safety violations that may happen in unsafe sections. Our approach is easy to use and comprehensive as it prevents heap and stack-based violations. We further compare process-based and in-process isolation mechanisms and the necessary requirements for data serialization, communication, and context switching. Our results show that in-process isolation can be effective and efficient, permits for a high degree of automation, and also enables a notion of application rewinding where the safe program section may detect and safely handle violations in unsafe code.

## How to get started

This repository can be cloned using the following commands:
```
git clone --recursive https://github.com/EricssonResearch/sdradrustffi.git
```
SDRaD-FFI accommodates various serialization crates required for different purposes, including:

- abomonation_v1: original version for [abomonation](git@github.com:TimelyDataflow/abomonation.git)
- abomonation_v2: that has include `<Vec<u8>>` optimization for [abomonation](https://github.com/secure-rewind-and-discard/abomonation.git)
- bincode_v1: [bincode version 1](https://github.com/bincode-org/bincode)
- bincode_v2: [bincode version 2](https://github.com/bincode-org/bincode)


To compile SDRaD-FFI, run `cargo build` with selected features 

```
cargo build --features "abomonation_v1"
```


### Simple Example 
To compartmentalize functions belonging to an unsafe library interface, they can be
annotated with `sandbox!` macro provided similiar to [Sandcrust](https://github.com/atopia/sandcrust)

```
sandbox!{
    fn F (...) {
        unsafe {
            ... // unsafe code
        }
    }
}
```


### Hardware Requirements
`sdradrustffi` requires a CPU supporting Intel Memory Protection Keys (MPK) and a [Linux Kernel supporting MPK](https://www.kernel.org/doc/html/next/core-api/protection-keys.html)


### 
Note: Current version of `sdradrustffi` doesn't support Hashbrown table, that is used by `cargo test`. 

## License 

© Ericsson AB 2023

BSD 3-Clause License

