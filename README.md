# wasmer4-microbench

The legendary battle between Wasmer 4 and Wasmer 2.2 in the micro-benchmarks arena.

Compares Singlepass and Cranelift backends in both compilation time and execution performance for both versions.

## Results

These are the outputs from running the benchmarks on my machine (x86 AMD CPU). The results may vary on different
machines.

```
test wasmer4::compile_large_cranelift    ... bench:  85,311,263 ns/iter (+/- 6,878,161)
test wasmer4::compile_large_single_pass  ... bench: 100,760,257 ns/iter (+/- 32,209,195)
test wasmer4::compile_medium_cranelift   ... bench:  10,540,740 ns/iter (+/- 1,968,193)
test wasmer4::compile_medium_single_pass ... bench:  20,877,361 ns/iter (+/- 4,504,683)
test wasmer4::compile_small_cranelift    ... bench:   6,640,782 ns/iter (+/- 1,463,444)
test wasmer4::compile_small_single_pass  ... bench:   2,619,003 ns/iter (+/- 527,398)
test wasmer4::exec_double_cranelift      ... bench:     435,272 ns/iter (+/- 49,444)
test wasmer4::exec_double_single_pass    ... bench:     444,480 ns/iter (+/- 2,239)
test wasmer4::exec_fib_cranelift         ... bench:     212,733 ns/iter (+/- 20,621)
test wasmer4::exec_fib_single_pass       ... bench:     306,690 ns/iter (+/- 30,240)
test wasmer4::exec_sha_cranelift         ... bench:     547,007 ns/iter (+/- 19,911)
test wasmer4::exec_sha_single_pass       ... bench:   2,202,029 ns/iter (+/- 57,151)

test wasmer22::compile_large_cranelift    ... bench: 526,452,124 ns/iter (+/- 29,008,466)
test wasmer22::compile_large_single_pass  ... bench: 103,057,898 ns/iter (+/- 35,507,808)
test wasmer22::compile_medium_cranelift   ... bench:  14,132,955 ns/iter (+/- 2,022,665)
test wasmer22::compile_medium_single_pass ... bench:  24,576,172 ns/iter (+/- 5,760,894)
test wasmer22::compile_small_cranelift    ... bench:   7,705,500 ns/iter (+/- 1,470,225)
test wasmer22::compile_small_single_pass  ... bench:   3,514,205 ns/iter (+/- 1,007,637)
test wasmer22::exec_double_cranelift      ... bench:     393,446 ns/iter (+/- 20,584)
test wasmer22::exec_double_single_pass    ... bench:     420,532 ns/iter (+/- 13,926)
test wasmer22::exec_fib_cranelift         ... bench:     204,258 ns/iter (+/- 11,088)
test wasmer22::exec_fib_single_pass       ... bench:     300,779 ns/iter (+/- 3,196)
test wasmer22::exec_sha_cranelift         ... bench:     543,167 ns/iter (+/- 3,337)
test wasmer22::exec_sha_single_pass       ... bench:   2,156,561 ns/iter (+/- 96,417)
```