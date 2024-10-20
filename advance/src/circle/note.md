## 循环引用

在使用 `RefCell` + `Rc` 时容易导致循环引用(因为计数器无法归零, 导致内存泄漏)

```rust
fn bad() {
    #[derive(Debug)]
    struct Owner {
        // 一个owner可以拥有多个tools
        tools: RefCell<Vec<Rc<Tool>>>,
        name: String,
    }

    #[derive(Debug)]
    struct Tool {
        owner: Rc<Owner>,
        name: String,
    }

    let z3 = Rc::new(Owner {
        tools: RefCell::new(Vec::new()),
        name: "z3".to_string(),
    });
    let l4 = Rc::new(Owner {
        tools: RefCell::new(Vec::new()),
        name: "l4".to_string(),
    });
    println!("example owner z3: {:?}", z3);
    println!("example owner z3: {:?}", l4);

    let t1 = Rc::new(Tool {
        owner: z3.clone(),
        name: "tool-1".to_string(),
    });
    let t2 = Rc::new(Tool {
        owner: l4.clone(),
        name: "tool-2".to_string(),
    });

    // 这里发生了循环引用
    // 最后z3,l4,t1,t2被Drop了, 但是他们在堆上的数据却相互持有所有权, 导致无法被清理
    z3.tools.borrow_mut().push(t1.clone());
    l4.tools.borrow_mut().push(t2.clone());

    // println!("example tool t-1: {:?}", t1);
    // println!("example tool t-2: {:?}", t2);
}
```

我们可以使用 `Weak` 弱引用来解决引用循环问题:

```rust
fn good() {
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
```