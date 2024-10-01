use std::{collections::HashMap, hash::Hash};

pub fn _entry() {
    // HashMap, 每个程序员都不陌生的数据结构
    // 和动态数组vector一样, HashMap 也是Rust标准库中提供的集合类型.
    // 但是又与动态数组不同, HashMap中存储的是一一映射的 KV 键值对,并提供了平均复杂度为 O(1) 的查询方法.
    // 当我们希望通过一个 Key 去查询值时,该类型非常有用. 以致于 Go 语言将该类型设置成了语言级别的内置特性
    //
    // 所有的集合类型都是动态的, 意味着它们没有固定的内存大小.
    // 因此它们底层的数据都存储在"内存堆"上, 然后通过一个"存储在栈中的引用类型(我们可以理解为存储在栈上的指针)"来访问.
    // 同时, 跟其它集合类型一致, HashMap 也是内聚性的: 即所有的 K 必须拥有同样的类型, V 也是如此
    //
    // 知识点: 跟 Vec 一样, 如果预先知道要存储的 KV 对个数, 可以使用 HashMap::with_capacity(capacity) 创建指定大小的 HashMap
    // 避免频繁的内存分配和拷贝, 提升性能

    // (1). 创建一个hashmap(注意: 需要使用use将crate引入当前作用域中, hashMap并不在prelude中)
    // 这跟创建vec方法类似, 先new再insert
    // 注意: !!! "map里面的元素并不是有序存放的" !!!
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    println!("exmaple hashmap: {:?}", map);
    // 创建一个指定大小的hashMap, 如果超过容量则会自动进行内存扩容,数据拷贝操作
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(3);
    assert_eq!(map.capacity(), 3);
    map.insert(1, 2);
    assert_eq!(map.len(), 1);

    // (2). 使用迭代器和collect方式创建一个hashMap
    // 先将 Vec 转为迭代器, 接着通过 collect 方法, 将迭代器中的元素收集后转成 HashMap
    // 需要注意的是: collect 方法在内部实际上支持生成多种类型的目标集合, 因此我们需要通过类型标注 HashMap<_,_> 来告诉编译器
    // 请帮我们收集为 HashMap 集合类型, 具体的 KV 类型，麻烦编译器您老人家帮我们推导
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    println!("team_list_vec: {:?}", teams_list);
    // 我们需要提供类型标注, 告诉rust编译器应该为我们收集什么类型数据
    let m: HashMap<_, _> = teams_list.into_iter().collect();
    println!("team_list_map: {:?}", m);

    // (3). 所有权问题
    // HashMap 的所有权规则与其它 Rust 类型没有区别:
    // * 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
    // * 若没实现 Copy 特征，所有权将被Move给 HashMap 中
    // * 如果你使用引用类型放入 HashMap 中, 请确保该引用的生命周期至少跟 HashMap 活得一样久(不能在HashMap还没被回收前引用就被回收了, 这会导致野指针)
    let name: String = String::from("shyunn");
    let mut m: HashMap<&String, i32> = HashMap::new();
    m.insert(&name, 123);
    // println!("name: {name}");   // 此时不能使用name了, 因为其所有权已经被移动进hashMap中
    let nn = String::from("nn");
    m.insert(&nn, 1);
    // std::mem::drop(nn); // 不允许发生这种事情, 会导致野指针发生
    println!("m: {:?}", m);

    // (4). HashMap的查询
    // 通过get()方法即可以获取, 但是需要注意以下几点:
    // * get 方法返回一个 Option<&i32> 类型: 当查询不到时, 会返回一个 None, 查询到时返回 Some(&i32)
    // * &i32 是对HashMap中"值的借用", 如果不使用借用，可能会发生所有权的转移
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let val: Option<&i32> = scores.get(&team_name);
    match val {
        Some(val) => println!("val: {val}"),
        None => (),
    }
    // 我们还可以通过 copied + unwrap 方式快速获取值
    // + copied: copied() 方法会创建一个 Option 中值的副本,并返回一个新的 Option,其中包含的是原始值的副本(注意: 值需要实现Copy trait特征!)
    // unwrap: 返回Option::Some(val)中的 val值
    let val = scores.get(&team_name).copied().unwrap();
    println!("copy + unwrap get map val: {:?}", val);
    // 使用循环的方式遍历map
    for (k, v) in &scores {
        println!("haaaashmap: k={k} v={v}");
    }

    // (5). 更新hashMap中的值
    // insert 不仅会插入新的entry, 还会覆盖已有的entry
    let mut scores = HashMap::new();
    scores.insert("red", 10);
    scores.insert("blue", 20);
    scores.entry("green").or_insert(30); // 如果不存在green, 则插入新的k=green,v=30的entry
    scores.entry("red").and_modify(|x| *x = 11); // 如果存在red, 则对其val进行修改
    println!("scores: {:?}", scores);
    // example
    let line = "hello rust!!! hello";
    let mut word_map = HashMap::new();
    for ele in line.split_whitespace() {
        let count = word_map.entry(ele).or_insert(0);
        *count += 1; // 我们解引用后直接修改map中的值
    }
    println!("word map: {:#?}", word_map);

    // (6). hash函数
    // 你肯定比较好奇为何叫哈希表, 到底什么是哈希?
    // * 动态大小: 哈希表可以根据需要动态地增加或减少容量
    // * 哈希函数: 哈希函数是哈希表的核心, 它将键转换为数组的索引. 理想情况下哈希函数应该将键均匀地分布到数组中, 以减少冲突
    // * 快速查找: 哈希表的查找时间复杂度平均为O(1),即常数时间复杂度. 这意味着无论数据量有多大, 查找操作的时间几乎不变
    //
    // 哈希冲突（Hash Collision）是指在使用哈希表时, 不同的键（Key）通过哈希函数计算得到相同的哈希值（Hash Value）
    // 进而映射到同一个存储位置（数组索引或哈希桶）的情况.
    // 由于哈希表的大小通常是有限的, 而可能的键的数量可能是无限的, 因此冲突是不可避免的
    // NOTE: 我们可以使用复杂的hash算法(开放寻址,再哈希法...)来解决, 最好是引入外部包解决.
}
