use core::time;
use lazy_static::lazy_static;
use std::{
    cell::OnceCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicI32, AtomicUsize, Ordering},
        Arc, OnceLock,
    },
    thread,
};

#[allow(dead_code)]
pub fn hello() {
    /*
    在一些场景, 我们可能"需要全局变量来简化状态共享的代码": 包括全局 ID,全局数据存储等等
    首先有一点可以肯定, "全局变量的生命周期肯定是'static", 但是不代表它需要用static来声明
    例如常量、字符串字面值等无需使用static进行声明, 原因是它们已经"被打包到二进制可执行文件中"(与程序共存亡)

    程序变量存在 "编译期初始化" 及 "运行期初始化"
    */

    // (1) 编译期初始化
    // 我们大多数使用的全局变量都只需要在编译期初始化即可, 例如静态配置、计数器、状态值等等
    //
    // 1. 静态常量
    // "全局常量可以在程序任何一部分使用",  如果它是定义在某个模块中, 你需要引入对应的模块才能使用
    // 常量: 顾名思义它"是不可变的", 很适合用作静态配置
    // 我们需要注意以下几点不同
    /*
        常量与普通变量的区别:
        1.关键字是 "const" 而不是 let
        2.定义常量 "必须指明类型"（如 i32）不能省略
        3.定义常量时变量的命名规则"一般是全部大写"
        4.常量可以在任意作用域进行定义, 其"生命周期贯穿整个程序的生命周期"
          编译时编译器会尽可能将其"内联到代码中", 所以 **"在不同地方对同一常量的引用并不能保证引用到相同的内存地址"**
        5.常量的赋值只能是"常量表达式/数学表达式", 也就是说"必须是在编译期就能计算出的值"!
          如果需要在运行时才能得出结果的值比如函数, 则不能赋值给常量表达式(常量就是编译期可以确定下来的值)
        6.对于变量出现"重复的定义(绑定)会发生变量遮盖", 后面定义的变量会遮住前面定义的变量, "常量则不允许出现重复的定义"(无法重复定义)
    */
    {
        const MAX_ID: usize = usize::MAX / 2;
        println!("1.1 最大ID值为: {}", MAX_ID);

        // 常量是不能调用函数的, 以下代码将会报错
        // fn dd() -> i32 {
        //     1
        // }
        // const VAL: i32 = dd();
    }
    //
    // 2. 静态变量
    // 静态变量允许声明一个全局的变量, 常用于全局数据统计
    /*
      Rust要求必须使用 "unsafe" 语句块才能修改static变量, 因为这种使用方式往往并不安全(也就是说: 如果是mut的必须要用unsafe)
      其实编译器是对的: "当在多线程中同时去修改时", 会不可避免的遇到脏数据(或者加锁？但是好像性能不高)
      只有在同一线程内或者不在乎数据的准确性时, 才应该使用全局静态变量

      和常量相同, 定义静态变量的时候必须赋值为"在编译期就可以计算出的值"(常量表达式/数学表达式)
      不能是运行时才能计算出的值(如函数)

      静态变量和常量的区别:

      1.静态变量不会被内联, 在整个程序中: 静态变量"只有一个实例", "所有的引用都会指向同一个地址"(常量可能会被内联, 地址可能不会一致)
      2.存储在静态变量中的值必须"要实现 Sync trait", "因为多个线程都可以共享其引用"
    */
    {
        static mut COUNT: i32 = 0;

        for _ in 1..=100 {
            thread::spawn(|| unsafe {
                COUNT += 1;
            });
        }

        thread::sleep(time::Duration::from_millis(30));
        unsafe {
            println!("2.2 静态变量 count = {}", COUNT);
        }
    }
    //
    // 3. 原子类型
    // 想要全局计数器、状态控制等功能, 又想要"线程安全"的实现, 原子类型是非常好的办法
    // 所以有没有一种可能, 我们可以将 静态变量 + 原子类型 结合使用~
    {
        let counter = Arc::new(AtomicI32::new(0));

        for _ in 1..=10 {
            let counter = counter.clone();
            thread::spawn(move || {
                counter.fetch_add(1, Ordering::Relaxed);
            });
        }

        thread::sleep(time::Duration::from_millis(30));
        println!("2.3 原子类型 count = {}", counter.load(Ordering::Relaxed));
    }
    //
    // 4. Example: 全局ID生成器
    {
        const MAX_ID: usize = usize::MAX / 2;
        static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct UUID(usize);

        impl UUID {
            fn new() -> Self {
                let c = GLOBAL_COUNT.load(Ordering::Acquire);
                if c == MAX_ID {
                    panic!("uuid is overflowed!")
                }
                GLOBAL_COUNT.fetch_add(1, Ordering::AcqRel);

                UUID(c + 1)
            }
        }

        impl std::fmt::Display for UUID {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "uuid = {}", self.0)
            }
        }

        let uid_1 = UUID::new();
        println!("{}", uid_1);
        let uid_2 = UUID::new();
        println!("{}", uid_2);
        let uid_3 = UUID::new();
        println!("{}", uid_3);
    }

    // (2) 运行期初始化
    // 静态初始化有一个致命的问题: 无法用函数进行静态初始化, 并且只能使用在编译期可以计算出的值
    // 但是我们在真实开发中, 无法避免需要使用运行期的值来做初始化
    // {
    //     // 例如, 无法将 string 作为静态初始化
    //     static mut LOCK: String = String::from("11");

    //     unsafe {
    //         LOCK = String::from("1");
    //     }
    // }
    //
    // 1.lazy_static
    // lazy_static是社区提供的非常强大的宏, 用于"懒初始化静态变量"
    // 之前的静态变量都是"在编译期初始化的", 因此无法使用函数调用进行赋值, 而lazy_static允许我们"在运行期初始化静态变量"！
    // lazy_static宏匹配的是"static ref", 所以定义的"静态变量都是不可变引用"(只读引用)
    //
    // 我们为什么会需要在运行期动态加载一个静态变量？
    // 一个全局的动态配置, 它"在程序开始后才加载数据进行初始化", 最终可以让各个线程直接访问使用(也就是在运行时加载)
    {
        lazy_static! {
            // 匹配的是不可变引用
            #[derive(Debug)]
            static ref MAP: HashMap<String, String> = {
                println!("2");
                let mut m = HashMap::new();
                m.insert(String::from("1"), String::from("z3"));
                m.insert(String::from("2"), String::from("l4"));
                m
            };
        }
        // 输出顺序: 1 -> 2 -> z3 -> l4 (由此可见这是懒加载的, 我们第一次使用时才会加载, 后续将不会加载)

        println!("1");
        println!("2.1 lazy_map: {}", MAP.get(&String::from("1")).unwrap());
        println!("2.1 lazy_map: {}", MAP.get(&String::from("2")).unwrap());
    }
    //
    // 2. Box::leak 主动泄漏内存
    // Rust 的借用和生命周期规则限制了我们做到这一点, 因为试图"将一个局部生命周期的变量赋值给全局生命周期"的CONFIG, 这明显是不安全的
    // 将一个变量从内存中泄漏(听上去怪怪的，竟然做主动内存泄漏), 然后将其变为 'static 生命周期
    // 最终该变量将和程序活得一样久, 因此可以赋值给全局静态变量
    // 主动将内存进行泄漏, 让其生命周期变成static的!
    {
        #[derive(Debug)]
        struct Config {
            name: String,
            age: i32,
        }

        static mut CONFIG: Option<&mut Config> = None;

        // load Config
        unsafe {
            let config = Box::new(Config {
                name: String::from("spring"),
                age: 20,
            });
            CONFIG = Some(Box::leak(config));

            println!("2.2 config: {:?}", CONFIG);
        }
    }
    // 3. 从函数中返回全局变量
    // 其实本质也一样的（生命周期问题）
    {
        #[derive(Debug)]
        struct Config {
            name: String,
            age: i32,
        }

        static mut CONFIG: Option<&mut Config> = None;

        impl Config {
            fn init() -> Option<&'static mut Self> {
                Some(Box::leak(Box::new(Config {
                    name: String::from("spring"),
                    age: 20,
                })))
            }
        }

        // load Config
        unsafe {
            CONFIG = Config::init();

            println!("2.3 config: {:?}", CONFIG);
        }
    }
    // 4.OnceCell和OnceLock
    // 用于你在"第一次访问时初始化"一个值. 它可以存储一个值, 该值在初始化后不能再被更改
    // OnceCell: 单线程下使用
    // OnceLock: 多线程下使用
    {
        // OnceLock example
        static LOGGER: OnceLock<Logger> = OnceLock::new();

        struct Logger;
        unsafe impl Sync for Logger {}

        impl Logger {
            fn init() -> &'static Self {
                LOGGER.get_or_init(|| {
                    println!("2.4 global logger init...");
                    Self
                })
            }
        }

        // 只会初始化一次
        Logger::init();
        Logger::init();
        Logger::init();
    }
    {
        static mut CONFIG: OnceCell<Op> = OnceCell::new();

        struct Op;

        impl Op {
            fn new() -> &'static Self {
                unsafe {
                    CONFIG.get_or_init(|| {
                        println!("2.4 init op...");
                        Op
                    })
                }
            }
        }

        Op::new();
        Op::new();
        Op::new();
    }
    // 总结
    /*
       简单来说全局变量可以分为两种:
       1."编译期初始化的全局变量": const创建常量, static创建静态变量, Atomic创建原子类型
       2."运行期初始化的全局变量": lazy_static用于懒初始化, Box::leak利用内存泄漏将一个变量的生命周期变为'static
    */
}
