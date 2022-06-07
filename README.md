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