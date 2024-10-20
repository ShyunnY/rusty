use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[allow(dead_code)]
pub fn hello() {
    // Rust的安全性是众所周知的, 但是不代表它不会内存泄漏
    // 一个典型的例子就是同时使用 Rc<T> 和 RefCell<T> 创建循环引用, "最终这些引用的计数都无法被归零"
    // "因此 Rc<T> 拥有的值也不会被释放清理"

    // 1. 循环引用
    //
    // 我们通过 Rc + RefCell 构建出一个循环依赖: a => b, b => a
    // 此时 a,b 的Rc引用计数都是2. 当超出作用域后, a和b变量都被drop了, 此时Rc全都变成 1
    // 但是此时 a和b "相互持有对方的Rc", 所以导致内存无法清理进而发生内存泄漏:
    // realConsA(rc=2) <- let a, realConsB
    // realConsB(rc=2) <- let b, realConsA
    // drop...
    // realConsA(rc=1) <- realConsB
    // realConsB(rc=1) <- realConsA
    // 此时ListA,ListB都不能被释放, 因为他们相互持有了对方的所有权!
    {
        #[derive(Debug)]
        enum List {
            // Cons(i32, Rc<RefCell<List>>)
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    List::Cons(_, item) => Some(item),
                    List::Nil => None,
                }
            }
        }

        let a: Rc<List>;
        let b: Rc<List>;
        {
            // 创建a
            a = Rc::new(List::Cons(10, RefCell::new(Rc::new(List::Nil))));
            println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
            println!("a指向的节点 = {:?}", a.tail());

            // 创建b
            b = Rc::new(List::Cons(99, RefCell::new(Rc::new(List::Nil))));
            println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
            println!("b指向的节点 = {:?}", b.tail());

            // 让 b => a(指向a)
            if let Some(item) = b.tail() {
                *item.borrow_mut() = Rc::clone(&a);
            }
            println!("此时发生 b => a");
            println!("a的rc计数 = {}", Rc::strong_count(&a));

            // 让 a => b
            if let Some(item) = a.tail() {
                *item.borrow_mut() = Rc::clone(&b);
            }
            println!("此时发生 a => b");
            println!("b的rc计数 = {}", Rc::strong_count(&b));
        }
    }

    // 2. Weak
    /*
        Weak 非常类似于Rc, 但是与Rc持有所有权不同, "Weak不持有所有权", 它仅仅保存一份指向数据的弱引用
        如果你想要访问数据需要通过 Weak 指针的 upgrade 方法实现, 该方法返回一个类型为 Option<Rc<T>> 的值

        看到这个返回相信大家就懂了: 何为弱引用？就是"不保证引用关系依然存在", 如果不存在就返回一个 None！

        因为 Weak 引用不计入所有权, 因此它无法阻止所引用的内存值被释放掉, 而且 Weak 本身不对值的存在性做任何担保
        引用的值还存在就返回 Some, 不存在就返回 None
        (一般Option的就是用于告诉我们, 提供者不保证值一定能存在!)

        Weak的弱恰恰非常适合我们实现以下的场景:
        1.持有一个 Rc 对象的"临时引用", 并且不在乎引用的值是否依然存在
        2.阻止 Rc 导致的循环引用, "因为 Rc 的所有权机制会导致多个 Rc 都无法计数归零"
        3.使用方式简单总结下：对于父子引用关系，可以让父节点通过 Rc 来引用子节点，然后让子节点通过 Weak 来引用父节点
    */
    // 我们可以看看 weak 的源码
    // {
    //     let inner = self.inner()?;
    //
    //     // 如果引用计数等于0, 则返回一个None(这代表了weak并不会持有其所有权)
    //     if inner.strong() == 0 {
    //         None
    //     } else {
    //         unsafe {
    //             inner.inc_strong();
    //             Some(Rc::from_inner_in(self.ptr, self.alloc.clone()))
    //         }
    //     }
    // }
    {
        let origin = Rc::new(5);
        let weak_1 = Rc::downgrade(&origin);

        // 此时我们通过 upgrade 升级为Rc, 并获取对应的值
        // "使用 upgrade 时会对原始的 Rc 进行引用计数+1, 因为就是将 Weak 升级为 Rc", 在使用完之后Rc会自动Drop了
        let ret = weak_1.upgrade();
        assert_eq!(2, Rc::strong_count(&origin));
        assert_eq!(5, *ret.unwrap());
        assert_eq!(1, Rc::strong_count(&origin));

        // 此时我们手动 Drop 了原始值时, weak_1 会返回一个None
        drop(origin);
        assert_eq!(None, weak_1.upgrade());
    }

    // 3. 使用 Weak 解决循环引用
    {
        #[derive(Debug)]
        struct Owner {
            // 一个owner可以拥有多个tools
            tools: RefCell<Vec<Weak<Tool>>>,
            name: String,
        }

        #[derive(Debug)]
        struct Tool {
            owner: Rc<Owner>,
            name: String,
        }

        let tools_owner = Rc::new(Owner {
            tools: RefCell::new(Vec::new()),
            name: "z3".to_string(),
        });
        println!("example tools_owner : {:?}", tools_owner);

        let t1 = Rc::new(Tool {
            owner: tools_owner.clone(),
            name: "tool-1".to_string(),
        });
        let t2 = Rc::new(Tool {
            owner: tools_owner.clone(),
            name: "tool-2".to_string(),
        });
        tools_owner.tools.borrow_mut().push(Rc::downgrade(&t1));
        tools_owner.tools.borrow_mut().push(Rc::downgrade(&t2));

        // 因为 weak 指针不能保证他所引用的对象仍然存在, 所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)
        // 判断其所指向的对象是否存在。
        // 当然, Option 为 None 的时候这个引用原对象就不存在了。
        for weak in tools_owner.tools.borrow().iter() {
            if let Some(tool) = weak.upgrade() {
                println!("{} owner has tool: {}", &tools_owner.name, tool.name)
            }
        }
    }

    // 3.5 插曲
    // 为什么最佳实践中: 对于父子引用关系, 可以让"父节点通过 Rc 来引用子节点", 然后让"子节点通过 Weak 来引用父节点"
    /*
    原因如下:
    1.防止循环引用
    问题: 如果父节点和子节点都持有 Rc 引用, 会导致循环引用. 这样在数据结构不再使用时,引用计数不会降到零从而导致内存泄漏

    2.引用计数的正确性
    父子关系: 父节点通常是子节点的"逻辑所有者"! 通过使用 Weak 引用, 父节点可以在不拥有强引用的情况下引用子节点
            "这样可以确保父节点不会阻止子节点的释放(如果子节点想要释放, 发现还有一个所有权在父那里, 很不合理!)"
    子对父的引用: 子节点持有父节点的 Rc 引用, 这样子节点可以安全地访问父节点的属性和方法(能够保证永远可以访问到有效的父节点)

    3.设计上的清晰性
    逻辑关系: 这种设计清晰地表达了父子关系的拥有权: 父节点对所有子节点的引用"是非拥有的", 而子节点对父节点的引用"是拥有的"
    其实我们就将其理解为: 子节点是可以自由释放的, 父节点是不能自由释放的(父节点释放前需要释放所有子节点)
    */

    // 4. tree
    #[derive(Debug)]
    struct Node {
        val: i32,
        parent: Option<RefCell<Rc<Node>>>,
        left: RefCell<Weak<Node>>,
        right: RefCell<Weak<Node>>,
    }

    // parent node
    let p = Rc::new(Node {
        val: 1,
        parent: None,
        left: RefCell::new(Weak::new()),
        right: RefCell::new(Weak::new()),
    });

    // left and right chirl
    let child_1 = Rc::new(Node {
        val: 2,
        parent: Some(RefCell::new(p.clone())),
        left: RefCell::new(Weak::new()),
        right: RefCell::new(Weak::new()),
    });
    let child_2 = Rc::new(Node {
        val: 3,
        parent: Some(RefCell::new(p.clone())),
        left: RefCell::new(Weak::new()),
        right: RefCell::new(Weak::new()),
    });
    *p.left.borrow_mut() = Rc::downgrade(&child_1);
    *p.right.borrow_mut() = Rc::downgrade(&child_2);

    // 4. unsafe 解决循环引用
    /*
    除了使用Rust标准库提供的这些类型,你还可以使用 unsafe 里的"裸指针"来解决这些棘手的问题
    虽然 unsafe 不安全, 但是在各种库的代码中依然很常见用它来实现自引用结构, 主要优点如下:
    1.性能高, 毕竟直接用裸指针操作
    2. 代码更简单更符合直觉: 对比下 Option<Rc<RefCell<Node>>>
    */

    // 填一个坑:  RefCell<Rc> 和 Rc<RefCell> 区别是啥, 我们该如何选择
    {
        // 1. 如果我们想修改Rc指向的指针, 就需要使用 RefCell<Rc<>>
        let msg = String::from("hello,rust!");
        let rc = Rc::new(msg);
        println!("修改Rc指向的指针 {}", rc);
        let msg_mock = String::from("hello,rust,mock!");
        let rcell = RefCell::new(rc);
        *rcell.borrow_mut() = Rc::new(msg_mock);
        println!("修改Rc指向的指针 {}", rcell.borrow());

        // 2. 如果我们想修改Rc指向的指针的值, 就需要使用 Rc<RefCell<>>
        let msg = String::from("hello,rust!");
        let rc = Rc::new(RefCell::new(msg));
        println!("修改Rc指向的指针的值 {}", rc.borrow());
        rc.borrow_mut().push_str("xxx");
        println!("修改Rc指向的指针的值 {}", rc.borrow());
    }

    // 在智能指针套的比较多时, 我们需要 "从内到外" 进行逐步分析
    /*
        我现在的做法是:
        * 需要动态的修改结构体的这个字段的值, 那么你应该把这个值最先用Refcell定义然后再进一步考虑
        * 比如他可能是空的, 那我要用option包一下
        * 在遍历过程中需要临时变量去引用它, 那我就用Rc/Weak包一下
        直到满足最终的条件
    */
}
