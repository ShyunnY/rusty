use std::{rc::Rc, sync::Arc, u8};

#[allow(dead_code)]
pub fn hello() {
    // 类型转换篇章
    // Rust 是类型安全的语言, 因此在Rust中 "做类型转换不是一件简单的事"

    // (1) as 类型转换
    // Rust 中内置了一些"基本类型之间的转换", 这里使用 as 操作符来完成
    // 因为每个类型能表达的数据范围不同, 如果把范围较大的类型转换成较小的类型, 会造成错误
    // 因此我们需要把范围较小的类型转换成较大的类型, 来避免这些问题的发生
    {
        let a = 1_000;
        let b = a as u8; // u8::Max=255, 强制转换会导致丢失数据
        println!("i32 -> u8 发生了数据丢失, result={}", b);
    }

    // (2) 将内存地址转换为指针
    // 强制类型转换的边角知识: 转换不具有传递性. 就算 e as U1 as U2 是合法的, 也不能说明 e as U2 是合法的（e 不能直接转换成 U2）
    {
        let arr = [1, 2, 3];
        let addr = arr.as_ptr() as usize;

        let second = addr + 4; // i32内存大小为4, 由于数组是连续分配内存的, 所以起始地址+4可以获取第二个元素
        let second_val = second as *mut i32;
        unsafe {
            *second_val = 22; // 在 unsafe 块中操作裸指针
        }
        println!("arr: {:?}", arr);
    }

    // (3) TryInto() 尝试转换
    // 在一些场景中, 使用 as 关键字会有比较大的限制
    // 如果你想要在类型转换上"拥有完全的控制而不依赖内置的转换", 例如处理转换错误, 那么可以使用 TryInto
    // 个人感觉类似于 Golang 中的类型断言: "v,ok := (interface{}).(Foo)", 尝试进行类型转换
    // 笔记: 如果你要使用一个特征的方法, 那么你需要引入该特征到当前的作用域中, 我们在上面用到了 try_into 方法, 因此需要引入对应的特征
    //      这是因为不同的特征可能存在同名方法, 所以我们需要显示引入该方法属于哪个trait的
    {
        let x: i16 = 1_000;
        // try_into 返回一个 Result, 我们通过"if let"的语法来匹配, 并且需要提供类型标注告诉编译期转换的类型
        let y: u8 = match x.try_into() {
            Ok(v) => v,
            Err(e) => {
                println!("转换失败, 原因是: {}", e);
                0
            }
        };
        println!("y = {y}");
    }

    // (4) 通用类型转换
    // 虽然 as 和 TryInto 很强大, 但是"只能应用在数值类型"上
    // 可是 Rust 有如此多的类型, 想要为这些类型实现转换, 我们需要另谋出路
    {
        // 一个例子: Foo 转换为 Bar
        // 这种转换方式看起来就很 "沙冰", 优秀如你肯定不能忍受!
        struct Foo {
            x: i32,
            y: i32,
        }

        struct Bar {
            x: i32,
            y: i32,
        }

        fn converse(foo: &Foo) -> Bar {
            Bar { x: foo.x, y: foo.y }
        }
    }
    //
    // 1. 强制类型转换
    // 在某些情况下类型是可以进行"隐式强制转换"的, 虽然这些转换弱化了 Rust 的类型系统
    // 但是它们的存在是为了让 Rust 在大多数场景可以工作(说白了，帮助用户省事), 而不是报各种类型上的编译错误
    // 首先在匹配特征时, 不会做任何强制转换(除了方法). 一个类型 T 可以强制转换为 U, 不代表 impl T 可以强制转换为 impl U
    {
        trait T {}

        impl<'a> T for &'a i32 {}

        fn foo<X: T>(_x: X) {}

        let _x: &mut i32 = &mut 1;
        // foo(x);  // 编译失败. "&mut i32可以转为 &i32, 但是 impl &mut i32 不能转为 impl &i32"
    }
    //
    // 2. 点操作符
    // 方法调用的点操作符看起来简单, 实际上非常不简单!
    // 它在调用时会发生很多魔法般的类型转换, 例如：自动引用、自动解引用，强制类型转换直到类型能匹配等
    // 假设有一个方法 foo, 它有一个接收器(接收器就是 self、&self、&mut self 参数)
    // 如果调用 value.foo(), 编译器在调用 foo 之前, 需要决定到底使用哪个"接受者类型"来调用
    {
        /*
           再进一步, 我们使用完全限定语法来进行准确的函数调用:
            1. 首先编译器检查它是否可以直接调用 T::foo(value), 称之为值方法调用(self方式调用)
            2. 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现, 上文提到过特征不能进行强制转换)
              那么编译器会尝试增加自动引用,例如会尝试以下调用: <&T>::foo(value) 和 <&mut T>::foo(value), "称之为引用方法调用"
            3. 若上面两个方法依然不工作, 编译器会试着解引用 T , 然后再进行尝试.
              这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U),
              那么编译器会使用 U 类型进行尝试, 称之为解引用方法调用
            4. 若 T 不能被解引用, 且 T 是一个定长类型(在编译期类型长度是已知的),那么编译器也会尝试将 T 从定长类型转为不定长类型
               例如将 [i32; 2] 转为 [i32]. 若还是不行, 那...没有那了, 最后编译器大喊一声：汝欺我甚，不干了！

            根据以上原则, 我们可以总结一套规则:
            * 首先看看值方法调用 (T是否存在这个方法) 是否Ok
            * 然后再看看引用方法调用 (&T是否存在这个方法) 是否Ok
            * 进一步看看对 类型T 进行 "解引用" 再重复尝试上两个规则是否可以(e.g. 有些类型被智能指针包裹住, 所以可以 Deref 解引用)
            * 最后发现 类型T 不能解引用了, 并且 类型T 是定长的, 尝试转为不定长再重复上面的规则 (例如数组可以转为切片)
            * rust: 我不玩了, 我摊牌了!
        */

        // Example1:
        // 我们可以尝试运用上面的规则
        // arr 数组的底层数据隐藏在了重重封锁之后, 那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁
        // 准确的访问到数组中的第一个元素？
        /*
            * 首先 arr[0] 只是Index特征的语法糖: 编译器会将 array[0] 转换为 array.index(0) 调用,
              (当然在调用之前, 编译器会先检查 array 是否实现了 Index 特征)
            * 接着编译器检查 Rc<Box<[T; 3]>> 的值方法是否有实现 Index 特征, 那自然没有
              不仅如此, &Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 的引用方法也没有实现 [[🌟规则一和二]]
            * 既然上面的都不能工作, 编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
              此时继续对 Box<[T; 3]> 进行上面的操作: Box<[T; 3]>, &Box<[T; 3]> 和 &mut Box<[T; 3]> 都没有实现 Index 特征
              所以编译器再进一步开始对 Box<[T; 3]> 进行解引用, 然后我们得到了 [T; 3] [[🌟规则三和规则一和规则二]]
            * [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉! 在直觉中, 数组都可以通过索引访问,实际上只有数组切片才可以!)
              它也不能再进行解引用, 因此编译器只能祭出最后的大杀器: 将定长转为不定长 [[🌟规则四]]
              因此 [T; 3] 被转换成 [T], 也就是数组切片! 切片实现了 Index 特征, 因此最终我们可以通过 index 方法访问到对应的元素
        */
        let arr: Rc<Box<[i32; 3]>> = Rc::new(Box::new([520, 2, 3]));
        println!("first: {}", arr[0]);

        // Example2:
        /*
            Q: cloned 的类型是什么？

            A: 首先编译器检查能不能进行值方法调用, t的类型是 &T
              同时clone方法的签名是 `fn clone(&self) -> Self`
              因此可以进行值方法调用, 再加上编译器知道了 T 实现了 Clone, 因此 cloned 的类型是 T
              // 由于 T 就存在clone方法, 所以直接调用 T.clone() 即可返回 T
              // 所以其实是 Clone::clone(t)  --> 也就是值方法调用(不需要传入引用哦～)
              // Clone: fn(&self) -> Self
        */
        fn do_something1<T: Clone>(t: &T) {
            let _cloned = t.clone(); // _cloned Type = &T
        }

        /*
            Q: cloned 的类型是什么？

            A: 首先通过值方法调用就不再可行, 因为 T 没有实现 Clone 特征, 也就无法调用 T 的 clone 值方法
               (也就是无法通过 Clone::clone(t) 值调用)
               接着编译器尝试引用方法调用, 此时 T 变成 &T
               在这种情况下, clone 方法的签名如下: `fn clone(&&T) -> &Self`
               接着我们现在对t进行了引用, 编译器发现 &T 实现了 Clone 类型(所有的引用类型都可以被复制，因为其实就是复制一份地址)
               因此可以推出 cloned 也是 &T 类型
               最终, "我们复制出一份引用指针", 这很合理!
               因为值类型 T 没有实现 Clone, 只能去复制一个指针了
               // 由于 T 不存在clone方法, 所以无法直接调用 T.clone() -> T (规则一失效)
               // 然后我们进一步对T添加引用 => &T, 发现 &T 实现了 Clone, (所有的引用类型都可以被复制,因为其实就是复制一份地址)
               // 所以此时可以调用引用方法, 对引用指针进行拷贝 ----------|
               // Clone: fn(&&self) -> &Self
        */
        fn do_something2<T>(_t: &T) {
            // let _cloned = t.clone();     // _cloned Type = &T
        }

        // Example3
        /*
            Q: foo_cloned 和 bar_cloned 的类型是什么?

            A: 首先要复习一下复杂类型派生 Clone 的规则:
               一个复杂类型能否派生 Clone, 需要它内部的"所有子类型都能进行 Clone"
               因此 Container<T>(Arc<T>) 是否实现 Clone 的关键在于 T 类型是否实现了 Clone 特征
               如果 T 不能实现 Clone, 那就代表 Container 实际上是不能实现 Clone 的

               // 1. Container<i32> 实现了 Clone 特征(i32实现了 Clone)
                  因此编译器可以直接进行值方法调用
                  此时相当于直接调用 Clone::clone(foo), 其中 clone 的函数签名是 `fn clone(&T) -> T`

               // 2. 然而, bar_cloned 的类型却是 &Container<T>
                  这里我们并不能确定 T 是什么类型, 即使我们通过 attribute 进行派生了 Clone, 我们也无法保证 T 的真实类型实现了 Clone
                  /*
                    // derive 展开的代码, 可以看见实现了要求就是 T: Clone
                    // 如果你的 T 不满足 Clone trait, 那就是不行呀
                    impl<T> Clone for Container<T> where T: Clone {
                        fn clone(&self) -> Self {
                            Self(Arc::clone(&self.0))
                        }
                    }
                   */
                  所以此时仅仅会对其 &Container 这个引用进行拷贝(又因为引用是个指针, 直接支持Clone的)
                  故得出结论:

                  foo_cloned: T=i32, 支持Clone, 所以调用值方法进行克隆 => Container<i32>
                  bar_cloned: T=???, 不确定是否支持Clone, 所以值方法不具有clone, 只能看其引用 &T
                              发现引用可以clone, 所以调用引用方法进行克隆 => &Container<T>
        */
        #[derive(Clone)]
        struct Container<T>(Arc<T>);

        fn clone_containers<T>(_foo: &Container<i32>, _bar: &Container<T>) {
            // let _foo_cloned = foo.clone();
            // let _bar_cloned = bar.clone();
        }
    }

    // (5) 变形记(Transmutes)
    // 前方危险, 敬请绕行！
    // 类型系统, 你让开！
    // 我要自己转换这些类型, 不成功便成仁！
    // 虽然本书大多是关于安全的内容, 我还是希望你能仔细考虑避免使用本章讲到的内容
    // 这是你在 Rust 中所能做到的真真正正、彻彻底底、最最可怕的非安全行为, 在这里所有的保护机制都形同虚设
    // ** 太可怕了,我们先不看 **
}
