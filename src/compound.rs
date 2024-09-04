
pub fn _entry(){
    _tuple();
    _struct();
}

fn _struct(){
    // 1). 创建一个结构体
    //
    // 初始化实例时, 每个字段都需要进行初始化
    // 初始化时的字段顺序不需要和结构体定义时的顺序一致
    let mut u1 = User{
        active: true,
        username: String::from("张三"),
        email: String::from("123@wx.com"),
        sign_in_count: 1,
    };
    println!("[Create]: {:?}",u1);

    // 2). 通过 '.' 访问结构体实例的字段
    // 必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变
    // 要不就一起 "可变", 要不就一起 "不变"
    u1.username = String::from("李四");
    println!("[Read and Mut]: {:?}",u1);

    // 3). 根据已有的 struct 构建新的 "结构体实例"
    //
    // 实现了Copy特征的类型无需所有权转移, 可以直接在赋值时进行"数据拷贝",
    // 其中 bool 和 u64 类型就实现了Copy特征,
    // 因此 active 和 sign_in_count 字段在赋值给 user2 时, 仅仅发生了拷贝,而不是所有权转移.
    // 而 username 和 email 具备所有权, 在赋值的时候转移给了u2, 所以u1无法用 u1.username,u1.email 了(但u1的其他实现了copy特征的数据类型还是可以使用)
    let _u2 = User{
        active: u1.active,
        ..u1    // u1中具有所有权的数据类型被 move 到了u2中, 此时只能使用u1中不具备所有权的数据
    };
    println!("[Read and Mut]: {:?}",u1.active);
}

fn _build_user(usr_name: String,email: String) -> User{
    // 简化 结构体struct 的构建
    User{
        active: true,
        username: usr_name,
        email,  // "当函数参数和结构体字段同名时", 可以直接使用缩略的方式进行初始化
        sign_in_count: 1,
    }
}
fn _tuple(){
    // 1. 声明一个简单的元组
    let t = (1,"2",true);
    println!("Decl Tuple: {:?}",t);

    // 2. 模式匹配解构元组
    let (x,y,z) = t;
    println!("Match Tuple x: {x} y: {y} z: {z}");

    // 3. 用 ".' 访问元组
    let xx = t.0;
    let yy = t.1;
    let  zz = t.2;
    println!("Dot Match Tuple x: {xx} y: {yy} z: {zz}");

    let s = String::from("hello,rust");
    let (_,length) = _tuple_example(s);
    println!("s length: {length}");
}

fn _tuple_example(s: String) -> (String,usize){
    let length = s.len();

    (s,length)  // 注意不能写成 "(s,s.len())", 因为在 s.len() 之前 s 的所有权已经交给了这个元组, 所以s已经是不能使用了(s不拥有原有的内存地址了)
}

// 类型定义区间 ==================

#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}