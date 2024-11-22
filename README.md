# Denox

This is a Rustler NIF that interacts with the V8 JavaScript engine using the
[deno_core](https://github.com/denoland/deno_core) crate and rustler. The use
case is pretty simple and specific:

- Evaluate JavaScript code with given bindings
- Return data from JavaScript code

Each execution results in a new V8 context that is then discarded. This is by
design so that you don't have to worry about global variable pollution or
leaking confidential data to other code that runs in the same process.

## Installation

This is not yet available on Hex, so you can add it directly from GitHub for now:

```elixir
def deps do
  [
    {:denox, github: "mylanconnolly/denox"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc).
