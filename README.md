# rs2ts
Rust structs and enums to TypeScript interfaces and enums 

CURRENTLY DOING:

* Procedural macro -based compilation of Rust structs and (C-like) enums into TypeScript structs and enums.

The advantage of procedural macros is that there's no extra configuration required and no extra build steps required. Parsing is blazing fast since we benefit from incremental compilation.

PLANNED:

* Use as a standalone tool: Command line executable that takes Rust file(s) as a parameter and generates TS code.

CURRENT STAGE:

See `rs2ts-proc/tests/test.rs`. Can currently e.g. do:

```rust
#[derive(ParseToTS)]
struct TestStruct {
  x: f32,
  y: f32,
  name: String,
  active: bool
}
```

And the result is:

```typescript
interface TestStruct {
  x: number;
  y: number;
  name: string;
  active: boolean;
}
```

Currently a new file is created for every interface, using the name of the Rust struct.

Files are written to `target/ts`.

Very much work-in-progress. :-)
