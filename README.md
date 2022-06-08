# mat2image

Rust crate that exposes `ToImage` trait.

It implements `ToImage` for
[Mat](https://docs.rs/opencv/latest/opencv/core/struct.Mat.html) to convert it 
into [DynamicImage](https://docs.rs/image/latest/image/enum.DynamicImage.html).

It also exposes `ToImageUnsafe`. The rationale behind this, comes after learning
through [profiling](#profiling) that
[Mat::iter](https://docs.rs/opencv/latest/opencv/core/struct.Mat.html#method.iter)
is painfully slow. Then `Mat::to_image_unsafe` is implemented using
[Mat::data](https://docs.rs/opencv/latest/opencv/core/trait.MatTraitConstManual.html#method.data)
accessing the raw data.

## TODO

- [ ] Refactor unsafe with
[data_bytes](https://docs.rs/opencv/latest/opencv/core/trait.MatTraitManual.html#method.data_bytes) and
[data_bytes_mut](https://docs.rs/opencv/latest/opencv/core/trait.MatTraitManual.html#method.data_bytes_mut)
- [ ] Can we convert BGR to RGB cheaply?

## Running examples

### Save as

This example reads an image (examples/tinta_helada.jpg) using
[opencv](https://docs.rs/opencv/latest/opencv/) and  saves it using
[image](https://docs.rs/image/latest/image/) API.

```
cargo run --release --example save_as [output_name[.jpg]=out.jpg]
```

## Profiling

When noticing that `to_image` takes too much time, I profiled using `perf` and
visualized it with [Firefox Profiler](https://profiler.firefox.com).

Full profiling:

```
# build example with debug symbols
cargo build --example save_as

# record perf data
# sudo because of `paranoid` kernel setting
sudo perf record -g -F 999 ./target/debug/examples/save_as

# convert output to firefox readable type
sudo perf script -F +pid >save_as.perf
```

Then load it in [Firefox Profiler](https://profiler.firefox.com).

**References**

- [Rust profiling](https://nnethercote.github.io/perf-book/profiling.html)
- [Firefox Profiler Guide](https://github.com/firefox-devtools/profiler/blob/main/docs-user/guide-perf-profiling.md)