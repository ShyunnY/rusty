use std::collections::HashMap;

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
    // 跟 Vec 一样, 如果预先知道要存储的 KV 对个数, 可以使用 HashMap::with_capacity(capacity) 创建指定大小的 HashMap
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
}
