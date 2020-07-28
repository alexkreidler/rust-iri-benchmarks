# rust-iri-benchmarks

This project benchmarks the 3 major IRI ([Internationalized Resource Identifier](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier)) libraries I could find in Rust:

- iref
- iri-string
- sophia_iri

IRIs are a relatively complex specification, and as strings can take up a lot of memory, especially if used in a Linked Data-type database. Some key-value databases use prefix compression to make the in-memory representations of keys smaller.

I was interested to figure out the overhead of these different libraries, as well as any differences in their API surface.

## Behind the scenes

From what I've gathered, they all use a type similar to the following:

```rust
pub struct Iri<'a> {
    scheme: Option<&'a str>,
    authority: Option<&'a str>,
    /// NB: path complies with the following rules:
    /// - does not contain the separators ('/')
    /// - its first element is "" if the path starts with '/'
    /// - its last element is "" if the path ends with a '/'
    path: Vec<&'a str>,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}
```

I've listed their specific type names below, in the order of the libs above:

- `ParsedIriRef`
- `RiReferenceComponents`
- `IriParsed`

Although they all use a similar type, they have differences in how these are parsed.

`sophia_iri` uses Regex expressions, which it appears based on the one benchmark so far make it slower.

`iri-string` uses raw parsing functions based on the [Nom](https://github.com/Geal/nom) parsing library.

`iref` appears to use a no library at all, just completely custom parsing code.

## Results

Right now, only one benchmark is implemented (instantiating the various IRIs from `str`s). However, `iri-string` and `iref` are in the lead, trading places.

More work to improve benchmark efficiency is required.
