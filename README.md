
[![hashids](http://hashids.org/public/img/hashids.gif "Hashids")](http://hashids.org/)

**Harsh** is a Rust implementation of the **Hashids** JavaScript library to generate YouTube-like ids from numbers. Use it when you don't want to expose your database ids to the user: [http://hashids.org/javascript](http://hashids.org/javascript)

Quick example
-------

```rust
let harsh = Harsh::default();
let id = harsh.encode(&[1, 2, 3]); // "o2fXhV"
let numbers = harsh.decode(id).unwrap(); // [1, 2, 3]
```

**Make your ids unique:**

Pass a project name to make your ids unique:

```rust
let harsh = Harsh::builder().salt("My Project").build().unwrap();
let id = harsh.encode(&[1, 2, 3]); // "Z4UrtW"

let harsh = Harsh::builder().salt("My Other Project").build().unwrap();
let id = harsh.encode(&[1, 2, 3]); // "gPUasb"
```

**Use padding to make your ids longer:**

Note that ids are only padded to fit **at least** a certain length. It doesn't mean that your ids will be *exactly* that length.

```rust
let harsh = Harsh::default(); // no padding
let id = harsh.encode(&[1]); // "jR"

let harsh = Harsh::builder().length(10).build().unwrap(); // pad to length 10
let id = harsh.encode(&[1]); // "VolejRejNm"
```

**Pass a custom alphabet:**

```rust
let harsh = Harsh::builder().alphabet("abcdefghijklmnopqrstuvwxyz").build().unwrap(); // all lowercase
let id = harsh.encode(&[1, 2, 3]); // "mdfphx"
```

**Encode hex instead of numbers:**

Useful if you want to encode [Mongo](https://www.mongodb.com/)'s ObjectIds. Note that *there is no limit* on how large of a hex number you can pass (it does not have to be Mongo's ObjectId).

```rust
let harsh = Harsh::default();

let id = harsh.encode_hex("507f1f77bcf86cd799439011").unwrap(); // "y42LW46J9luq3Xq9XMly"
let hex = harsh.decode_hex("y42LW46J9luq3Xq9XMly").unwrap(); // "507f1f77bcf86cd799439011" 
```

Pitfalls
-------

1. When decoding, output is always an array of numbers (even if you encode only one number):

	```rust
	let harsh = Harsh::default();

    let id = harsh.encode(&[1]);
    println!("{:?}", harsh.decode(&id).unwrap()); // [1]
	```

2. Encoding negative numbers is not supported.
3. If you pass bogus input to `encode()`, an empty string will be returned:

	```rust
	let harsh = Harsh::default();

	let id = harsh.decode("a123"); // note lack of unwrap call; would panic here
	println!("{:?}", id); // ""
	```

4. Do not use this library as a security tool and do not encode sensitive data. This is **not** an encryption library.

Randomness
-------

The primary purpose of Hashids is to obfuscate ids. It's not meant or tested to be used as a security or compression tool. Having said that, this algorithm does try to make these ids random and unpredictable:

No repeating patterns showing there are 3 identical numbers in the id:

```rust
let harsh = Harsh::default();
println!("{}", harsh.encode(&[5, 5, 5])); // A6t1tQ
```

Same with incremented numbers:

```rust
let harsh = Harsh::default();

println!("{}", harsh.encode(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])); // wpfLh9iwsqt0uyCEFjHM

println!("{}", harsh.encode(&[1])); // jR
println!("{}", harsh.encode(&[2])); // k5
println!("{}", harsh.encode(&[3])); // l5
println!("{}", harsh.encode(&[4])); // mO
println!("{}", harsh.encode(&[5])); // nR
```

Curses! #$%@
-------

This code was written with the intent of placing created ids in visible places, like the URL. Therefore, the algorithm tries to avoid generating most common English curse words by generating ids that never have the following letters next to each other:

	c, f, h, i, s, t, u

Support
-------

Have a question? Open an issue here, or find the author of the original JavaScript library: 

> [@IvanAkimov](http://twitter.com/ivanakimov) or [ivanakimov.com](http://ivanakimov.com)

Maybe one of these days I'll get around to fixing my website up. :)

Changelog
---------

### 0.2.0

- Convert to result-based API and add quickcheck tests courtesy of Dr-Emann.

### 0.1.5

- Fix panic when decoding values containing characters not found in alphabet

### 0.1.3

- Remove dependency on clippy. (Still using clippy, but just as `cargo clippy` now.)

### 0.1.2

- Changed `HarshFactory` to `HarshBuilder` in order to stop rubbing my OCD the wrong way<sup>1</sup>
- Updated dependencies

> 1. I apologize for the inconvenience this causes, but we all know this is better in the long run; if I stay sane, I can continue to keep this lib up to date!

License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE][license-url-ap2] or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT][license-url-mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[travis-url]: https://travis-ci.org/archer884/harsh
[travis-image]: https://travis-ci.org/archer884/harsh.svg?branch=master

[appveyor-url]: (https://ci.appveyor.com/project/archer884/harsh)
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/archer884/harsh?branch=master&svg=true

[license-url-mit]: https://github.com/archer884/harsh/blob/master/LICENSE-MIT
[license-url-ap2]: https://github.com/archer884/harsh/blob/master/LICENSE-APACHE
