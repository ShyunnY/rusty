use std::pin::Pin;

#[allow(dead_code)]
pub fn hello() {
    // 结构体自引用在 Rust 中是一个众所周知的难题, 我们先来看看其样貌
    {
        struct SelfRef<'a> {
            val: String,
            val_ref: &'a str, // val_ref需要引用val
        }

        // 也就是说, 让结构体内部的引用指向同一个结构体内部的字段（此时生命周期与结构体保持一致）

        // 我们无法编译以下代码, 因为同时发生了 move + 借用
        // let s = "".to_string();
        // let sr = SelfRef {
        //     val: s,
        //     val_ref: &s,
        // };
    }

    // 1. 使用 Option 解决
    // 在某种程度上来说, Option 这个方法可以工作. 但是这个方法的限制较多, 例如从一个函数创建并返回它是不可能的!
    // 在返回值类型上需要提供生命周期, 但是这个生命周期是凭空产生的
    {
        #[derive(Debug)]
        struct SelfRef<'a> {
            val: String,
            val_ref: Option<&'a str>, // val_ref需要引用val
        }

        let s = "self".to_string();
        let mut sr = SelfRef {
            val: s,
            val_ref: None,
        };
        sr.val_ref = Some(&sr.val);

        println!("Option解决自引用: {:?}", sr);
    }

    // 2. 使用 unsafe 方式
    // 我们在 pointer_to_value 中直接存储裸指针, 而不是 Rust 的引用
    // 因此不再受到Rust借用规则和生命周期的限制, 而且实现起来非常清晰、简洁
    // 但是缺点就是: 通过指针获取值时需要使用 unsafe 代码
    {
        #[derive(Debug)]
        struct SelfRef {
            val: String,
            val_ref: *const String,
        }

        impl SelfRef {
            fn new(txt: &str) -> Self {
                SelfRef {
                    val: String::from(txt),
                    val_ref: std::ptr::null(),
                }
            }

            fn init(&mut self) {
                self.val_ref = &self.val;
            }

            fn value(&self) -> &String {
                &self.val
            }

            fn point_value(&self) -> &String {
                assert!(!self.val_ref.is_null(), "");

                unsafe { &*(self.val_ref) }
            }
        }

        let sr = SelfRef::new("hello,self");
        println!(
            "unsafe 解决自引用: value = {}, addr = {:p}",
            sr.value(),
            sr.value()
        );
        // println!(
        //     "unsafe 解决自引用: point_value = {}, addr = {:p}",
        //     sr.point_value(),
        //     sr.point_value(),
        // );
    }

    // 3. 使用 Pin
    // 在Rust中, Pin就是用来确保某个数据（比如一个自引用的结构体）"在内存中的位置不会改变"
    // 这样, 任何指向这个数据的引用都始终是有效的
    // 我们可以发现: 自引用最大的问题就是所有权发生移动时, 内存地址会变化! 所有Pin就保证内存地址不要进行修改
    {
        #[derive(Debug)]
        struct NoMove {
            val: String,
            val_ref: *const String,
        }

        impl NoMove {
            fn new(msg: String) -> Pin<Box<NoMove>> {
                // 首先我们使用 Box 将对象的值给固定住, 裸指针先给一个空值
                let mut ret = Box::pin(NoMove {
                    val: msg,
                    val_ref: std::ptr::null(),
                });

                // 然后获取裸指针的值, 并将其赋值到结构体中
                let raw: *const String = ret.val.as_ptr() as *const String;
                (*ret.as_mut()).val_ref = raw;

                // 最后返回一个 Pin
                ret
            }
        }

        let ret = NoMove::new("msg".to_string());
        println!("ret: {:?} val_addr: {:p}", *ret, &*ret.val);
    }

    // TODO: 拓展知识: 如何使用Pin？
}
