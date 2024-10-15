use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, File},
    io,
};

use anyhow::{Context, Result};

#[allow(dead_code)]
pub fn hello() {
    // 高级错误处理

    // (1) 组合器模式
    one();
    // (2) 自定义错误（成为Error类型, 实现From转换Error类型）
    two();
    // (3) 归一化不同的错误类型
    three();
}

#[allow(dead_code)]
fn one() {
    // 在Rust中,组合器更多的是用于对返回结果的类型进行变换: 例如使用 ok_or 将一个 Option 类型转换成 Result 类型

    // 1. or(), and()
    /*
        跟布尔关系的与/或很像, 这两个方法会对两个表达式做逻辑组合最终返回Option / Result
        * or(): 表达式"按照顺序求值", 若任何一个表达式的结果是 Some 或 Ok, "则该值会立刻返回(其实就是立刻返回左值)"
        * and(): 若两个表达式的结果都是 Some 或 Ok, 则"第二个表达式中的值被返回". 若任何一个的结果是 None 或 Err, 则立刻返回
        * xor(): 仅作用在Option上的异或操作

        其实我们将其理解为条件判断中即可:  condition1 && condition2, condition1 || condition2
    */
    {
        // Example: Option的or()和and()

        // 两个都是Some, "返回第二个表达式"
        assert_eq!(Some(10).and(Some(20)), Some(20));
        // 有一个是None, 返回None
        assert_eq!(Some(10).and(Option::<i32>::None), None);
        // 有一个是Some即可
        assert_eq!(Some(10).or(Option::None), Some(10));

        // Example: Result的or()和and()
        let o1: Result<&str, &str> = Ok("ok1");
        let o2: Result<&str, &str> = Ok("ok2");
        let e1: Result<&str, &str> = Err("error1");
        let e2: Result<&str, &str> = Err("error2");

        // 立刻返回左值
        assert_eq!(o1.or(o2), o1);
        assert_eq!(o1.or(e1), o1);
        assert_eq!(e1.and(e2), e1);
        assert_eq!(o1.and(e2), e2);
    }

    // 2. or_else() 和 and_then()
    /*
        or_else()和and_then()与 or(),and()类似, 只是第二个参数是个闭包

        "and_then的闭包是有参数的: 如果第一个类型返回Some/OK, 则作为闭包的参数!!!"
    */
    {
        let s1 = Some("some1");
        let s2 = Some("some2");
        let n: Option<&str> = None;

        assert_eq!(s1.or_else(|| s2), s1);
        assert_eq!(s1.or_else(|| n), s1);
        assert_eq!(s1.and_then(|_| { s2 }), s2);
        assert_eq!(s1.and_then(|_| n), n);
    }

    // 3. filter
    /*
        filter其实就是作为过滤使用的
    */
    {
        let s1 = Some(10);
        let n: Option<i32> = None;
        assert_eq!(s1.filter(|v| v % 2 == 0), s1);
        // 因为n=None没有值, 所以结果也没有值
        assert_eq!(n.filter(|v| v % 2 == 0), n);
    }

    // 4. map() 和 map_err()
    /*
        1.处理Some或者OK: map() 可以将Some或Ok"中的value值映射为另一个":  例如Some(10) => Some("hello")
        2.处理Err: map_err() 可以将Err"中的value值映射为另一个":  例如Err("404") => Err(404)
    */
    {
        let s1 = Some(10);
        let s2 = Some(String::from("10"));
        let s3: Result<i32, &str> = Ok(10);
        let s4: Result<String, &str> = Ok(String::from("10"));

        assert_eq!(s1.map(|s| s.to_string()), s2);
        assert_eq!(s3.map(|s| s.to_string()), s4);
    }

    // 5. map_or() 和 map_or_else()
    /*
        这两兄弟跟 map()类似
        1. map_or(): 如果前一个值是None/Err, 那么使用默认值
        2. map_or_else(): 如果前一个值是None/Err, 那么使用默认闭包值
    */
    {
        let s1 = Some(10);
        assert_eq!(s1.map_or(99, |v| v), 10);
        assert_eq!(None.map_or(99, |v| v), 99);

        let o1 = Ok(10);
        assert_eq!(o1.map_or_else(|_: i32| 1, |v| v), 10);
    }

    // 6. ok_or() and ok_or_else()
    // "这两兄弟可以将 Option 类型转换为 Result 类型:" Option => Result
    // 其中 ok_or "接收一个默认的 Err 参数"
    // 我发现了!带一个else一般都是存在闭包函数的
    {
        let s = Some(10);
        let n: Option<()> = None;
        assert_eq!(s.ok_or("error"), Ok(10));
        assert_eq!(n.ok_or("error"), Err("error"));

        assert_eq!(s.ok_or_else(|| "err"), Ok(10));
        assert_eq!(n.ok_or_else(|| "err"), Err("err"));
    }
}

#[allow(dead_code)]
fn two() {
    /*
    虽然标准库定义了大量的错误类型, 但是一个严谨的项目光使用这些错误类型往往是不够的
    例如我们可能会自定义一个错误类型暴露给用户

    为了帮助我们更好的定义错误, Rust在标准库中提供了一些可复用的特征, 例如 std::error::Error 特征
    当"自定义类型实现该特征后, 该类型就可以作为 Err 来使用"

    use std::fmt::{Debug, Display};
    pub trait Error: Debug + Display {
        fn source(&self) -> Option<&(Error + 'static)> { ... }
    }

    实际上, 自定义错误类型只需要实现 Debug 和 Display 特征即可
    source 方法是可选的(因为已经存在了默认的实现)! 而 Debug 特征往往也无需手动实现, 可以直接通过 derive 来派生
    */
    {
        // 定义了一个错误类型AnyErr
        // 当为它派生了 Debug 特征, 同时手动实现了 Display 特征后, 该错误类型就可以"作为Err来使用了"
        // 事实上实现 Debug 和 Display 特征并不是作为 Err 使用的必要条件, 即使我们移除实现也可以使用

        #[derive(Debug)]
        struct AnyErr {
            code: u16,
            page: &'static str,
        }

        impl Display for AnyErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "简单的自定义错误: {{ code = {}, page = {} }}",
                    self.code, self.page
                )
            }
        }

        fn produce_err() -> Result<(), AnyErr> {
            Err(AnyErr {
                code: 404,
                page: "/user/list.html",
            })
        }

        match produce_err() {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
    //
    // 通过 From trait 将错误进行转换
    // 在rust中如果需要将一个类型进行隐式转换, 需要该类型实现 From trait, rust编译期会自动调用from方法进行类型隐式转换
    // 可以看到From特征就是将一个类型转换成另外一个
    // pub trait From<T>: Sized {
    //     /// Converts to this type from the input type.
    //     fn from(value: T) -> Self;
    // }
    //
    // 事实上该特征在之前的 ? 操作符章节中就有所介绍
    // 大家都使用过 String::from 函数吧？它可以通过 &str 来创建一个 String, 其实该函数就是 From 特征提供的
    //
    // 实际上我们可以实现From特征, 让其他类型转换为自定义Err
    {
        #[derive(Debug)]
        struct AnyErr {
            kind: String,
            msg: String,
        }

        impl Display for AnyErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(
                    f,
                    "实现了From的自定义错误(支持转换): {{ kind = {}, errMsg = {} }}",
                    self.kind, self.msg,
                )
            }
        }

        // 以下代码中除了实现 From 外, 还有一点特别重要: "那就是 ? 可以将错误进行隐式的强制转换"
        // File::open 返回的是 std::io::Error, 我们并没有进行任何显式的转换, 它就能自动变成 AnyError
        // "这就是 ? 的强大之处！(支持隐式调用From进行转换)", 在?时会将原来的Err转换为我们自定义的Err
        impl From<io::Error> for AnyErr {
            fn from(err: io::Error) -> Self {
                AnyErr {
                    kind: "io".to_string(),
                    msg: err.to_string(),
                }
            }
        }

        impl Error for AnyErr {}

        fn io_dyn_err() -> Result<(), Box<dyn Error>> {
            io_err()?;
            Ok(())
        }

        fn io_err() -> Result<(), AnyErr> {
            File::open("xxoo")?;
            Ok(())
        }

        match io_err() {
            Ok(_) => (),
            Err(e) => eprint!("{}", e),
        }
    }
}

#[allow(dead_code)]
fn three() {
    // 归一化不同的错误类型
    // 在实际项目中, 我们往往会为不同的错误定义不同的类型, 这样做非常好
    // 但是如果你要"在一个函数中返回不同的错误呢"？ 我们应该需要一个能代表通用 error 的类型
    // 要实现这个目的有三种方式:
    //  1. 使用特征对象 Box<dyn Error>
    //  2. 自定义错误类型
    //  3. 使用 thiserror

    // 1. 使用特征对象
    // 如果我们希望自定义错误可以返回成 error 特征对象, 我们需要让自定义错误类型实现 Error 特征
    {
        // 自定义类型实现 Debug + Display 特征的主要原因就是为了能转换成 Error 的特征对象
        // 而特征对象恰恰是在同一个地方使用不同类型的关键
        fn render() -> Result<(), Box<dyn Error>> {
            let key = env::var("key")?;
            File::open(key)?;

            Ok(())
        }

        match render() {
            Ok(_) => (),
            Err(e) => eprintln!("1.特征对象 {}", e),
        }
    }
    // 但是有一个问题: Result 实际上不会限制错误的类型, 也就是一个类型就算不实现 Error 特征
    // 它依然可以在 Result<T, E> 中作为E来使用, 此时这种特征对象的解决方案就无能为力了

    // 2. 与特征对象相比, 自定义错误类型麻烦归麻烦. 但是它非常灵活, 因此也不具有上面的类似限制
    //    可以为不同的Error类型实现From, 因为 "?" 可以"进行隐式 From 转换", 可以将"其他 Error"转为自己的 Error

    // 3. 简化错误处理
    // anyhow第三方crate的错误处理
    {
        fn process_file(path: &str) -> Result<()> {
            fs::read_to_string(path).with_context(|| format!("not found path: {}", path))?;

            Ok(())
        }
        let mock = "mock.txt";
        match process_file(&mock) {
            Ok(_) => (),
            Err(e) => {
                // 我们可以对错误进行转换
                // add::<T>() 这也是一种泛型函数的使用方式, 用于手动推断其类型(我们用类型注释也可以的)
                println!("3.anyhow: {:#?}", &e);
                if let Ok(io_err) = e.downcast::<io::Error>() {
                    eprintln!("3.anyhow ioErr: {:#}", io_err);
                } else {
                    eprintln!("未知错误");
                }
            }
        }

        fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }

        // 实际上我们可以使用以下两种方式调用泛型函数, 其实都一样
        // 如果返回值不是泛型的话, 我们用第一种。 反之用第二种
        // add(1, 2);
        // add::<f32>(1.1, 2.2);
    }
}
