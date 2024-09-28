pub fn _entry() {}

fn _module() {}

fn _package_and_crate() {
    // 什么是Package(项目)？什么又是Crate(包)
    // * 项目(Package): 可以用来构建、测试和分享包
    // * 工作空间(WorkSpace): 对于大型项目，可以进一步将多个包(Crate)联合在一起，组织成工作空间
    // * 包(Crate): 一个由"多个模块"组成的树形结构, 可以作为三方库进行分发, 也可以生成可执行文件进行运行
    // * 模块(Module): 可以一个文件多个模块, 也可以一个文件一个模块, 模块可以被认为是真实项目中的代码组织单元

    //
    // 进一步理解:
    // 1. 工作空间 workspace: 集合多个 package 的管理概念
    // 2. 包 package: package 管理一个到多个carte，也只是一个管理概念
    // 3. 单元包 crate: 单元包 crate 真实组织代码和代码关系的单元
    // 4. 模块 mod: 单元包内代码的载体, 由单个文件或者一个带mod.rs文件的目录构成

    // 以下是注意事项:
    // (1). 一个 Package 只能包含一个库(library)类型的单元包Crate, 但是可以包含多个二进制可执行类型的单元包Crate
    //
    // (2). 一个工作区下可以组织并管理多个 Package 包
    //
    // (3). `src/main.rs`是二进制包的根文件, 该二进制包的包名跟所属 Package 相同
    //
    // (4). `src/lib.rs`是一个库类型的单元包入口, 只能作为三方库被其它项目引用，而不能独立运行
    //
    // (5). 我们需要牢记: Package 是一个项目工程, 而d单元包Crate只是一个编译单元

    // (6). 一个典型的rustPackage如下
    //     .
    // ├── Cargo.toml
    // ├── Cargo.lock
    // ├── src          // **实际上也代表了主crate**
    // │   ├── main.rs  // 默认的二进制单元包(编译后生成与package同名的binary)
    // │   ├── lib.rs   // 一个package只能拥有的唯一lib.rs库单元包
    // │   └── bin      // 其余的二进制单元包(编译成与文件同名的binary)
    // │       └── main1.rs
    // │       └── main2.rs
    // ├── tests        // 集成测试文件
    // │   └── some_integration_tests.rs
    // ├── benches      // 基准性能测试
    // │   └── simple_bench.rs
    // └── examples     // 项目example实例
    //     └── simple_example.rs
}
