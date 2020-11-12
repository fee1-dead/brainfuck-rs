# brainfuck-rs
Generate b****fuck with Rust macros. 

To run brainfuck code simply wrap it around a macro invocation, for example helloworld in brainfuck:

```rust
fn main() {
    bf! {
      ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]
      >>.>---.+++++++..+++.>>.<-.<.+++.------.--------.
      >>+.>++.
    }
}
```
