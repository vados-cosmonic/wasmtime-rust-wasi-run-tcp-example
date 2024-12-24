# Wasmtime `wasi:cli/run`-able TCP example

This folder contains example code for a [WebAssembly Component][cm-book] runnable with [`wasmtime`][wasmtime]
that opens (and immediately closes) a TCP connection.

> ![NOTE]
> Not familiar with WebAssembly components or WASI? Start with the [Component Model book][cm-book]!n

This component can be used with `wasmtime run` because it implements the [WASI CLI world][wasi-cli]
(in particular the `wasi:cli/run` interface is exported). This means that any runtime (whether `wasmtime` or
any other runtime that supports WebAssembly components) can run the generated component.

[wasi]: https://github.com/WebAssembly/WASI/tree/main
[cm-book]: https://component-model.bytecodealliance.org/
[wasmtime]: https://github.com/bytecodealliance/wasmtime
[wasi-cli]: https://github.com/WebAssembly/wasi-cli
