# Example: Add a custom tracing subscriber

An example project showing how to add a custom subscriber for the tracing crate.

## Running

When running this example, you should observe the following output:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/example-rust-tracing-subscriber`
2022-12-11T18:27:26.536167Z ERROR example_rust_tracing_subscriber: Hic forum est.
The source 'example_rust_tracing_subscriber' says on the 'ERROR' level: 'Hic forum est.'
2022-12-11T18:27:26.536202Z  INFO example_rust_tracing_subscriber: Populus properat.
The source 'example_rust_tracing_subscriber' says on the 'INFO' level: 'Populus properat.'
```

The `RUST_LOG` environment variable is correctly considered:

```shell
$ RUST_LOG=debug cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/example-rust-tracing-subscriber`
2022-12-11T18:28:37.214611Z ERROR example_rust_tracing_subscriber: Hic forum est.
The source 'example_rust_tracing_subscriber' says on the 'ERROR' level: 'Hic forum est.'
2022-12-11T18:28:37.214642Z  INFO example_rust_tracing_subscriber: Populus properat.
The source 'example_rust_tracing_subscriber' says on the 'INFO' level: 'Populus properat.'
2022-12-11T18:28:37.214654Z DEBUG example_rust_tracing_subscriber: In vino veritas.
The source 'example_rust_tracing_subscriber' says on the 'DEBUG' level: 'In vino veritas.'
```
