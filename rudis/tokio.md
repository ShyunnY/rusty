## Tokio 框架知识点

### Tokio中的任务

#### `'static` 约束

当使用 Tokio 创建一个任务时，该任务类型的生命周期必须是 `'static`。
意味着: 在任务中不能使用外部数据的引用, 或者只能引用 `'static` 生命周期变量。

原因在于：默认情况下，变量并不是通过 `move` 的方式转移进 `async` 语句块的， v 变量的所有权依然属于 `main` 函数。

> 拓展:
> * `&'static` 是一个具体的引用类型，表示**引用的生命周期是 `'static`**，适用于指向在整个程序运行期间有效的数据。
> * `T: 'static` 是一个**泛型约束,确保泛型类型 T 不包含任何短暂引用**,适用于需要确保数据持久性的情况

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // 我们必须添加 move 关键字, 将所有权转移进 Future 中
    // 如果我们想在多个线程中共享一个数据所有权, 需要使用 Arc
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}
```

#### `Send` 约束

`tokio::spawn` 生成的任务**必须实现 `Send` 特征**, 因为当这些任务在 `.await` 执行过程中发生阻塞时, Tokio 调度器会将任务在线程间移动。

> 当 `.await` 执行一个 FutureA 发生阻塞时, 调度器需要保存 FutureA 现场并切换到另外一个 FutureB 上执行. 
> 如果 FutureA 调用 `wake()` 通知执行器可以恢复执行时, 有可能调度器会将该 FutureA 派到另外一个线程上执行.
> 所以我们需要让 Future以及 Future内的所有数据都实现 `Send` 特征

**一个任务要实现 `Send` 特征, 那它在 `.await` 调用的过程中所持有的全部数据都必须实现 `Send` 特征。**

当 `.await` 调用发生阻塞时, 任务会让出当前线程所有权给调度器, 然后当任务准备好后, 调度器会从上一次暂停的位置继续执行该任务. 该流程能正确的工作，任务必须将 `.await` 之后**使用的所有状态保存起来, 这样才能在中断后恢复现场并继续执行。**
若这些状态实现了 `Send` 特征(可以在线程间安全地移动)，那任务自然也就可以在线程间安全地移动

```rust
use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // 语句块的使用强制了 `rc` 会在 `.await` 被调用前就被释放
        // 因此 `rc` 并不会影响 `.await`的安全性
        // ** 我们主要看是否跨过了 `.await` 使用
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` 的作用范围已经失效, 因此当任务让出所有权给当前线程时, 它无需作为状态被保存起来
        yield_now().await;
    });
}
```

其实跟自引用很类似, 在一个 Future 中如果 `跨 .await` 执行将需要考虑更多事情(因为这代表不一定在一次 `poll()` 就能执行完毕, 就可能会发生线程切换执行问题)。

### 惰性的 Async

`async` 操作在 Tokio 中是惰性的:

```rust
loop {
    // 我们没有显式调用 .await 执行它, 所以并不会执行该任务
    async_op();
}
```

当我们显示使用 `.await` 时, 此时会执行该 Future 。但是也会等待该 Future 执行完毕才会进行下一轮循环继续执行下一个 Future。

```rust
loop {
    // 当前 async_op 完成后, 才会开始下一次循环执行 Future
    async_op().await;
}
```

**原因如下:**

在 Tokio 中, `fn main()`函数其实是**一个巨大的 Future** 。在 Rust 的异步编程中，在一个 Future 调用 `.await` 时**会阻塞等待该 Future 完成(父Future => 子Future)**。 

除非我们在该 Future 中同时运行多个 子Future(父Future => 多个子Future), 类似于我们使用 `tokio::spawn` 启动一个子Future。 如果在该子Future中调用 `.await` 被阻塞了，那么就会**切换调度运行**另外一个 `tokio::spawn` 启动的的子Future
