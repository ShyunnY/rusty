use std::{marker::PhantomPinned, pin::Pin};

#[allow(dead_code)]
pub fn hello() {
    // 牛逼的定海神针 -- Pin/Unpin

    // Pin 在栈上
    // one();

    // Pin 在堆上
    // two();
}

#[allow(dead_code)]
fn one() {
    // 自引用下
    // 交换一下指针指向的数据, 打印之后会发现: testA和testB的裸指针都指向了对方的数据中
    // swap只是交换了指针的内容值, *const指针的地址值没有变化, 所有就会导致引用变成了不合法了

    #[derive(Debug)]
    struct PinTest {
        data: String,
        data_ref: *const String,
        // 使用 PhantomPinned 类型标记自动帮助我们实现了 "!Unpin" 特征
        _marker: PhantomPinned,
    }

    impl PinTest {
        fn new(msg: &str) -> Self {
            PinTest {
                data: String::from(msg),
                data_ref: std::ptr::null(),
                _marker: PhantomPinned,
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ref: *const String = &self.data;
            let this = unsafe {
                // get_unchecked_mut 获取 self数据 的可变引用: 必须由我们保证不会移动 self数据
                // 针对 &mut T 有以下几种方式移动(多数与 mem 相关)
                // 1. 通过 std::mem::replace 移动
                // 2. 通过 std::mem::take 移动
                // 3. 通过 std::mem::swap 交换
                // 4. 通过 Option::take 移动(本质也是 mem::replace )
                self.get_unchecked_mut()
            };
            this.data_ref = self_ref;
        }

        fn get_data(self: Pin<&Self>) -> &str {
            &self.get_ref().data
        }

        fn get_data_ref(self: Pin<&Self>) -> &str {
            assert!(!self.data_ref.is_null(), "data_ref no null!");
            unsafe { &*self.data_ref }
        }
    }

    let mut a = PinTest::new("testA");
    let mut test_a = unsafe { Pin::new_unchecked(&mut a) };
    PinTest::init(test_a.as_mut());

    let mut b = PinTest::new("testB");
    let mut test_b = unsafe { Pin::new_unchecked(&mut b) };
    PinTest::init(test_b.as_mut());

    // 不让移动
    // std::mem::swap(test_a.get_mut(), test_b.get_mut());
}

#[allow(dead_code)]
fn two() {
    #[derive(Debug)]
    struct PinTest {
        data: String,
        data_ref: *const String,
        // 使用 PhantomPinned 类型标记自动帮助我们实现了 "!Unpin" 特征
        _marker: PhantomPinned,
    }

    impl PinTest {
        fn new(msg: &str) -> Pin<Box<Self>> {
            let pt = PinTest {
                data: String::from(msg),
                data_ref: std::ptr::null(),
                _marker: PhantomPinned,
            };

            // 使用 Box 将其 Pin 到堆上
            let mut boxed = Box::pin(pt);
            let p: *const String = &boxed.as_ref().data;

            unsafe {
                // 使用 as_mut 将 Pin<Box<PinTest>> => Pin<&mut PinTest>
                boxed.as_mut().get_unchecked_mut().data_ref = p;
            }

            boxed
        }

        fn get_data(self: Pin<&Self>) -> &str {
            &self.get_ref().data
        }

        fn get_data_ref(self: Pin<&Self>) -> &str {
            unsafe { &*self.get_ref().data_ref }
        }
    }

    {
        struct Foo {
            data: String,
            _marker: PhantomPinned,
        }

        impl Foo {
            fn new() -> Self {
                Foo {
                    data: "foo".to_string(),
                    _marker: PhantomPinned,
                }
            }
        }

        // 固定在栈上, 无法获取 &mut T
        let mut foo = Foo::new();
        let mut _foo = unsafe { Pin::new_unchecked(&mut foo) };
        // let x = foo.get_mut();

        // 固定在堆上, 无法获取 &mut T
        let mut _bar = Box::pin(Foo::new());
        // let a = _bar.as_mut().get_mut().data;
    }
}
