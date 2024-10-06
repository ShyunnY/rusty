use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[allow(dead_code)]
pub fn hello() {
    // Cell和RefCell
    /*
       Cell和RefCell为什么会存在?
       Rust 的编译器之严格, 可以说是举世无双!
       特别是在所有权方面, Rust 通过严格的规则来保证所有权和借用的正确性, 最终为程序的安全保驾护航
       但是严格是一把双刃剑, 带来安全提升的同时, 损失了灵活性

       因此Rust提供了Cell和RefCell用于内部可变性
       简而言之: "可以在拥有不可变引用的同时修改目标数据"(相当于视借用规则而不见)
       对于正常的代码实现来说, 这个是不可能做到的（要么一个可变借用, 要么多个不可变借用）

       内部可变性的实现是因为Rust使用了"unsafe"来做到这一点
       但是对于使用者来说这些都是透明的, 因为这些"不安全代码都被封装到了安全的API中"
    */

    // (1) Cell
    // Cell 和 RefCell 在功能上没有区别, 区别在于 Cell<T> 适用于 "T 实现 Copy" 的情况
    {
        // 有几点需要注意:
        // 1. "origin"是&str类型, 实现了Copy（就是拷贝个指针呀）
        // 2. "c.get()" 获取一个值, "c.set()" 设置一个值
        // 3. 我们可以看到变量one获取了Cell内部的引用, 但同时我们还能修改Cell内部的值(其实就是Copy发威)
        // **这个违背了Rust的借用规则, 但是由于 Cell 的存在, 我们很优雅地做到了这一点**

        let origin = Cell::new("origin");

        let one = origin.get();
        println!("origin : {one}");

        origin.set("demo");
        let two = origin.get();
        println!("one,two: {one},{two}");
    }

    // (2) RefCell
    // 由于Cell类型针对的是实现了 "Copy" 特征的值类型
    // 因此在实际开发中, Cell 使用的并不多.
    // 因为我们要解决的往往是"可变、不可变引用共存导致的问题", 此时就需要借助于 "RefCell" 来达成目的
    /*
       一个数据只有一个所有者: Rc/Arc让一个数据可以拥有多个所有者
       要么多个不可变借用, 要么一个可变借用: RefCell实现编译期可变、不可变引用共存
       违背规则导致编译错误: 违背规则导致运行时panic

       可以看出: Rc/Arc 和 RefCell 合在一起, 解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题
       但是它们并不是银弹!
       例如 RefCell 实际上"并没有解决可变引用和引用可以共存的问题"
       只是将报错从编译期推迟 => 到运行时, 从编译器错误变成了 panic 异常
       简而言之: RefCell只是将借用规则从编译期的检查放到了运行时检查, 如果违反了规则就会进行panic!
    */
    {
        let rc = RefCell::new(String::from("xxoo"));
        // RefCell不可变借用
        let _rc_immut = rc.borrow();
        // RefCell可变借用
        // let _rc_mut = rc.borrow_mut();

        // 实际上此时已经违反了 可变借用和不可变借用 同时存在的问题
        // 但是编译期并没有报错, 如果执行则会panic
        // println!("rc_immut: {} rc_mut: {}", _rc_immut, _rc_mut);
    }
    //
    // RefCell 为何存在???
    // 可能很多人都会有疑问:  还不如在编译期报错, 至少能提前发现问题, 而且性能还更好
    // 存在即合理,究其根因. 在于 Rust 编译期的"宁可错杀，绝不放过"的原则
    // 当编译器不能确定你的代码是否正确时, 就统统会判定为错误, 因此难免会导致一些误报
    // 而 RefCell "正是用于你确信代码是正确的, 而编译器却发生了误判时"
    //
    // RefCell 的运行时错误在这种情况下也变得非常可爱: 一旦有人做了不正确的使用, 代码会 panic!
    // 然后告诉我们哪些借用冲突了
    // 总之当你确信编译器误报但不知道该如何解决时, 或者你有一个引用类型
    // 需要被四处使用和修改然后导致借用关系难以管理时, 都可以优先考虑使用 "RefCell" !!!
    //
    /*
       RefCell 的简单总结:
       1.与 Cell 用于可Copy的值(Copy可以拷贝一个副本, 所以本质上不是操作同一个数据)不同, RefCell 用于引用
       2.RefCell "只是将借用规则从编译期推迟到程序运行期", 并 "不能帮你绕过这个规则"
       3.RefCell 适用于编译期误报或者"一个引用被在多处代码使用、修改以至于难于管理借用关系"时
       4.使用 RefCell 时, 违背借用规则会"导致运行期的 panic"
    */
    //
    /*
       选择 Cell 还是 RefCell???
       1.Cell 只适用于 Copy 类型, 用于"提供值". 而 RefCell 用于提供引用
       2.Cell 不会panic, 而 RefCell 会panic

       与 Cell 的 zero cost 不同, RefCell 其实是有一点运行期开销的
       原因是它包含了一个字节大小的“借用状态”指示器, 该指示器在每次运行时借用时都会被修改
       进而产生一点开销...

       总之当非要使用内部可变性时, 首选 Cell! 只有你的类型"没有实现 Copy 时", 才去选择 RefCell
       (但是似乎很多自定义类型都难以实现Copy...)
    */

    // (3) 内部可变性
    // 之前我们提到 RefCell 具有内部可变性, 何为内部可变性？
    // 简单来说: "对一个不可变的值进行可变借用", 但这个"并不符合 Rust 的基本借用规则"
    {
        // 例如以下代码:
        // 我们不能对一个不可变的值进行可变借用, 这会破坏 Rust 的安全性保证
        // let x = 10;
        // let y = &mut x;
    }
    // 虽然基本借用规则是Rust的基石, 然而在某些场景中:
    // 一个值可以在"其方法内部被修改", 同时"对于其它代码不可变"是很有用的
    {
        // Example
        // 由于trait签名定义在外部, 我们无法对其进行修改
        // 但是我们又希望进行可变借用, 此时我们就需要使用 RefCell 把一个不可变 => 变成可变

        // === 外部trait ===
        trait Send {
            fn send_msg(&self, msg: String);
        }

        // === 内部实现 ===
        struct MemoryCache {
            queue: RefCell<Vec<String>>, // 使用RefCell让不可变 => 变成可变
        }

        impl Send for MemoryCache {
            fn send_msg(&self, msg: String) {
                self.queue.borrow_mut().push(msg);
            }
        }
    }
    //
    {
        // Example:

        let x = 10;
        // let y = &mut x;  // 无法把一个不可变的类型进行可变借用

        // 但是我们可以通过 RefCell 进行操作: 对其进行可变借用
        let y = RefCell::new(x);
        let mut z = y.borrow_mut();
        *z = 1 + 1;
    }

    // (4) Rc+RefCell组合使用
    // 在 Rust 中, 一个常见的组合就是 Rc 和 RefCell 在一起使用
    // 前者可以实现"一个数据拥有多个所有者", 后者可以"实现数据的可变性"
    // ** 因为Rc默认是进行不可变借用!!! **
    // ** 如果我们希望多个所有者并且拥有数据可变性, 那么我们通常都会使用 Rc + RefCell **
    {
        // example
        // 由于Rc的所有者们"共享同一个底层的数据", 因此当一个所有者修改了数据时
        // 会导致全部所有者持有的数据都发生了变化

        let msg = String::from("我很丑, 但我很温柔...");
        let x = Rc::new(RefCell::new(msg));

        let y = x.clone();
        let z = x.clone();

        x.borrow_mut().push_str("###");
        println!("y={}, z={}", y.borrow(), z.borrow());
    }

    // (5) Cell::from_mut新方法
    /*
       在 Rust 1.37 版本中新增了两个非常实用的方法:
       Cell::from_mut，该方法将 &mut T 转为 &Cell<T>
       Cell::as_slice_of_cells，该方法将 &Cell<[T]> 转为 &[Cell<T>]
    */

    // (6) 用一个例子来说明为什么我们需要使用 `RefCell` !!!
    // 由于Rust的"mutable"特性, 一个结构体中的字段"要么全都是immutable", "要么全部是 mutable"
    // "不支持针对部分字段进行设置"!!!
    // 比如在一个 struct 中, 可能只有个别的字段需要修改, 而其他字段并不需要修改
    // 为了一个字段而将整个 struct 变为 &mut 也是不合理的!
    // 所以实现 内部可变性 的 Cell 和 RefCell 正是为了解决诸如这类问题存在的
    // 通过它们可以实现 struct "部分字段可变", 而不用将整个 struct 设置为 mutable
    // 总结来说:
    // 我们可以不需要因为修改一个字段将整个struct都变成可变的, 这很不合理
    {
        // Example

        // 定义银行结构体
        struct Bank {
            // 使用RefCell存储余额, 因为余额是内部可变的, 而其他字段我们不变
            balance: RefCell<i32>, // 这个内部可变, 即使是&self的我们也可以使用
            name: String,          // 这个是内部不可变, &self情况下我们不可以直接使用
        }

        impl Bank {
            // 创建一个新的银行对象
            fn new() -> Bank {
                // 初始化余额为0
                Bank {
                    balance: RefCell::new(0),
                    name: "cn-bank".to_string(),
                }
            }

            // 存款
            fn deposit(&self, amount: i32) {
                // 获取内部可变引用
                let mut balance = self.balance.borrow_mut();
                // 修改余额
                *balance += amount;

                // 我们不可以执行下面代码, 因为整个struct都是不可变的
                // self.name.push_str("string");
            }

            // 取款
            fn withdraw(&self, amount: i32) -> bool {
                // 获取内部可变引用
                let mut balance = self.balance.borrow_mut();
                // 如果余额充足，则修改余额并返回true
                if *balance >= amount {
                    *balance -= amount;
                    true
                // 否则返回false
                } else {
                    false
                }
            }
        }

        // 创建一个新的银行
        let bank = Bank::new();
        // 存款100元
        bank.deposit(100);
        // 取款50元，余额应该是50元
        assert!(bank.withdraw(50));
        assert_eq!(*bank.balance.borrow(), 50);
    }
}
