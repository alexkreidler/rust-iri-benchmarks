# rust-iri-benchmarks

This project benchmarks the 3 major IRI ([Internationalized Resource Identifier](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier)) libraries I could find in Rust:

- iref
- iri-string
- sophia_iri
- (I also recently found [oxiri](https://github.com/oxigraph/oxiri), which I may test in another update)

IRIs are a [step up in complexity from regular URLs](https://www.ietf.org/rfc/rfc3987.txt), and storing and manipulating them can have a real performance cost. In a triple store or graph database, these differences in performance could be huge. Some key-value databases, for example, use prefix compression to make the in-memory representations of keys smaller.

I was interested to investigate both the performance and differences in APIs that these libraries provided.

## Behind the scenes

From what I've gathered, they all use a type similar to the following:

```rust
pub struct Iri<'a> {
    scheme: Option<&'a str>,
    authority: Option<&'a str>,
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

## TODOs

- Copy the generated benchmarks from my target dir to host on a static site server so other users can view
- Maybe setup CI
