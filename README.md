# Adds methods to read/write binary numbers to the `Read` and `Write` traits.

This is a simple crate that adds methods like `read_u8`, `read_u32_le`, `read_u64_be`, `write_i128_le`, `write_u16_be`, ... to the Rust [Read](https://doc.rust-lang.org/std/io/trait.Read.html) and [Write](https://doc.rust-lang.org/std/io/trait.Write.html) traits.

The `impl` feature includes the implementations for the WriterTypes and ReaderTypes traits. It is enabled by default.

# Example

### Reading

```rust
// read u32 (4 bytes) from a file in little endian.
let num = f.read_u32_le().unwrap();
```

```rust
// read i64 (8 bytes) from a file in big endian.
let num = f.read_i64_be().unwrap();
```

### Writing

```rust
// write u32 (4 bytes) to a file in little endian.
let num = f.write_u32_le(0xdeadbeef).unwrap();
```

```rust
// write i128 (16 bytes) to a file in big endian.
let num = f.write_i64_be(12345).unwrap();
```

This works with any type that implements [Read](https://doc.rust-lang.org/std/io/trait.Read.html) or [Write](https://doc.rust-lang.org/std/io/trait.Write.html).

# License

Public domain (Unlicense)
