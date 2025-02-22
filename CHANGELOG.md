# Changelog

This is the changelog of [Oct](https://crates.io/crates/oct/).
See `README.md` for more information.

## 0.19.0

* Implement `Default` for `vec::IntoIter`
* Mark `String::is_char_boundary` with `const`
* Update lints
* Clean up code
* Bump MSRV to `1.86` for `oct`
* Bump Rust edition to `2024` for `oct` and `oct-macros` (and `oct-benchmarks`)
* Fix `<vec::IntoIter as Iterator>::nth` not considering the current position
* Optimise `Iterator`, `DoubleEndedIterator`, and `ExactSizeIterator` implementations for `vec::IntoIter`
* Fix `<vec::IntoIter as Iterator>::size_hint` unsafely underflowing the size
* Fix `<vec::IntoIter as Iterator>::nth` allowing out-of-bounds reads
* Fix bounds on `Arc` include in `/oct/src/decode/decode/mod.rs`
* Implement `PartialEq<Cow<str>>` for `String`
* Bring back `vec` macro
* Update syntax for `string` macro
* Unmark `vec::IntoIter::{as_slice, as_mut_slice}` with `const`
* Optimise `PartialEq`, `Eq`, and `PartialOrd` implementations
* Rewrite `Clone` implementations for `Vec` and `vec::IntoIter`
* Update tests
* Unimplement `PartialEq<&mut [u8]>` for `Slot`
* Rewrite `Vec::into_boxed_slice`
* Rename `Vec::into_alloc_vec` to `into_vec`

## 0.18.0

* Clean up code
* Update `Hash` implementation for `Vec`
* Update lints
* Remove `vec` macro
* Rework `str` macro as `string`
* Remove `Vec::copy_from_slice`
* Rename `new` and `new_unchecked` in `Vec` to `copy_from_slice` and `copy_from_slice_unchecked`
* Add new `new` constructor to `Vec`
* Update docs
* Add `test_vec_new` test

## 0.17.1

* Clean up code
* Fix `<{IntoIter, Vec} as Drop>::drop`
* Update `track_caller` usage

## 0.17.0

* Update signature for `Slot::write`
* Remove `Slot::is_full`
* Specify version in root Cargo manifest
* Update docs
* Clean up code
* Add `str` and `vec` macro
* Add `track_caller` attribute to some functions
* Add `str_macro` and `vec_macro` tests
* Add `peek` and `peek_into` methods to `Input`

## 0.16.3

* Update repository link (SSH is not permitted on `crates.io`)

## 0.16.2

* Update repository link
* Clean up code
* Fix error bounds on `Decode` for `HashMap`
* Fix error bounds on `Encode` for `(T0, ..)`

## 0.16.1

* Update readme
* Update docs
* Update lints

## 0.16.0

* Reimplement `Decode` for `alloc::vec::Vec`, `alloc::string::String`, `CString`, `LinkedList`, `HashMap`, and `HashSet`
* Reimplement `DecodeBorrowed` for `alloc::vec::Vec`, `alloc::string::String`, and `CString`
* Update and add tests
* Update readme
* Clean up code
* Implement `From<Infallible>` for all error types
* Update docs
* Rework `EnumEncodeError` and `EnumDecodeError`
* Add `encode_char` benchmark
* Implement `Encode` and `Decode` for `OsStr`, `OsString`, `c_void`, and `BinaryHeap`
* Remove `never-type` feature flag
* Rework `PrimDiscriminant` as `PrimRepr`
* Add `PrimDiscriminant` enumeration
* Implement `From<T: PrimRepr>` for `PrimDiscriminant`
* Implement `PrimRepr` for `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`
* Implement `Debug`, `Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex`, `LowerExp`, and `UpperExp` for `PrimDiscriminant`
* Implement `Clone`, `Copy`, `Eq`, `PartialEq`, and `Hash` for `PrimDiscriminant`
* Implement `Eq` and `PartialEq` for **all** error types
* Add `as_slice` and `as_ptr` methods to `Input`
* Implement `AsRef<[u8]>` and `Borrow<[u8]>` for `Input`
* Implement `SizedEncode` for `c_void`
* Fix `<LinkedList as Encode>::Error`
* Add `new_unchecked` constructor to `String` and `Vec`
* Rework `from_utf8` and `from_utf8_unchecked` in `String`
* Remove `StringError`
* Rework `String` to make it trivially-destructable
* Actually mark `String::as_mut_str` with `const`
* Unimplement `PartialOrd<{&str, alloc::string::String}>` for `String`
* Implement `PartialEq<str>` for `String`
* Unimplement `PartialOrd<{[T; M], &[T], alloc::vec::Vec<T>}>` for `Vec<T, N>`
* Remove `is_full` method from `String` and `Vec`
* Implement `Copy` for `String`
* Implement `PartialEq<{Self, [u8], &[u8]}>`, `Eq`, and `Debug` for `Input`
* Implement `PartialEq<[U]>` for `Vec<T, ..>`
* Implement `PartialEq<Vec<U, ..>>` for `alloc::vec::Vec<T>`
* Implement `PartialEq<String>` for `alloc::string::String`
* Add `is_char_boundary` and `as_mut_bytes` methods to `String`
* Add doc aliases
* Update lints
* Fix atomics being imported from `std`

## 0.15.3

* Update readme
* Fix soundness hole in `<Vec as Decode>::decode`

## 0.15.2

* Clean up code
* Update docs
* Update lints
* Update crate metadata

## 0.15.1

* Update logo

## 0.15.0

* Rename `SizedStr` to `String`
* Rename `SizedSlice` to `Vec`
* Mark `String::as_mut_str` with `const`
* Update readme
* Relicence under MPL 2.0
* Licence logo
* Rename `SizedIter` to `IntoIter`
* Update docs
* Update copyright notices
* Add `vec`, `string`, and `slot` modules
* Refactor tests
* Clean up code
* Rename `LengthError` fields
* Implement `PartialEq` and `Eq` for some error types
* Remove `Utf16Error` and `CStringDecodeError`
* Rename `StrError` to `StringError`
* Remove `Self: Sized` requirement from `SizedEncode`
* Remove `T: Sized` requirement from `Encode` for `PhantomData<T>`
* Unimplement `Decode` for `alloc::vec::Vec`, `alloc::string::String`, `CString`, `LinkedList`, `HashMap`, and `HashSet`
* Unimplement `DecodeBorrowed` for `alloc::vec::Vec`, `alloc::string::String`, and `CString`
* Implement `Debug` for `Output`
* Remove `B: Sized` requirement from `Decode` for `Cow<'_, B>`
* Add `must_use` attribute to `Vec::{each_ref, each_mut}`
* Rework `Vec::set_len` and add `set_len_unchecked` method
* Remove `{String, Vec}::capacity`
* Implement `SizedEncode` for `PhantomPinned`
* License Cargo manifests
* License benchmarks
* Add temporary `never-type`, `f16`, and `f128` feature flags
* Remove `chars` and `char_indices` from `String`
* Implement `Drop` for `IntoIter` and `Vec`
* Implement `Encode`, `SizedEncode`, and `Decode` for `UnsafeCell`
* Update error requirements for some `Encode` and `Decode` implementations
* Fix soundness hole with `Vec::copy_from_slice`
* Rename `Vec::into_vec` to `into_alloc_vec`

## 0.14.5

* Update docs icon
* Update crate descriptions
* Update homepage URLs

## 0.14.4

* Update docs icon
* Update homepage URL
* Fix docs entries for the `encode` and `decode` modules

## 0.14.3

* Update readme
* Fix `SizedEncode` implementation for `Bound`
* Clean up code
* Fix license notices

## 0.14.2

* Update docs
* Update readme
* Update package description
* Clean up code

## 0.14.1

* Update logo
* Update readme

## 0.14.0

* Add more benchmarks
* Redefine `bool` scheme
* Remove `BoolDecodeError`
* Rename project to *oct*
* Rename `librum` crate to `oct`
* Rename `librum-macros` crate to `oct-macros`
* Rename `librum-benchmarks` crate to `oct-benchmarks`
* Update lints
* Update logo
* Restructure tests
* Rename `IStream` to `Input`
* Rename `OStream` to `Output`
* Make `Output::write` and `Input::{read, read_into}` fallible
* Add `OutputError` and `InputError` error types
* Mark `Output::write` and `Input::{read, read_into}` with `const`
* Add `position`, `capacity`, and `remaining` methods to `Output` and `Input`
* Rename `SizeError` to `LengthError`
* Rework some error types
* Fix feature flags for `From<CStringDecodeError>` for `GenericDecodeError`
* Rename `Buf` to `Slot`
* Remove `{Output, Input}::close`
* Implement `AsRef<[u8]>`, `Borrow<[u8]>`, `PartialEq<{Self, [u8], &[u8], &mut [u8]}>`, and `Eq` for `Output`
* Add `as_slice` and `as_ptr` methods to `Output`
* Add `encode` and `decode` modules
* Update homepage link
* Refactor code
* Update readme

## 0.13.1

* Update readme
* Null dependency patch versions

## 0.13.0

* Add decode benchmarks
* Add missing docs for `{Decode, Encode}::Error`
* Encode numericals in little-endian
* Add `read_into` method to `IStream`
* Fix `<AtomicBool as Decode>::Error` (and invalid values resulting in panics from `decode`)
* Add `Copy` bound to `PrimitiveDiscriminant`
* Update docs
* Refactor benchmarks
* Run benchmarks multiple times
* Bump dependency versions
* Update readme

## 0.12.1

* Update readme

## 0.12.0

* Support custom errors in `Encode` and `Decode` (using associated `Error` type)
* Further split `EncodeError` into `IsizeEncodeError`, `UsizeEncodeError`, `CollectionEncodeError`, `RefCellEncodeError`, `ItemEncodeError`, and `EnumEncodeError`
* Fix the `Encode` implementation of `LinkedList`
* Further split `DecodeError` into `BoolDecodeError`, `CharDecodeError`, `CStringDecodeError`, `NonZeroDecodeError`, `CollectionDecodeError`, `SystemTimeDecodeError`, `EnumDecodeError`, and `ItemDecodeError`
* Honour custom discriminant types
* Add `DecodeBorrowed` trait (implement appropriately)
* Implement `Decode` for `Cow`
* Refactor derive macros
* Update lints
* Rename test modules from `test` to `tests`
* Restructure some trait implementations
* Add `proc-macro` feature flag
* Add `GenericEncodeError` and `GenericDecodeError` error types for derived traits
* Add `PrimitiveDiscriminant` trait
* Lock `Arc` implementations behind atomic widths
* Add `?Sized` clauses to some `Encode` implementations
* Update readme
* Fix doc entry for `SizedStr::new`
* Update atomic tests
* Make `SizedEncode` a safe trait
* Do not automatically implement `Encode` when deriving `SizedEncode`
* Add `copy_from_slice` method to `SizedSlice`
* Add `each_ref` and `each_mut` methods to `SizedSlice`
* Add more benchmarks
* Remove Ciborium benchmarks
* Rename project to *Librum*
* Rename `bzipper` crate to `librum`
* Rename `bzipper_macros` crate to `librum-macros`
* Rename `bzipper_benchmarks` crate to `librum-benchmarks`

## 0.11.0

* Add `into_bytes` destructor to `SizedStr`
* Clean up code
* Update and add more tests
* Manually implement `<SizedIter as Iterator>::nth`
* Implement `Encode` and `Decode` for `CString`, `SystemTime`, `Duration`
* Implement `Encode` for `CStr`
* Update docs
* Fix includes in `/bzipper/src/decode/mod.rs` and `/bzipper/src/sized_encode/mod.rs`
* Add new `NullCString` and `NarrowSystemTime` error variants to `DecodeError`
* Optimise `<String as Decode>::decode`
* Update lints
* Implement `SizedEncode` for `SystemTime` and `Duration`
* Update benchmark stats
* Update readme

## 0.10.1

* Clean up and refactor code
* Add more tests
* Fix `DoubleEndedIterator` implementation for `SizedIter`

## 0.10.0

* Clean up code
* Implement `Encode` and `Decode` for `Cell` and `HashSet`
* Implement `SizedEncode` for `Cell`
* Add missing `SizedEncode` implementations for `Cow`, `LazyCell`, and `LazyLock`
* Unimplement `Decode` for `Cow`, `LazyCell`, and `LazyLock`
* Add missing `Decode` implementations for `RefCell`
* Fix feature flags for `SizedEncode` implementations of `Rc` and `Arc`

## 0.9.0

* Implement `Encode` and `Decode` for `LinkedList`, `HashMap`, `Cow`, `PhantomPinned`, `LazyCell`, `LazyLock`
* Add missing `Decode` implementation for `Box`
* Update inline rules
* Implement traits for tuples using macros
* Implement `SizedEncode` for `PhantomPinned`, `Cow`, `LazyCell`, `LazyLock`, `&_`, `&mut _`
* Implement `Encode` for `&_` and `&mut _`
* Update docs

## 0.8.1

* Update package metadata

## 0.8.0

* Rename `FixedString` to `SizedStr`
* Implement `PartialEq<String>` and `PartialOrd<String>` for `SizedStr`
* Add constructors `from_utf8` and `from_utf8_unchecked` to `SizedStr`
* Remove `pop`, `push_str`, and `push` from `SizedStr`
* Implement `FromIterator<char>` for `SizedStr`
* Rename `Serialise` to `Encode`
* Rename `Deserialise` to `Decode`
* Remove `Sized` requirement for `Encode`
* Add benchmarks
* Update package metadata
* Rename `Sstream` to `OStream`
* Rename `Dstream` to `IStream`
* Update readme
* Refactor code
* Update lints
* Implement `Encode` and `Decode` for `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `Mutex`, `Box`, `RwLock`, `Rc`, `Arc`, `Wrapping`, `Saturating`, `AtomicBool`, `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicUsize`, `AtomicIsize`, `SocketAddrV4`, `SocketAddrV6`, `SocketAddr`, `Range`, `RangeFrom`, `RangeFull`, `RangeInclusive`, `RangeTo`, `RangeToInclusive`, `Bound`, `RefCell`, `String`, and `Vec`
* Update docs
* Add `SizedSlice` type
* Add `SizedIter` type
* Rename `Buffer` type to `Buf`
* Remove `Add` and `AddAssign` implementations from `SizedStr`
* Add *Features* section to readme
* Honour explicit enumeration discriminants
* Encode enumeration discriminants as `isize`
* Add `SizedEncode` trait
* Outsource `MAX_SERIALISED_SIZE` to `SizedEncode` as `MAX_ENCODED_SIZE`
* Implement `Iterator`, `ExactSizeIterator`, `FusedIterator`, and `DoubleEndedIterator` for `SizedIter`
* Implement `AsRef<[T]>` and `AsMut<[T]>` for `SizedIter<T, ..>`
* Implement `Clone` for `SizedIter`
* Add `as_slice` and `as_mut_slice` methods to `SizedIter`
* Add `from_raw_parts` constructor and `into_raw_parts` destructor to `SizedSlice`
* Add `set_len` method to `SizedSlice`
* Add `len`, `is_empty`, `is_full`, and `capacity` methods to `SizedSlice`
* Add `as_slice` and `as_mut_slice` methods to `SizedSlice`
* Add `as_ptr` and `as_mut_ptr` methods to `SizedSlice`
* Implement `AsMut<[T]>` and `AsRef<[T]>` for `SizedSlice<T, ..>`
* Implement `Borrow<[T]>` and `BorrowMut<[T]>` for `SizedSlice<T, ..>`
* Implement `Deref<[T]>` and `DerefMut<[T]>` for `SizedSlice<T, ..>`
* Implement `Debug` for `SizedSlice`
* Implement `Default` for `SizedSlice`
* Implement `Clone` for `SizedSlice`
* Implement `Encode`, `Decode`, and `SizedEncode` for `SizedSlice`
* Implement `Eq` and `PartialEq` for `SizedSlice`
* Implement `Ord` and `PartialOrd` for `SizedSlice`
* Implement `From<[T; N]>` for `SizedSlice<T, N>`
* Implement `Hash` for `SizedSlice`
* Implement `Index` and `IndexMut` for `SizedSlice`
* Implement `IntoIterator` for `SizedSlice` (including references hereto)
* Implement `TryFrom<&[T]>` for `SizedSlice<T, ..>`
* Implement `From<SizedSlice<T, ..>>` for `Vec<[T]>`
* Implement `From<SizedSlice<T, ..>>` for `Box<[T]>`
* Add `into_boxed_slice` and `into_vec` destructors to `SizedSlice`
* Add `into_boxed_str` and `into_string` destructors to `SizedStr`
* Bump Rust version to `1.83` for `bzipper`
* Mark `SizedStr::as_mut_ptr` as const
* Implement `FromIterator<T>` for `SizedSlice<T, ..>`
* Make `SizedStr::new` take a `&str` object
* Add `is_empty` and `is_full` methods to `Buf`
* Disallow non-empty single-line functions
* Add `SAFETY` comments
* Implement `PartialEq<&mut [u8]>` and `PartialEq<[u8]>` for `Buf`
* Implement `Index` and `IndexMut` for `Buf`
* Add `from_raw_parts` constructor and `into_raw_parts` destructor to `Buf`
* Add *Documentation* and *Contribution* sections to readme
* Add *Copyright & Licence* section to readme
* Add Clippy configuration file
* Add more unit tests
* Add debug assertions
* Remove `as_ptr` and `as_slice` methods from `IStream` and `OStream`
* Remove `len`, `is_empty`, and `is_full` methods from `IStream` and `OStream`
* Unimplement all manually-implemented traits from `IStream` and `OStream`
* Mark `new` and `write` in `OStream` as const
* Mark the `read` method in `IStream` as const
* Add `close` destructor to `OStream` and `IStream`
* Implement `Encode` for `[T]` and `str`
* Encode `usize` and `isize` as `u16` and `i16` again
* Split `Error` type into `EncodeError`, `DecodeError`, `Utf8Error`, `Utf16Error`, `SizeError`, and `StringError`
* Remove `Result` type
* Add `error` module
* Make `IStream::read` and `OSream::write` panic on error
* Update logo
* Add more examples to docs
* Unmark all functions in `Buf` as const
* Implement `From<SizedStr>` for `Box<str>`
* Always implement `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, and `UnwindSafe` for `Buf`
* Add *Examples* section to readme
* Implement `SizedEncode` for all previous `Encode` types
* Bump dependency versions
* Implement `SizedEncode` for `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `Mutex`, `Box`, `RwLock`, `Rc`, `Arc`, `Wrapping`, `Saturating`, `AtomicBool`, `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicUsize`, `AtomicIsize`, `SocketAddrV4`, `SocketAddrV6`, `SocketAddr`, `Range`, `RangeFrom`, `RangeFull`, `RangeInclusive`, `RangeTo`, `RangeToInclusive`, `Bound`, and `RefCell`

## 0.7.0

* Make `alloc` and `std` default features
* Make serialisations variably sized again
* Refactor derive implementations
* Completely rework streams
* Fix tuple deserialisation
* Encode `FixedString` in UTF-8
* Remove methods `from_chars` and `set_len` from `FixedString`
* Rename `as_slice` and `as_mut_slice` methods in `FixedString` to `as_str` and `as_mut_str`
* Add methods `as_bytes`, `push_str`, `chars`, `capacity`, and `char_indices` to `FixedString`
* Rework `FixedString` traits
* Remove `FixedIter`
* Update lints
* Add methods `set_len` and `set_len_unchecked` to `Buffer`
* Elaborate docs
* Update readme
* Do not require `Serialise` for `Deserialise`
* Rename `SERIALISED_SIZE` in `Serialise` to `MAX_SERIALISED_SIZE`
* Use streams in `Serialise` and `Deserialise`
* Drop `Serialise` requirement for `Buffer`
* Add methods `with_capacity` and `capacity` to `Buffer`

## 0.6.2

* Fix `Deserialise` derive for unit variants
* Refactor `Serialise` derive for enumerations

## 0.6.1

* Bump dependency version
* Update docs
* Add more examples

## 0.6.0

* Update readme
* Add `Buffer` type
* Bump minor version
* Implement `PartialEq<&[char]>` for `FixedString`
* Update tests
* Implement `PartialOrd<&[char]>` and `PartialOrd<&str>` for `FixedString`
* Remove custom methods `get`, `get_unchecked`, `get_mut`, and  `get_unchecked_mut`, `iter`, and `iter_mut` from `FixedString`

## 0.5.2

* Respecify version numbers

## 0.5.1

* Specify `bzipper_macros` version

## 0.5.0

* Bump minor version
* Add macros crate
* Add derive macros
* Update package metadata
* Update readme
* Expand docs
* Require fixed size (de)serialisations
* Add more error variants
* Require `bzipper::Error` for (de)serialisation
* Reworks streams
* Remove `Buffer`
* Rework `FixedString`
* Serialise `usize` and `isize` as `u32` and `i32`, respectively
* Rework arrays (de)serialisation
* Fix `Result` serialisations
* Add new logo
* Add features `alloc` and `std`
* Specify rustc version
* Rename `FixedStringIter` to `FixedIter`
* Implement `Serialise` and `Deserialise` for single tuples and `PhantomData`

## 0.4.7

* Extensively elaborate docs
* Update readme

## 0.4.6

* Fix docs logo (again)
* Update docs (add examples)

## 0.4.5

* Fix package metadata :(

## 0.4.4

* Fix docs logo

## 0.4.3

* Reformat changelog
* Update logo
* Add docs logo

## 0.4.2

* Update package metadata

## 0.4.1

* Update readme

## 0.4.0

* Add logo
* Clean up code
* Fix array deserialisation (require `Default`)
* Bump minor
* Update commenting
* Make serialisations fallible
* Impl `Serialise` and `Deserialise` for `usize` and `isize` (restrict to 16 bits)
* Add new errors: `UsizeOutOfRange`, `IsizeOutOfRange`
* Rework sstreams
* Add buffer type
* Fix serialisation of `Option<T>`
* Disable `std`
* Rename error: `EndOfDStream` -> `EndOfStream`
* Update documentation
* Update readme
* Reformat changelog

## 0.3.0

* Bump minor
* Document errors
* Rename: `ArrayLengthMismatch` -> `ArrayTooShort`
* Remove error `FixedStringTooShort`
* Rename: `InvalidUtf8` -> `BadString`
* Rework errors
* Rename methods: `as_d_stream` -> `as_dstream`, `to_s_stream` -> `to_sstream`
* Add `SERIALISATION_LIMIT` constant to `Serialise`
* Make some deserialisations infallible
* Add method `append_byte` to `SStream`
* Add method `take_byte` to `DStream`
* Rename `SStream` -> `Sstream`, `DStream` -> `Dstream`
* Update readme
* Update documentation
* Make `Deserialise` require `Serialise`
* Fix copyright/license notice in `"src/serialise/test.rs"`

## 0.2.0

* Clean up code
* Implement `Ord` and `PartialOrd` for `FixedString`
* Implement `Index` and `IndexMut` for `FixedString`
* Add `get` and `get_mut` methods to `FixedString`
* Implement `From<[char; N]>` for `FixedString`
* Bump minor
* Implement `Serialise` and `Deserialise` for tuples

## 0.1.0

* Bump minor
* Export all in crate root
* Add fixed string type
* Add new errors
* Update documentation
* Add `as_d_stream` method to `SStream`
* Add `to_s_stream` and `as_slice` methods to `DStream`

## 0.0.2

* Add license files

## 0.0.1

* Fix copyright notices
* Add license notices
* Update readme

## 0.0.0

* Add changelog
* Fork from `backspace`
* Add gitignore
* Add documentation
* Add tests
* License under LGPL-3
* Configure lints
* Add readme
