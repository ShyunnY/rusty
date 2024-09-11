pub fn _entry() {
    // (1) if/else
    // * if,else 语句块是表达式, 我们可以使用 if,else 表达式的返回值来给" 变量 "进行赋值
    // * 用 if,else 来赋值时，要保证 `每个分支返回的类型一样`
    let condition: bool = true;
    let val: i32 = if condition {
        1 // 注意: 需要返回表达式
    } else {
        2 // 注意: 需要返回表达式
    };
    dbg!(val);
    if val == 2 {
        dbg!("2");
    } else if val == 1 {
        dbg!("1");
    }

    // (2) for
    // for循环是rust的大杀器
    // [语法: for 元素 in 集合]
    // 注意: 使用 for 时我们往往使用集合的` 引用 `形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）
    // 如果不使用引用的话, 所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了 ):
    //
    // NOTE: 对于实现了 copy 特征的数组(例如 [i32; 10] )而言, for item in arr 并不会把 arr 的所有权转移, 而是直接对其进行了拷贝
    // 因此循环之后仍然可以使用 arr
    //
    // 1). for item in collection [转移所有权]
    // 2). for item in &collection [不可变的借用]
    // 3). for item in &mut collection [可变借用]
    let collection: [i32; 5] = [5, 4, 3, 2, 1];
    // 第一种使用方式中 collection[index] 的索引访问, 会因为边界检查(Bounds Checking)导致运行时的性能损耗
    // —— Rust 会检查并确认 index 是否落在集合内
    // 第一种方式里对 collection 的索引访问是非连续的, 存在一定可能性在两次访问之间, collection 发生了变化, 导致脏数据产生
    for i in 0..collection.len() {
        println!("insecure: {}", collection[i]);
    }
    // 第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法
    // 直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)
    for ele in collection {
        println!("secure: {}", ele);
    }

    // (3) continue
    for ele in 0..4 {
        if ele == 2 {
            continue;
        }
        println!("continue: {}", ele);
    }

    // (4) while and loop
    let mut n = 0;
    while n <= 3 {
        println!("in while: {n}");
        n += 1;
    }
    println!("i'm coming!");
    // loop 是无条件循环, 必须手动指明退出的点
    // * loop "是一个表达式, 可以返回一个值"
    // * break 可以单独使用, 也可以"带一个返回值"
    let ret: i32 = loop {
        if n == 7 {
            break n;
        }
        println!("in loop: {n}");
        n += 1;
    };
    println!("i;m coming2, ret={ret}");
    
    // 最佳实践:
    // 1.遍历一个集合时, 我们最好使用迭代器去方法. 通过索引的方式会引入不必要的 'runtime边界检查'.
    // 2.直接使用迭代器可以在编译期就确保访问是有效的.
    let demo = [55,44,33,22,11];
    for (i,ele) in demo.iter().enumerate(){
        println!("iter index={i} ele={ele}");
    }

    // 通过 labels 方式跳出嵌套循环
    'outer: for _ in 0..3{
        for _ in 0..2{
            break 'outer;   // 直接break跳到标签定义的外层
        }
    }

    // 以下代码中, bor对s进行了 不可变借用, 同时 ss 想解引用获取s, 这是不行的.
    // 因为对bor解引用实际上会使得bor调用s的COPY trait, 也就是说在这整个过程中 所有权都不会移动
    // ** s 的所有权始终保持在变量 s 上, 并且 bor 只是一个对 s 的引用, ss 是 s 的拷贝的拥有者 **
    // let s = String::from("This");
    // let bor = &s;
    // let ss = *bor;

}
