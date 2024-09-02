pub fn _var(){
    // 命名变量
    let num = "i32";
    println!("命名变量: {:?}",num);

    // 变量绑定
    let bind = "val";
    println!("当前值 {:?} 被绑定到 bind 变量上",bind);

    // 可变变量
    let mut mutating = "1";
    println!("变化前: {:?}",mutating);
    mutating = "2";
    println!("变化后: {:?}", mutating);

    // 忽略未使用的变量
    let _unused: i32;

    // 变量解构
    let (name,age) = ("z3",20);
    println!("变量解构: name={name} age={age}");

    // 变量遮蔽(实际上进行了 "内存再分配", 同名变量覆盖)
    let num = 123;
    println!("变量遮蔽前: {num}");
    let num = "json!";
    println!("变量遮蔽后: {num}");

    // 常量, 在编译期完成后确定(rust中常量使用蛇形大写)
    const MAX_POINTS: u8 = 100;
    println!("常量最大分数: {MAX_POINTS}");
}