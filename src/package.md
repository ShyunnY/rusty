# Rust中关于包的总结

在很多项目中, 我们不可能将所有代码写在一个文件中. 此时我们就需要通过多个文件和文件夹来组织代码.

在rust中, 我们一般使用 `mod` + `use` 来组织模块关系:

* `mod` 关键字: 用于**加载一个新的模块到当前模块下**中(也可以理解为将新模块作为当前模块的儿子)
* `use` 关键字: 用于**使用加载完成的模块中的资源**(针对某个模块使用 `use` 前*一定需要将模块通过 `mod` 加载到当前作用域中*)

> 我们在使用 `mod` 加载模块时, 也需要考虑模块之间的关系.
> 假设在 `people/` 文件夹下存在多个 `.rs`文件: a,b,c,d. 此时我们在 `people/mod.rs` 文件中加载一个新的模块 `pub mod e`, 那么这个 e 相对其他四个模块就是兄弟模块.

## 文件形式的模块

当我们使用单文件作为模块时, mod 的名称与文件名相同, 我们只需要在 `main.rs` 中使用 `mod <moduleName>`加载模块即可

当我们使用希望一个文件夹及其下面的文件作为模块时, 我们需要在文件夹平级提供 **文件夹同名.rs** 或者在文件夹下提供 **mod.rs** 文件.
例如: 当前有一个目录 `instrutment`, 在其下存在两个文件 `tracing.rs/metrics.rs`. 

此时在该文件夹中的模块顺序就是: 

```shell
instructment
    |-- tracing
    |-- metrics
```

如果此时我们在 main.rs 中希望使用 tracing/metrics 的模块, 那么我们需要在 instrutment 文件夹下提供一个 `mod.rs` 文件. 该文件将通过 `pub mod` **将子模块加载到当前模块中**.

```rust
# == instrutment.rs ==
# 将 tracing 和 metrics 模块加载到 instructment 模块下
pub mod tracing;
pub mod metrics;
```

此时我们在 main 中, 只需要将 instructment 模块引入到当前作用域即可:
```rust
# == main.rs ==
# 我们仅导入instructment, 但是背后会级连导入 instrument 下的 tracing 和 metrics 模块.
# 因为这两个模块已经加载进 instrutment 作为其的子模块, 同时他两个还是 "pub" 的
mod instructment;
```