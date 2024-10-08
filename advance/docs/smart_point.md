# 智能指针
在各个编程语言中, 指针的概念几乎都是相同的：**指针是一个包含了内存地址的变量，该内存地址引用或者指向了另外的数据。**

在 Rust 中，最常见的指针类型是引用，引用通过 `&` 符号表示。不同于其它语言，引用在 Rust 中被赋予了更深层次的含义，那就是：**可变或不可变的借用其它变量的值**. 引用本身很简单，除了指向某个值外并没有其它的功能，也不会造成性能上的额外损耗，因此是 Rust 中使用最多的指针类型。

> 在 rust 中, 单纯的指针负担起借用规则的使命

而智能指针则不然, 它虽然也号称指针，但是它是一个复杂的家伙：通过比引用更复杂的数据结构, 包含比引用更多的信息，例如元数据，当前长度，最大可用长度等。总之，Rust 的智能指针并不是独创，在 C++ 或者其他语言中也存在相似的概念。

> 在 rust 中, 所谓智能指针: **就是在引用的基础上, 提供了更多信息和功能**

Rust 标准库中定义的那些智能指针，虽重但强，可以提供比引用更多的功能特性，例如本章将讨论的引用计数智能指针。该智能指针允许你同时拥有同一个数据的多个所有权，它会跟踪每一个所有者并进行计数，当所有的所有者都归还后，该智能指针及指向的数据将自动被清理释放。

引用和智能指针的另一个不同在于前者仅仅是借用了数据, 而后者往往可以拥有它们指向的数据, 然后再为其它人提供服务。

> 注意关键点: **智能指针是同时 `具有引用和所有权` 的, 他拥有了他所指向的数据**

在之前的章节中，实际上我们已经见识过多种智能指针，例如动态字符串 String 和动态数组 Vec，它们的数据结构中不仅仅包含了指向底层数据的指针，还包含了当前长度、最大长度等信息，其中 String 智能指针还提供了一种担保信息：所有的数据都是合法的 UTF-8 格式。

智能指针往往是 `基于结构体实现` , 它与我们自定义的结构体最大的区别在于它实现了 `Deref` 和 `Drop` 特征：

* Deref 可以 **让智能指针像引用(rust可以自动调用 Deref 解引用)** 那样工作, 这样你就可以写出同时支持智能指针和引用的代码, 例如 *T
* Drop 允许你指定智能指针**超出作用域后自动执行**的代码, 例如做一些数据清除等收尾工作


1. Box<T>: 可以将值分配到堆上
2. Rc<T>: 引用计数类型, 允许多所有权存在
3. Ref<T> 和 RefMut<T>: 允许将借用规则检查从编译期移动到运行期进行


## Box 额外知识点

### Box 内存布局

先来看看 Vec<i32> 的内存布局:

```
(stack)    (heap)
┌──────┐   ┌───┐
│ vec1 │──→│ 1 │
└──────┘   ├───┤
           │ 2 │
           ├───┤
           │ 3 │
           ├───┤
           │ 4 │
           └───┘

```

之前提到过 Vec 和 String 都是智能指针, 从上图可以看出该**智能指针存储在栈**中, 然后**指向堆上的数组数据**

那如果数组中每个元素都是一个 Box 对象呢? 来看看 `Vec<Box<i32>>` 的内存布局：

```
                    (heap)
(stack)    (heap)   ┌───┐
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec2 │──→│B1 │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │B2 │───→│ 2 │
           ├───┤    └───┘
           │B3 │─┐  ┌───┐
           ├───┤ └─→│ 3 │
           │B4 │─┐  └───┘
           └───┘ │  ┌───┐
                 └─→│ 4 │
                    └───┘

```
上面的 B1 代表被 Box 分配到堆上的值 1。

可以看出**智能指针 vec2 依然是存储在栈上**, 然后**指针指向一个堆上的数组**，该数组中每个元素都是一个 Box 智能指针, 最终**Box 智能指针又指向了存储在堆上的实际值。** 其实就是: `vec => box => data`

### 总结

Box 就是用于将数据存放在堆上, **一切皆对象 = 一切皆 Box.**

我们可以看看 Box 的源码:

```rust
// 本质上就是一个元组结构体, 将数据存放在堆上
pub struct Box<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);
```

我们可以将所有权、借用规则与这些智能指针做一个对比：

｜Rust 规则	 ｜ 智能指针带来的额外规则 ｜

* 一个数据只有一个所有者 ｜ Rc/Arc让一个数据可以拥有多个所有者
* 要么多个不可变借用，要么一个可变借用 ｜ RefCell实现编译期可变、不可变引用共存
* 违背规则导致编译错误 ｜ 违背规则导致运行时panic

## RefCell

`RefCell` 具有内部可变性: **对一个不可变的值进行可变借用**

内部可变性的核心用法: 通过包裹一层 `RefCell`, 成功的让 &self 中的不可变成为一个可变值, 然后实现对其的修改