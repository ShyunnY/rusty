## Pin 的概念

我先来从宏观层面解读一下。`Pin`是一个这样的智能指针: **他内部包裹了另外一个指针P，并且只要P指针指向的内容（我们称为T）没有实现Unpin，则可以保证T永远不会被移动（move）。**

`Pin`这个单词也很形象的表示Pin就像钉子一样可以把T钉住。所以Pin一般来说用`Pin<P<T>>`这种方式表示（P是Pointer的缩写，T是Type的缩写）。这个定义初看有点绕，我们来划几个重点：

* Pin自身是一个智能指针。 为什么呢？因为他impl了 **Deref** 和 **DerefMut**
* Pin包裹的内容**只能是指针**，不能是其他普通类型。比如Pin<u32>就没有意义。
* Pin具有“钉住”T不能移动的功能，这个功能是否生效取决于T是否impl Unpin。简单的说，**如果T实现了Unpin，Pin的“钉住”功能完全失效了，这时候的Pin<P<T>>就等价于P<T>**
* Unpin是一个auto trait，编译器默认会给所有类型实现Unpin。唯独有几个例外，他们实现的是!Unpin。这几个例外是: PhantomPinned，编译器为async/await desugar之后生成的impl Future的结构体。
( 所以Pin<P<T>>默认情况下的“钉住”功能是不生效的，只针对上面说的这几个impl !Unpin的情况生效。

我们可以看看官方的 Pin 部分源码:

```rust
#[stable(feature = "pin", since = "1.33.0")]
#[lang = "pin"]
#[fundamental]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Pin<P> {
    pointer: P,
}

// 默认就实现了 Deref , 可以看出 Pin 本质上也是个智能指针
#[stable(feature = "pin", since = "1.33.0")]
impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        Pin::get_ref(Pin::as_ref(self))
    }
}

// DerefMut只能在 Target 类型实现了 Unpin(不固定) 才能使用
#[stable(feature = "pin", since = "1.33.0")]
impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
}
```

## 为什么需要 Pin ？

在自引用结构体下, 如果我们获取了其可变引用就可能会导致其发生 Move, 从而使得
结构体内指针失效.

**只要我们想办法让Safe Rust下不暴露可变借用即可！**

作为一个结构体, 它自身**没办法限制自己不能可变借用**, 因为我们直接用 `&mut Test{...}` 就可以轻松拿到。那从标准库中去找找，`Box<T>` 呢？先不考虑它性能问题，我们把结构体T包裹在Box中，看Box能不能保证不暴露 `&mut T` 出去。看一下API文档，很遗憾不能。`Box::leak()` 返回值就是 `&mut T`，更甚者 `Box impl DerefMut`，就算不用leak()我们也可以通过 `* &mut Box<T>` 解引用轻松拿到 `&mut T！`

不用找了, 在Pin之前的标准库中确实没有这样的API能够防止在Safe Rust下不暴露&mut T。

所以，接下来是该Pin登场的时候了！

## Pin 登场

我们找到了问题的根源在哪, Pin就是从根源上解决这个问题的。现在我们很清晰了，是不是可以用一句话概括：**`Pin就是一个不会让你在 Safe Rust 暴露可变借用&mut的智能指针?`**

答案是：不全正确。这就是Pin概念起初让大家一脸懵逼的地方。

下面让Pin自己来解答大家的疑惑，Pin说：“你们不是想让我保证被我包裹的指针 P<T> 永远钉住不让 move 吗？我可以答应，但我有一个原则。那就是**我永远不能钉住持有通行证的朋友，这张通行证就是Unpin**。如果没有这张通行证，请放心，我会把你钉得死死的！”

我们再进一步概括: **`Pin就是一个不会让你在 Safe Rust 暴露可变借用&mut的智能指针, 必须建立在 Pin 中的类型是实现了 !Unpin`**

举个例子: 比如我是Pin，你是 `P<T>`，如果你**impl了 `Unpin`** ，我会提供两种办法让你**在 Safe Rust下拿到 `&mut T`**

* 第一种，使用：`Pin::get_mut()`
  ```rust
  impl<'a, T: ?Sized> Pin<&'a mut T> {
    // 我们可以看见, 使用 get_mut的前提就是 T 需要是 Unpin 的 !
    pub fn get_mut(self) -> &'a mut T 
    where T: Unpin {
        self.pointer
    }
  }
  ```
* 第二种，我 impl 了 `DerefMut`，你可以**解引用**拿到 `&mut T`
  ```rust
  impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    // P 需要也实现了 DerefMut trait, 并且 P 指向的类型 T 是 Unpin 的!
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
  }
  ```

在 rust 中, 默认给你们所有类型发了通行证（都实现了Unpin）

但是 rust 留了一个叫 `PhantomPinned` 的小伙伴。别看他名字很奇怪，他可是我很喜欢的得力助手！**因为他实现的是 `!Unpin` ！**

`PhantomPinned` 其实就是一个用于标记的结构体, 在我们的结构体中引入之后, 就代表我们实现了 `!Unpin`：

```rust
use std::marker::PhantomPinned;

struct Test {
   a: String,
   b: *const String,
   // 这代表我们的 Test 结构体实现了 !Unpin 了, 此时不可以在 safe 下移动啦！
   _marker: PhantomPinned,
}
```

**使用了 `PhantomPinned` 就会保证你没办法在 Safe Rust 下拿到可变借用 `&mut T`**（不信你去翻翻我的API）, 拿不到 `&mut T` 你就没办法作用到std::mem::swap()上，也就是说**你被我钉住了**！

当然我还是提供了一个 `unsafe` 的 `get_unchecked_mut()`. **不管你有没有实现Unpin，你都可以通过调用这个方法拿到 `&mut T`**，但是你需要遵守Pin的契约（参考下面），否则出了什么问题后果自负！

> 对于 `Pin<P<T>>`:
> * 如果 `P<T>` 符合 `Unpin`，那P<T>从被Pin包裹到被销毁，都要一直保证 P<T> **不被钉住**
> * 如果 `P<T>` 符合 `!Unpin`，那P<T>从被Pin包裹到被销毁，都要一直保证 P<T> **被钉住**

```rust
impl<'a, T: ?Sized> Pin<&'a mut T> {
    // 这是一个 unsafe 方法, 需要在 unsafe 下使用
    pub unsafe fn get_unchecked_mut(self) -> &'a mut T {
        self.pointer
    }
}
```

我们再用一句话来总结: **`如果你实现了Unpin，Pin可以让你在 Safe Rust 下拿到 &mut T，否则会把你在 Safe Rust 下钉住（也就是拿不到 &mut T ）`**

## 如何构造一个 Pin ?

首先我们要梳理清楚怎样把P<T>用Pin包裹起来，也就是怎样构造一个Pin。查看文档会发现主要有这几种方式

### `Pin::new()`

如果你的 P 指向的 T 是 `Unpin` 的话，你可以安全的调用 `Pin::new()` 构造一个Pin. 可以看到它底层实际上是调用 `unsafe` 的 `Pin::new_unchecked()`，之所以`Pin::new()`是安全的，是因为 `Unpin` 的情况下Pin的 **”钉住“ 效果是不起作用的，跟正常的指针一样了**

```rust
impl<P: Deref<Target: Unpin>> Pin<P> {
    pub fn new(pointer: P) -> Pin<P> {
        // Safety: the value pointed to is `Unpin`, and so has no requirements
        // around pinning.
        unsafe { Pin::new_unchecked(pointer) }
    }
}
```

### `Pin::new_unchecked()`

这个方法很简单, 但它是 `unsafe` 的. 标为 `unsafe` 的原因是编译器没办法保证使用者后续操作一定遵守Pin的契约。

只要有存在违反契约的可能性, 就必须用 `unsafe` 标记，因为这是使用者的问题，编译器没办法保证。

如果使用者通过 `Pin::new_unchecked()` 构造一个 `Pin<P<T>>` 之后Pin的生命周期结束了，但P<T>依然存在，则后续操作**依然可能被move**，造成内存不安全

也就是说: `Pin::new_unchecked()` 代表了需要由用户保证其遵守 Pin 的契约

```rust
impl<P: Deref> Pin<P> {
    pub unsafe fn new_unchecked(pointer: P) -> Pin<P> {
        Pin { pointer }
    }
}
```

### 其他

包括 `Box::pin()`, `Rc::pin()` 和 `Arc::pin()` 等，底层都是调用上面的`Pin::new_unchecked()`


## 异步编程中的Future

接下来讲一下Pin目前最重要的一个应用：`Future`。当初2018年官方异步组引入Pin API的初衷就是为了解决 **Future内部自引用的问题**。

当我们跨 `.await` 使用引用时, 就会产生 future 内部自引用:

```rust
fn demo() {
    async fn foo() {
        println!("hello,foo");
    }

    // 跨 await 使用引用
    async {
        let data = String::from("bar");
        let data_ref = &data;   // 这里发生了借用
        foo().await;
        println!("borrowed: {}", data_ref); // 这里使用了借用
    };
}
```

由于 Rust 的 future 是通过不断地被 poll 从而驱动状态机的状态流转的，foo 函数返回的这个 self-referential future 如果会被移动（比如在赋值、传参、数组扩缩容等情况下），那么这个 self-referential 的数据结构就会被破坏，从而导致异常。

Rust 的 future 的特性：

* Future **刚刚创建出来的时候，移动它是安全的：因为没有被 poll 过，self-referential 的结构还没有形成**
* Future **第一次被 poll 之后，移动这个 future 就不再是是安全的：因为跨越不同的 yield point 之间，一定会导致生成嵌套的 self-refenrential future**

我们来看看 Future 的签名:

```rust
pub trait Future {
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

可以看到: poll 方法的 self 的类型是：`Pin<&mut Self>`，这就意味着，调用一次 poll 之后，**self 的所有权就 somehow 被这个 Pin 获取了**，持有 future 对象的代码 **`无法再通过任何办法获取到 future 的所有权，也就无法移动这个 future`**


```rust
impl<P: Deref<Target: Unpin>> Pin<P> {
//             ^~~~~ Deref 的目标类型必须是 Unpin   
}
```

可以看到, Pin 所实现的 Deref trait **是有类型限制的**：它要求 pin 里面的指针的目标类型必须实现 `Unpin Trait`. **如果指针的目标类型存在 self-referential 的情况，那么它是不能被 `deref_mut` 的，也就无法从 Pin 中取出原始的值从而任意移动**

* `pin.deref();`    // 无论实现了 Unpin 与否, 都可以进行调用
* `pin.deref_mut();`    // 只有实现了 Unpin 才能调用

**`Pin<&mut Self> 就是为了确保 Future 对象自己不会被移动`**