use core::slice;
use std::{slice::from_raw_parts, str::from_utf8, vec};

#[allow(dead_code)]
pub fn hello() {
    // (1) 解引用裸指针
    one();
    // (2) unsafe 函数
    two();
    // (3) 安全抽象包裹住 unsafe～
    three();
    // (4) 实现unsafe特征~
    four();
    // (5) 使用 union 联合体
    five();

    /*
       一个小总结:
       unsafe 只应该用于这五种场景,其它场景你应该坚决的使用安全的代码:
       1. "解引用裸指针"
       2. "调用一个 unsafe 或外部的函数"
       3. "访问或修改一个 `可变的静态变量` "
       4. "实现一个 unsafe 特征"
       5. "访问 union 中的字段"

       能不使用 unsafe 一定不要使用, 就算使用也要控制好边界让范围尽可能的小!
       在使用 unsafe 时一定要控制好边界!!!
       并且裸指针是不能自动进行Drop的, 所以我们需要确保指向的内存是有效的!!!
    */
}

#[allow(dead_code)]
fn one() {
    /*
        裸指针(raw pointer, "又称原生指针") 在功能上跟引用类似, 同时它也需要显式地注明可变性/不可变性
        但是又和引用有所不同, 裸指针长这样: *const T 和 *mut T, 它们分别代表了不可变和可变

        之前学过 * 操作符, 知道它可以用于解引用. 但是在裸指针 *const T 中, 这里的 * "只是类型名称的一部分", 并没有解引用的含义
        至此我们已经学过三种类似指针的概念: 引用、智能指针和裸指针

        与前两者不同, 裸指针:
        1.可以"绕过 Rust 的借用规则", 可以同时拥有一个数据的可变、不可变指针, 甚至还能拥有多个可变的指针(不受借用规则管控)
        2.并"不能保证指向合法的内存", 可以是 null
        3."没有实现任何自动的回收" (drop)

        总之, 裸指针与我们传统学习的编程语言中指针概念一样
    */

    // 1. 基于引用创建裸指针
    // 在创建裸指针时我们并没有使用 unsafe 包裹: 这是因为"创建裸指针是安全的行为", 而"解引用裸指针才是不安全的行为"
    {
        let num = 99;
        let _raw_mut = num as *mut i32;
        let _raw_immut = num as *const i32;
    }

    // 2. 基于内存地址创建裸指针
    // 基于一个内存地址来创建裸指针: 可以想象这种行为是相当危险的
    // 试图使用任意的内存地址往往是一种未定义的行为(undefined behavior),因为该内存地址有可能存在值也有可能没有, 就算有值也大概率不是你需要的值
    {
        // 如果我们想使用内存地址构建裸指针, 一定需要使用现有的内存地址!!!

        let msg = String::from("value");
        let length = msg.len();

        // 将 String 的地址转为一个不可变的裸指针
        let addr = msg.as_ptr() as usize;
        let p = addr as *mut u8;
        unsafe {
            let ret = from_utf8(from_raw_parts(p, length)).unwrap();
            println!("1.2 内存地址 ret got {{{ret}}}");
        }
    }

    // 3. 使用 "*" 对裸指针进行进行解引用
    // 对裸指针进行解引用时, 一定需要使用 unsafe 块包裹起来(这是不安全的)
    {
        let data = 99;
        // 我们可以对其进行隐式转换, 但建议通过 as 进行显示转换
        let raw: *const i32 = &data;

        unsafe { println!("1.3 解引用 data got {{{}}}", *raw) }
    }

    // 4. 基于智能指针创建裸指针
    {
        let b = Box::new(100_00);
        let raw = &*b as *const i32;

        unsafe {
            println!("1.4 智能指针 data got {{{}}}", *raw);
        }
    }

    /*
       小结:
       使用裸指针可以让我们创建两个可变指针都指向同一个数据!
       如果使用安全的Rust,你是"无法"做到这一点的. 违背了借用规则, 编译器会对我们进行无情的阻止
       因此裸指针可以"绕过借用规则", 但是由此带来的数据竞争问题, 就需要大家自己来处理了
       (RefCell也可以在编译期间绕过借用规则, 将其推迟到编译期
    */
}

#[allow(dead_code)]
fn two() {
    /*
        unsafe 函数从外表上来看跟普通函数并无区别, 唯一的区别就是它需要使用 "unsafe fn" 来进行定义
        这种定义方式是为了告诉调用者: 当调用此函数时你需要注意它的相关需求, 因为 Rust 无法担保调用者在使用该函数时能满足它所需的一切需求

        强制调用者加上 "unsafe" 语句块才能调用 unsafe 函数, 就可以让他清晰的认识到: 正在调用一个不安全的函数, 需要小心看看文档
        看看函数有哪些特别的要求需要被满足

        使用 unsafe 声明的函数时, 一定要看看相关的文档, 确定自己没有遗漏什么
        还有, unsafe 函数中 "无需套娃再次使用unsafe块, 因为认为整个函数作用域内都是不安全的(unsafe)"
    */
    {
        // 整个 hey() 函数内都是unsafe的
        unsafe fn hey() {
            println!("2.1 unsafe函数的 hey kong!");
        }

        // 必须要在 unsafe块 中才能调用 unsafe函数
        unsafe {
            hey();
        }
    }
}

#[allow(dead_code)]
fn three() {
    /*
        一个函数包含了 unsafe 代码不代表我们需要将整个函数都定义为 unsafe fn
        只要我们能够确保函数内部的 unsafe 使用是合法的/安全的/兜底的, 那么我们就大胆的使用
    */

    /*
       以下代码中存在几个关键点:
       1."as_mut_ptr" 会返回"指向 slice 首地址"的裸指针 *mut i32
       2."slice::from_raw_parts_mut" 函数 "通过指针和长度" 来创建一个新的切片. 简单来说: 该切片的初始地址是 ptr, 长度为 mid
       3."ptr.add(mid)" 可以获取第二个切片的初始地址. 由于切片中的元素是 i32 类型, 每个元素都占用了 4 个字节的内存大小
         实际上对 ptr 进行添加了 T 大小字节的偏移量后获取指针地址 => ptr + 4 * mid(获取偏移后的地址)
    */
    // 虽然 "split_at_mut 使用了 unsafe", 但我们无需将其声明为 unsafe fn
    // 这种情况下就是使用安全的抽象包裹 unsafe 代码, "这里的 unsafe 使用是非常安全的, 因为我们从合法数据中创建了的合法指针"
    fn split_mut_slice(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let addr = slice.as_mut_ptr();
        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(addr, mid),
                slice::from_raw_parts_mut(addr.add(mid), len - mid),
            )
        }
    }

    // 通过 unsafe 的裸指针我们可以同时获取同一个vec的两个可变切片引用
    let mut arr = vec![1, 2, 3, 99, 88, 77];
    let (r1, r2) = split_mut_slice(&mut arr, 3);
    r1.iter_mut().for_each(|x| *x += 10);
    r2.iter_mut().for_each(|x| *x += 1);
    println!("3.1 安全的抽象unsafe函数 {{{:?}}}", arr);

    // 所以我们在使用编写 unsafe 时, 一定要再三确保可靠性
    // 以下代码你敢用不?
    // let addr = 0x01234567;
    // let p = addr as *const i32;
}

#[allow(dead_code)]
fn four() {
    // 实现 unsafe trait
    /*
        说实话 unsafe 的特征确实不多见, 之所以会有 unsafe 的特征, 是因为该特征至少有一个方法包含有编译器无法验证的内容
        通过 unsafe impl 的使用, 我们告诉编译器: "相应的正确性由我们自己来保证",无需操心
    */

    // 声明一个 unsafe 的trait
    unsafe trait Foo {
        fn demo();
    }
}

#[allow(dead_code)]
fn five() {
    // union 是一种特殊的数据类型, 它允许"在相同的内存位置存储不同的数据类型"
    // union 可以存储多种类型的数据, 但一次只能存储其中一种类型(也就是用一块内存来存储不同的类型)
    // 由于 union 可以存储多种类型的数据, 因此使用时需要非常小心以确保不会意外地覆盖数据
    //
    /*
        访问 union 的字段是不安全的, 因为 Rust 无法保证当前存储在 union 实例中的数据类型, 故我们需要使用 unsafe
        union 的使用方式跟结构体确实很相似, "但是前者的所有字段都共享同一个存储空间"
        意味着往 union 的某个字段写入值，会"导致其它字段的值会被覆盖"
    */

    union Number {
        d1: i32,
        d2: f32,
    }

    let mut number = Number { d1: 99 };
    unsafe {
        println!("5.1 在unsafe中使用union number: {:?}", number.d1);
    }

    number.d2 = 1.1;
    unsafe {
        println!("5.1 在unsafe中使用union number: {:?}", number.d2);
    }
}
