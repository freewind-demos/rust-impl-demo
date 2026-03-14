fn main() {
    println!("=== Rust 方法和实现演示 ===\n");

    // 1. 基础方法调用
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形面积: {}", rect.area());
    rect.width = 10; // 修改字段
    println!("修改后面积: {}", rect.area());

    // 2. 多个 impl 块
    // 可以为同一个类型创建多个 impl 块
    rect.set_width(20);
    println!("set_width 后面积: {}", rect.area());

    // 3. 关联函数（类似构造函数）
    let square = Rectangle::square(10);
    println!("正方形面积: {}", square.area());

    // 4. 方法接收不同形式的 self
    let mut r1 = Rectangle { width: 10, height: 5 };
    r1.set_dimensions(20, 10);
    println!("r1 面积: {}", r1.area());

    // 不可变借用
    let r2 = Rectangle { width: 10, height: 5 };
    println!("r2 宽度: {}", r2.get_width());

    // 5. 链式调用示例
    let result = Rectangle::square(5)
        .set_width(10)
        .area();
    println!("链式调用面积: {}", result);

    println!("\n=== 总结 ===");
    println!("impl 块用于定义类型的方法");
    println!("&self 是不可变借用，&mut self 是可变借用");
    println!("self 本身会获取所有权（较少用）");
    println!("关联函数不接收 self，常用作构造函数");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 第一个 impl 块：基本方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 第二个 impl 块：更多方法
impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    // 返回 self 以支持链式调用
    fn set_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
