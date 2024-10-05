use std::ops::{Deref, DerefMut};

#[allow(dead_code)]
pub fn hello() {
    // Defer智能指针
    /*
       何为智能指针？能不让你写出`****s`形式的解引用, 我认为就是智能
       智能指针的名称来源, 主要就在于它实现了 Deref 和 Drop 特征，这两个特征可以智能地帮助我们节省使用上的负担：
       * Deref 可以"让智能指针像引用那样工作", 这样你就可以写出"同时支持智能指针和引用的代码", 例如 *T
       * Drop 允许你指定"智能指针超出作用域后自动执行的代码", 例如做一些数据清除等收尾工作
    */

    // (1) 通过'*'获取引用背后的值
    // 常规引用是一个指针类型, "包含了目标数据存储的内存地址"
    // 对常规引用使用"*"操作符, 就可以通过"解引用的方式获取到内存地址对应的数据值"
    {
        let x = 5;
        let y = &10;
        assert_eq!(x, 5);
        assert_eq!(*y, 10);
        // assert_eq!(5, y);    // 我们无法比较一个i32和引用
    }

    // (2) 智能指针解引用
    // Rust 将解引用提升到了一个新高度. 考虑一下智能指针, 它是一个结构体类型
    // 如果你直接对它进行 *XXX, 显然编译器不知道该如何办(rust: 你对一个结构体使用"*", are you ok?)
    // 因此我们可以为智能指针结构体"实现 Deref 特征"
    // "实现 Deref 后的智能指针结构体", 就可以像普通引用一样通过 * 进行解引用
    {
        // x本质上是结构体, 但是为啥我们能够直接对结构使用"*"呢?
        // 原因就在于: Box实现了 Deref trait
        let x = Box::new(10);
        assert_eq!(*x, 10); // Box被 deref 成 i32
    }
    // 定义自己的智能指针
    {
        // 现在让我们一起来实现一个智能指针, 功能上类似 Box<T>。
        struct MyBox<T>(T);

        // 为 MyBox 实现 Deref 特征, 以支持 * 解引用操作符
        // (如果不实现deref, 你直接使用'*MyBox', 天知道你想干啥)
        // 以下代码可以翻译成
        // MyBox<T>: Deref<Target=T>, "&MyBox == &T", "*MyBox == *T"
        impl<T> Deref for MyBox<T> {
            type Target = T;

            // 这里有一个关键点: 我们返回的是内部数据的"常规引用"
            fn deref(&self) -> &Self::Target {
                &(self.0)
            }
        }

        impl<T> MyBox<T> {
            fn new(t: T) -> Self {
                MyBox(t)
            }
        }

        // 此时我们可以对MyBox进行解引用啦～
        // 当我们调用 *MyBox 时, rust编译期会尝试调用MyBox的deref方法, 获取其返回的常规引用
        // 所以实际上: "*MyBox == *(MyBox.deref())"
        let x = MyBox::new(10);
        assert_eq!(*x, 10);
    }

    // (3) "*"背后的原理
    /*
       当我们对智能指针Box(或者是实现了deref的类型)进行"解引用时", 实际上 Rust 为我们调用了以下方法：
       ```*(y.deref())```

       1.首先调用 deref 方法返回值的常规引用
       2.然后"通过 * 对常规引用进行解引用", 最终获取到目标值

       至于Rust为何要使用这个有点啰嗦的方式实现, 原因在于"所有权系统的存在"
       如果 deref 方法直接返回一个值而不是引用, 那么该值的"所有权将被转移给调用者"
       而我们不希望调用者仅仅只是 *T 一下, 就拿走了智能指针中包含的值(很不合理)

       需要注意的是: "* 不会无限递归替换"
       从 *y 到 *(y.deref()) 只会发生一次, 而不会继续进行替换然后产生形如 *((y.deref()).deref()) 的怪物
       如果我们想多次进行解引用, 我们也需要多次使用 "*". 例如： "**y == *(y.deref()).deref()"
    */

    // (4) 函数和方法中的隐式 Deref 转换
    /*
       对于函数和方法的传参, Rust 提供了一个极其有用的隐式转换: Deref 转换
       若一个类型实现了 Deref 特征, 那它的"引用"在传给函数或方法时，会根据参数签名来决定是否进行"隐式的 Deref 转换"

       注意这里有几个细节:
       1.类型需要实现 Deref trait
       2."类型的引用"传递给函数或方法
    */
    {
        // Example:
        // 1. String 实现了 Deref 特征, 可以在需要时自动被转换为deref返回的类型(这里是&str)
        // 2. &s 是String的引用类型, 当它被传给 display 函数时, 自动通过 Deref 转换成了&str
        // 3. 必须使用 &s 的方式来触发 Deref("仅引用类型的实参"才会触发自动解引用, 一定要是 引用!引用!引用! 重要的事说三遍)

        let s = String::from("隐式deref进行转换");
        display_str(&s);

        fn display_str(s: &str) {
            println!("&str: {}", s);
        }
    }
    //
    // 连续的隐式 Deref 转换
    // 如果你以为 Deref 仅仅这点作用, 那就大错特错了!
    // Deref 可以支持连续的隐式转换, 直到找到适合的形式为止:
    {
        let b = Box::new(String::from("我是被Box包裹住的String"));

        // 这里做了多次的连续隐式Deref转换
        // 1.将&Box通过Deref转换为&String
        // 2.将&String通过Deref转换为&str
        // 3.发现匹配了函数签名, 此时停止转换
        display_str(&b);

        fn display_str(s: &str) {
            println!("&str: {}", s);
        }
    }
    // 总之, 当参与其中的类型定义了 Deref 特征时:
    // Rust **"会分析该类型并且连续使用 Deref 直到最终获得一个引用来匹配函数或者方法的参数类型"**
    // 这种行为完全不会造成任何的性能损耗, 因为完全是在编译期完成
    //
    //
    /*
       但是 Deref 并不是没有缺点, 缺点就是：
       如果你不知道某个类型是否实现了 Deref 特征, 那么在看到某段代码时, 并不能在第一时间反应过来该代码发生了隐式的 Deref 转换
       事实上不仅仅是 Deref, 在 Rust 中还有各种 From/Into 等等会给阅读代码带来一定负担的特征
       还是那句话: 一切选择都是权衡, 有得必有失! 得了代码的简洁性, 往往就失去了可读性.
       Go 语言就是一个刚好相反的例子

       // 总而言之就是提高了灵活性, 但是减少了可读性
    */
    {
        // Example2
        // 在 赋值 和 方法调用 中进行隐式Deref转换
        let b = Box::new(String::from("句号"));
        // 背地里隐式调用了 "2次Deref" 来转换成 &str (注意: 赋值操作需要手动解引用)
        let expr: &str = &b;
        // 背地里隐式调用了 "2次Deref" 来转换成 &str 再调用方法 (注意: 方法调用会自动解引用)
        let s = b.to_string();

        println!("expr: {expr}");
        println!("s: {s}");
    }

    // (5) Deref 规则总结
    /*
        Deref小小滴规则总结:
        一个类型为 T 的对象 Foo, 一个类型为 U 的对象 Bar
        如果 T: Deref<Target=U> 那么 foo 的引用 &foo 在应用的时候会自动转换为 &bar
    */
    //
    // 引用归一化
    // Rust 编译器实际上只能对 &v 形式的引用进行解引用操作, 那么问题来了: 如果是一个智能指针或者 "&&&&v "类型的呢?
    // 答案是: Rust 会在解引用时自动把智能指针和 &&&&v 做"引用归一化操作", 转换成 &v 形式, 最终再对 &v 进行解引用
    // 换句话说: 就是将 &&&&&&&&v => &v, 然后再进行解引用
    // 1.把智能指针（比如在库中定义的，Box、Rc、Arc、Cow 等）从结构体Deref为内部的引用类型, 也就是转成结构体内部的 &v
    // 2.把多重&, 例如 &&&&&&&v，归一成 &v
    /*
        // 可以看到以下标准库的源码
        // &T: Deref<Target=T>
        // 也就是说会将 &&T => &T
        // 那么按照这个逻辑 &&&T 经过多次deref最终会归一化成为 &T
        impl<T: ?Sized> Deref for &T {
            type Target = T;

            fn deref(&self) -> &T {
                *self
            }
        }
    */
    {
        // 一个例子
        // 你瞧瞧, 这里有多少个&!
        // 但是不用怕! 最终还是可以归一化为 &T
        // 然后进行deref => &str 拿到与签名对应的引用类型!!!
        fn display_str(s: &str) {
            println!("&str: {}", s);
        }

        let b = Box::new(String::from("example"));
        display_str(&&&&&&&&&&&&b);
    }

    // (6) 三种 Deref 转换
    /*
       在之前我们讲的都是"不可变的 Deref 转换"
       实际上 Rust 还支持"将一个可变的引用转换成另一个可变的引用"以及"将一个可变引用转换成不可变的引用", 规则如下：
       1.当 T: Deref<Target=U>, 可以将 &T 转换成 &U, 也就是我们之前看到的例子
       2.当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
       3.当 T: Deref<Target=U>, 可以将 &mut T 转换成 &U

       NOTE: Rust 可以把可变引用隐式的转换成不可变引用, 但反之则不行
       其实我们从所有权和借用规则中可以理解:
       当你拥有一个可变的引用, 那"该引用肯定是对应数据的唯一借用", 那么此时将可变引用变成不可变引用并不会破坏借用规则!
       但是如果你拥有一个不可变引用, 那同时"可能还存在其它几个不可变的引用"
       如果此时将其中一个不可变引用转换成可变引用, 就变成了可变引用与不可变引用的共存, 最终破坏了借用规则
       //
       所以说: 手握不可变引用相当于拿捏了, 这个厕所只有我一个人可以上！
    */
    {
        // Example
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(v: T) -> Self {
                MyBox(v)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &(self.0)
            }
        }

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut (self.0)
            }
        }

        fn demo1(s: &mut String) {
            s.push_str("abc");
        }

        fn demo2(s: &String) {
            println!("sss: {s}");
        }

        // 注意哈: 我们在使用引用时, 也需要手动指明是否是可变引用: &mut ref/&ref
        let mut b = MyBox::new(String::from("==>"));
        demo1(&mut b); // &mut T => &mut U
        demo2(&mut b); // &mut T => &U
    }

    // 总结啦!
    /*
       Deref 可以说是 Rust 中最常见的隐式类型转换
       而且它可以连续的实现如 Box<String> => String => &str 的隐式转换, 只要链条上的类型都实现了 Deref 特征

       我们也可以为自己的类型实现 Deref 特征
       但是原则上来说, "只应该为自定义的智能指针实现 Deref"
       例如虽然你可以为自己的自定义数组类型实现 Deref 以避免 myArr.0[0] 的使用形式
       '但是 Rust 官方并不推荐这么做, 特别是在你开发三方库时'
    */
}
