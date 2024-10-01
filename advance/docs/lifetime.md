在 Rust 中, **闭包（closures）和函数指针（function pointers）在处理引用时有着不同的生命周期要求**. Rust 的类型系统和借用检查器（borrow checker）需要确保所有的引用都是有效的, 这意味着引用必须在它们被使用时仍然有效**(引用的数据类型必须在引用期间始终有效)**.

当闭包捕获变量时, 它可以以不同的方式捕获变量：

* 通过引用 (&T): 闭包捕获了变量的引用, 因此它必须**保证在闭包的生命周期内, 被引用的变量是有效的**
* 通过可变引用 (&mut T): 闭包捕获了变量的可变引用, 同样必须**保证在闭包的生命周期内被引用的变量是有效的**
* 通过值 (T): 闭包通过**移动（move）**的方式捕获变量, 这意味着变量的**所有权被转移给了闭包**

对于函数指针, 它们的生命周期是**显式指定(通过函数签名中的 `'a,'b` 显示指定)**的, 因此编译器可以很容易地验证引用的有效性.

然而, 闭包的生命周期是隐式的, 编译器需要**根据闭包捕获的变量来推断生命周期**.

为了处理这种情况, Rust 引入了 `Fn` 、`FnMut` 和 `FnOnce` 这三个 trait, 它们分别对应闭包的三种捕获方式:
* Fn：闭包通过不可变引用来捕获变量。
* FnMut：闭包通过可变引用来捕获变量。
* FnOnce：闭包通过值来捕获变量。

当你尝试将一个闭包赋值给一个变量或传递给函数时, ***Rust 需要知道闭包的生命周期***. 

为了满足这个需求, Rust 编译器会根据闭包捕获的变量类型和方式, 自动为闭包实现相应的 Fn trait

这样, 编译器就可以**确保闭包的生命周期与它捕获的变量的生命周期相匹配**
例如, 考虑以下闭包：
```rust
let closure = |x: &i32| -> &i32 {
    x
};
```

这个闭包尝试捕获一个不可变引用 x 并返回它。然而，由于闭包的生命周期是隐式的，编译器无法保证在闭包的生命周期内，x 的引用是有效的。因此，这个闭包不能直接被编译。

> 其实就是说: 当一个闭包赋值给了变量, 同时这个闭包捕获了外部引用. 假设使用者偷摸着把闭包变量藏起来了, 等闭包内
> 捕获的引用销毁了再使用, 岂不是达成了 "悬垂引用" 的目的? (偷摸着干坏事)

为了解决这个问题, 你可以使用 move 关键字来强制闭包通过值来捕获变量, 从而转移所有权：

```rust
let x = 5;
let closure = move |x: i32| -> &i32 {
    &x
};
```

在这个例子中，x 的所有权被移动到了闭包中，因此闭包拥有 x 的生命周期。现在，这个闭包可以被编译，因为它满足了 Rust 的借用规则。
如果你希望闭包捕获引用而不是移动变量，你需要使用显式的生命周期注解来告诉编译器闭包的生命周期与某个特定的生命周期绑定：

```rust
Copy
fn foo<'a>(x: &'a i32) -> impl Fn() -> &'a i32 {
    move || x
}
```

在这个例子中，foo 函数返回一个闭包，该闭包捕获了参数 x 的引用，并返回一个指向 x 的引用。通过为函数参数 x 和返回类型添加生命周期注解 'a，我们告诉编译器闭包的生命周期与 x 的生命周期相同。这样，编译器就可以确保在闭包的生命周期内，返回的引用是有效的。