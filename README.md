# Axes

Axes is a layout engine written in Rust.

The goal of the project is simple, to provide a fast and predictable layout engine.

> [!WARNING]
> Axes is still a work in progress. The API may change.

## What is Axes?

Axes implements a custom layout model. It is **not** a CSS Flexbox/Grid engine.

If you need CSS compatibility, you should probably use Taffy instead.  
But if you need a lightweight and fast layout engine that you can embed into your own engine, Axes might be a better fit.

## Comparison

Axes was benchmarked against Taffy 0.9.2 from crates.io.

Benchmark results can be found in `benches/results-26-02-2026.md`.

In these benchmarks, Axes performs faster in all tested cases (Approximately 2x faster than Taffy).

Taffy supports significantly more layout features and full CSS-like behavior, which affects performance characteristics. Axes focuses on a smaller, custom layout model.

## Contributing

Contributions are welcome.

If you want to help:

- Open an issue to discuss ideas or report bugs
- Submit a pull request
- Improve documentation or examples

Since the project is still evolving, breaking changes may happen between minor versions.

## License

Axes is licensed under the MIT License.
