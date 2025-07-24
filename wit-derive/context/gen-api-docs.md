1) generate using rustdoc

```sh
cargo rustdoc -p wasm_component_layer --output-format json -Z unstable-options
cargo rustdoc -p wasm_runtime_layer --output-format json -Z unstable-options
cargo rustdoc -p wit-parser@0.235.0 --output-format json -Z unstable-options
```


2) convert to markdown

```sh
# once (note: needs to be my own fork because official version is outdated)
cargo install rustdoc-md

rustdoc-md --path ./target/doc/wasm_component_layer.json --output ./context/wasm_component_layer.md
rustdoc-md --path ./target/doc/wasm_runtime_layer.json --output ./context/wasm_runtime_layer.md
rustdoc-md --path ./target/doc/wit_parser.json --output ./context/wit_parser.md
```

