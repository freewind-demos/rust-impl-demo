# Rust Impl Demo

## 简介

演示 Rust 的方法实现。

## 基本原理

使用 impl 块为结构体、枚举等定义方法。

## 启动和使用

```bash
cargo run
```

## 教程

### 基本方法

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### self 的形式

```rust
fn area(&self)      // 不可变借用
fn set_width(&mut self) // 可变借用
fn consume(self)    // 获取所有权
```

### 关联函数

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```
