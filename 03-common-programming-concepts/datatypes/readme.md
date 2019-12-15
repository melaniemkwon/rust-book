https://doc.rust-lang.org/book/ch03-02-data-types.html

## Scalar Types
### Integer types

Length	|Signed	|Unsigned
--- | --- | ---
8-bit	|`i8`	    |`u8`
16-bit	|`i16`	|`u16`
32-bit	|`i32`	|`u32`
64-bit	|`i64`	|`u64`
128-bit	|`i128`	|`u128`
arch	|`isize`	|`usize`

### Integer Literals
Number literals	| Example
--- | ---
Decimal	| `98_222`
Hex	| `0xff`
Octal	| `0o77`
Binary	| `0b1111_0000`
Byte (`u8` only)	| `b'A'`

In general, use integer default `i32` which is fast.

### Floating-Point Types
Length	| primitive type
--- | ---
32-bit	|`f32` single precision
64-bit	|`f64` double precision