# renderer-vulkano

Renderer with Rust Language and Vulkano middleware

## Set up

1. [安装 Rust](https://course.rs/first-try/installation.html)
2. Terminal 在工程根目录运行 `cargo run`

## Reference

### Rust

- [Rust 语言圣经](https://course.rs/about-book.html)
- [Rusty Book 锈书](https://rusty.rs/about.html) : Rust 的知名应用汇总
- [Visualizing memory layout of Rust's data types](https://www.bilibili.com/video/BV1KT4y167f1) : Rust 的内存管理视频，生动形象

### Vulkan

- [Vulkan Tutorial](https://vulkan-tutorial.com/) 官方教程
- [Vulkan Guide](https://vkguide.dev/) 比较简洁的教程
- [Vulkan Lecture Series](https://www.youtube.com/watch?v=tLwbj9qys18&list=PLmIqTlJ6KsE1Jx5HV4sd2jOe3V1KMHHgn&index=1) 生动形象的视频教程

### Vulkano

- [Vulkano](https://github.com/vulkano-rs/vulkano) 是官方推荐的 high-level rust vulkan api，
  - 使用 [Ash](https://github.com/ash-rs/ash) 访问 Vulkan api，Ash 是接近 Vulkan C++ Api 的 low-level rust api
  - 使用 [shaderc-rs](https://github.com/google/shaderc-rs) 编译 shader
  - 使用 winit 创建窗口
- [Vulkano-tutorial](https://github.com/bwasty/vulkan-tutorial-rs#swap-chain) Vulkano 模仿 Vulkan C++ 的教程，但是很久没更新了，推荐直接看 Vulkano 源码