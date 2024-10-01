use std::fmt::Debug;

pub fn _entry() {
    // Vec 本质上就是一个动态数组
    // 动态数组允许你存储多个值, 这些值在内存中一个紧挨着另一个排列,因此访问其中某个元素的成本非常低
    // 动态数组只能存储相同类型的元素,如果你想存储不同类型的元素,可以使用之前讲过的"枚举类型或者特征对象"

    // (1). 创建和更新vec
    _create_and_update_vec();

    // (2). vec的生命周期
    // vec实际上是一个实现"Move trait"的对象,其也会具有生命周期
    // Vector类型在超出作用域范围后, 会被自动调用Drop删除
    // 但是,但是,但是!
    // 当 Vector 被删除后, 它 "内部存储的所有内容也会随之被删除"
    // 目前来看这种解决方案简单直白, 但是当 Vector 中的元素被引用后, 事情可能会没那么简单
    {
        let vc = vec![1, 1, 1];
        println!("in scope vector: {:?}", vc);
    } // 超出scope了, 此时vec被Drop了, 我们无法使用了

    // (3). 如何读取一个vec?
    // 读取指定位置的元素有两种方式可选:
    // * 通过下标索引访问. (这会直接返回该元素的Copy副本或者Move所有权, 由于vec是动态增长的, 这可能会导致溢出)
    // * 使用 get 方法. (这会返回一个`Option`包装的元素, 即使溢出我们也可以根据模式匹配确保不会panic)
    //
    // 当我们确保索引不会越界的时候, 就用索引访问否则请用 get
    let vc = vec![1, 2, 3, 4, 9];
    let num_1 = vc[1]; // 通过索引访问
    println!("number_1: {num_1}");
    if let Some(val) = vc.get(4) {
        println!("number_2: {val}");
    }

    // (4). 同时借用多个数组元素
    // 对数组的不可变借用发生在可变借用前面, 将是不被允许的.
    //
    // 本质上就是存在野指针的问题
    // 原因在于: 数组的大小是可变的,当旧数组的大小不够用时, Rust 会重新分配一块更大的内存空间
    // 然后把旧数组拷贝过来. 这种情况下: 之前的引用显然会指向一块无效的内存, 这非常 rusty
    let mut vc = vec![99, 88, 77, 11];
    let _first = &vc[0]; // 此处进行 "不可变借用"
    vc.push(1); // 这里进行了可变借用, 如果我们在下面再使用first, 这将是不被允许的

    // (5). 迭代遍历vec所有元素
    // NOTE: 可以使用 &vc 声明在迭代过程是不可变的借用
    // 使用迭代的方式去遍历数组,这种方式比用下标的方式去遍历数组更安全也更高效（每次下标访问都会触发数组边界检查）
    for ele in &vc {
        println!("vc ele: {ele}");
    }
    // 当然也可以在遍历过程中进行更改
    for ele in &mut vc {
        *ele = *ele + 1;
        println!("mut vc ele: {ele}")
    }

    // (5). 存储不同类型
    _diff_ele_in_vec();

    // (6). vec常见方法
    _common_method();

    // (7). vector的排序
    // 在 rust 里实现了两种排序算法, 分别为稳定的排序 sort 和 sort_by, 以及非稳定排序 sort_unstable 和 sort_unstable_by
    // 当然，这个所谓的"非稳定"并不是指排序算法本身不稳定, 而是指在排序过程中对相等元素的处理方式
    // 在"稳定"排序算法里, "对相等的元素不会对其进行重新排序". 而在"不稳定"的算法里则不保证这点。
    // 总体而言，"非稳定"排序的算法的速度会优于"稳定"排序算法, 同时, "稳定"排序还会额外分配原数组一半的空间
    //
    let mut vc = vec![9, 4, 1, 3, 6, 0];
    vc.sort();
    println!("stable sort i32: {:?}", vc);

    // 在浮点数当中, 存在一个 "NAN" 的值. 这个值无法与其他的浮点数进行对比.
    // 因此浮点数类型并没有实现"全数值可比较Ord的trait", 而是实现了"部分可比较的特性PartialOrd的trait"
    let mut vc: Vec<f64> = vec![1.2, 0.9, 0.0001, 0.33];
    vc.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); // 我们可以手动指明部分比较API
    println!("unstable sort f64: {:?}", vc);

    // 结构体排序
    // "排序需要我们实现 Ord 特性", 那么如果我们把我们的结构体实现了该特性, 是否就不需要我们自定义对比函数了呢？
    // 是,但不完全是. 实现 Ord 需要我们实现 "Ord、Eq、PartialEq、PartialOrd" 这些属性.
    // 好消息是: 你可以`derive`这些属性(需要确保你的结构体中所有的属性均实现了 "Ord" 相关特性, 否则会发生编译错误)
    //
    // 知识点: 使用derive可以帮助自定义类型实现常见的trait,
    let mut vc = vec![
        Person {
            name: String::from("张三"),
            age: 20,
        },
        Person {
            name: String::from("王五"),
            age: 19,
        },
        Person {
            name: String::from("李四"),
            age: 22,
        },
    ];
    vc.sort_unstable_by(|pre: &Person, post: &Person| pre.age.cmp(&post.age));
    println!("unstable sort struct: {:?}", vc);
}

fn _common_method() {
    // 动态数组意味着我们增加元素时, 如果容量不足就会导致 vector 扩容（
    // 目前的策略是重新申请一块 2 倍大小的内存, 再将"所有元素拷贝到新的内存位置", 同时更新指针数据）
    // 显然, 当频繁扩容或者当元素数量较多且需要扩容时, 大量的内存拷贝会降低程序的性能
    // 同时我们需要注意, 如果我们持有不可变引用时, 是不能对vec中的元素进行操作, 就是为了避免这种情况

    // 1. 创建一个默认值为0, 初始长度为3, 容量为3的vector
    let vec = vec![0; 3];
    println!(
        "vec1: {:?}, vec len: {}, vec cap: {}",
        vec,
        vec.len(),
        vec.capacity()
    );

    // 2. 对vec进行容量操作
    let mut vc: Vec<i32> = Vec::with_capacity(3);
    vc.extend([99, 88]); // 将一个vector附加到vc上
    println!(
        "vec2: {:?}, vec len: {}, vec cap: {}",
        vc,
        vc.len(),
        vc.capacity()
    );
    // 对vc进行扩容
    vc.reserve(10); // reserve = vc.len() + additional
    _print_vec(&vc);
    // 对vc进行缩容(释放掉未使用的空间)
    vc.shrink_to_fit();
    _print_vec(&vc);

    // 3. vec的常用方法
    let mut vv: Vec<i32> = vec![1, 2, 3, 4];
    let empty: bool = vv.is_empty(); // 判读vec是否为空
    assert_eq!(!empty, true);
    vv.insert(1, 222); // 在指定位置插入值. 注意: index > vec.len() 将会panic!
    assert_eq!(vv, [1, 222, 2, 3, 4]);
    let ret = vv.remove(0); // 移除并返回指定位置的值(超出数组边界将会panic!)
    assert_eq!(ret, 1);
    assert_eq!(vv, [222, 2, 3, 4]);
    if let Some(val) = vv.pop() {
        // 移除并返回vec末尾的值(返回一个Option)
        assert_eq!(val, 4);
        assert_eq!(vv, [222, 2, 3]);
    }
    vv.append(&mut vec![11, 2, 3]); // 将一个vec追加到另一个vec的末尾
    println!("vv append: {:?}", vv);
    vv.retain(|x| x % 2 == 0); // 保留指定条件的值
    println!("vv retain: {:?}", vv);

    // 当然, 我们也可以像获取数组切片一样获取vec的切片
    let vc: Vec<i32> = vec![9, 5, 2, 7];
    let vc_ref = &vc[0..=vc.len() - 2]; // 获取vector的切片
    println!("vc_ref: {:?}", vc_ref);
}

fn _print_vec<T: Debug>(vc: &Vec<T>) {
    println!(
        "vec: {:?}, vec len: {}, vec cap: {}",
        vc,
        vc.len(),
        vc.capacity()
    )
}

fn _create_and_update_vec() {
    // 1. 使用 Vec::new() 创建一个 vector
    // 使用 Vec::new 创建动态数组是最 rusty 的方式, 它调用了 Vec 中的 new 关联函数
    // 但是在vector.push之前, rust无法得知vec的类型是什么, 我们尽量养成提前声明类型的习惯
    //
    // 如果"预先知道要存储的元素个数", 可以使用 Vec::with_capacity(capacity) 创建动态数组,
    // 这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝, 提升性能
    let mut vc = Vec::new();
    vc.push(1);
    println!("create vc: {:?}", vc);

    // 2. 使用 vec! 宏创建一个 vector
    let mut vch = vec![1, 2, 3, 4, 5, 99, 520];
    println!("create vch: {:?}", vch);

    // 3. 更新vec中的值(追加一个新的值)
    // 与其它类型一样, 必须将v声明为`mut`后才能进行修改
    vch.push(999);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: i32,
}

fn _diff_ele_in_vec() {
    // vec的元素必须类型相同, 这是个共识. 但是我们如果希望存放不同的类型应该咋做呢?
    // 当然是: 使用枚举或者特征对象

    // 1.枚举(类型都是枚举, 然后内部是使用不同的类型)
    let enum_vec = vec![
        IPAddr::V4(String::from("127.0.0.1")),
        IPAddr::V6(String::from("::1")),
    ];
    println!("enum vector: {:?}", enum_vec);

    // 2.特征对象
    // 在实际使用场景中, 特征对象数组要比枚举数组常见很多.
    // 主要原因在于特征对象非常灵活, 而编译器对枚举的限制较多, 且无法动态增加类型
    // &dyn Human引用特征对象方式
    // 注意: 使用for遍历Move特征的类型时, 我们需要分清楚我们是否需要进行借用操作
    let traitobj_vec: Vec<&dyn Human> = vec![&woman, &man];
    for ele in traitobj_vec {
        ele.say();
    }

    // Box<dyn Human> 智能指针管理方式
    let traitobj_vec: Vec<Box<dyn Human>> = vec![Box::new(man), Box::new(woman)];
    for ele in &traitobj_vec {
        ele.say();
    }
    println!("{:?}", traitobj_vec);
}

trait Human: Debug {
    fn say(&self);
}

#[derive(Debug)]
struct man;

impl Human for man {
    fn say(&self) {
        println!("man!")
    }
}

#[derive(Debug)]
struct woman;

impl Human for woman {
    fn say(&self) {
        println!("woman!")
    }
}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}
