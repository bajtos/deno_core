// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
#![deny(warnings)]
deno_ops_compile_test_runner::prelude!();

#[op2(fast)]
fn op_buffers(#[buffer] _a: &[u8], #[buffer] _b: &mut [u8]) {
}
