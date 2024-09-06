use std::arch::x86_64::__get_cpuid_max;
use std::process::abort;

pub fn _entry(){
    // _tuple();
    // _struct();
    // _slice();
    _string();
}
fn _string(){
    // 字符串是由字符组成的连续集合
    // Rust 在语言级别, 只有一种字符串类型: str 它通常是以引用类型出现 &str
    // String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串
    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码

    // 1). &str转换为String
    let str_to_string = String::from("hello,rust");
    let str_to_string1 = "hello,golang".to_string();
    println!("str to String: {str_to_string}");
    println!("str to String1: {str_to_string1}");

    // 2). String转换为&str    (本质上是取引用, 实际上这种灵活用法是因为 deref 隐式强制转换)
    let string_to_str = String::from("hello,python");
    greet_string(&string_to_str);
    greet_string(&string_to_str[6..]);

    // 3). 字符串索引和字符串切片注意
    // 在rust中, 由于字符串使用unicode类型, utf-8编码, 不能确定 "s[1] "一定能落在字符串内部字符的边界
    // 因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符
    //
    // 字符串切片是非常危险的操作，因为切片的索引是通过字节来进行，但是字符串又是 UTF-8 编码，因此你无法保证索引的字节刚好落在字符的边界上
    // let hello = "中国人";
    // let s = &hello[0..2];    // 不能确保落在“中”的边界
    // 在通过索引区间来访问字符串时，需要格外的小心

    // 4). 操作字符串.  由于 String 是可变字符串, 下面对 Rust 字符串进行修改，添加，删除等常用方法
    // 4.1) 追加(push)
    let mut line = String::from("hello,rust");
    println!("push before: {line}");
    line.push_str("!!!");
    println!("push after: {line}");

    // 4.2) 插入(insert)
    line.insert_str(6," my ");
    println!("insert after: {line}");

    // 4.3) 替换(replace)
    // replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。
    // 该方法会替换 "所有匹配到" 的字符串。"该方法是返回一个新的字符串，而不是操作原来的字符串"
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust","RUST");
    dbg!(new_string_replace);
    // replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
    // "该方法是返回一个新的字符串, 而不是操作原来的字符串"
    let string_replacen = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replacen = string_replacen.replacen("rust","RUsT",1);
    dbg!(new_string_replacen);
    // replace_range() 接收两个参数: 第一个参数是要替换字符串的范围（Range）, 第二个参数是新的字符串
    // 该方法是直接 "操作原来的字符串", 不会返回新的字符串。该方法需要使用 mut 关键字修饰
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8,"R");
    dbg!(string_replace_range);

    // 4.4) 删除(Delete)
    // pop() 删除并返回字符串的最后一个字符. (这是操作原始字符串的, 需要字符串可变)
    // 存在返回值, 其返回值是一个 Option 类型. 如果字符串为空, 则返回 None. 如果不为空, 则返回删除的最后一个字符
    let mut string_pop = String::from("golang@");
    let ret1= string_pop.pop();
    dbg!(ret1);
    let ret2 = string_pop.pop();
    dbg!(ret2);
    dbg!(string_pop);
    // remove() 删除并返回字符串中指定位置的字符. (这是操作原始字符串的, 需要字符串可变)
    // 返回值是删除位置的字符串. 只接收一个参数表示该字符起始索引位置.
    // remove() 方法是按照字节来处理字符串的, 如果参数所给的位置不是合法的字符边界, 则会发生错误
    let mut string_remove = String::from("!golang@");
    let ret3 = string_remove.remove(0);
    dbg!(ret3);
    dbg!(string_remove);
    // truncate()  删除字符串中从指定位置开始到结尾的全部字符
    // 无返回值. 该方法 truncate() 方法是按照字节来处理字符串的, 如果参数所给的位置不是合法的字符边界
    let mut string_truncate = String::from("abcd");
    string_truncate.truncate(2);
    dbg!(string_truncate);
    // clear() 清空字符串
    // 调用后, 将会删除字符串中的所有字符, 相当于 truncate() 方法参数为 0 的时候
    let mut string_clear = String::from("string is clear!");
    string_clear.clear();
    dbg!(string_clear);

    // 4.5) 连接(Connect)
    // 使用 "+" 或者 "+=" 连接字符串, 要求右边的参数必须为字符串的切片引用（Slice）类型.
    // 其实当调用 "+" 的操作符时, 相当于调用了 std::string 标准库中的 add() 方法,
    // 这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 + 时, "必须传递切片引用类型"
    // 不能直接传递 String 类型. "+ 是返回一个新的字符串(并不需要在原始字符串上修改), 所以变量声明可以不需要 mut 关键字修饰"
    let string_head = String::from("hello,");
    let string_tail = String::from("HELL");

    // 因为 "+" 实际上是 add(self, &str) 特征, 调用的时候所有权以及传递进add了, 后续我们就不能再用了
    // **string_head 在 add() 方法调用后就被释放了**
    //
    // 后面我们学习方法的时候就知道了:  xxx(self), 代表调用结构体的这个方法后, 结构体会在该方法内被销毁(所有权转移进去了).
    let string_add = string_head + &string_tail;
    dbg!(string_add);
    // println!("{string_head}");   // 这里不能再用 string_head 了, 他的所有权已经被转移走了
    //
    // format! 宏拼接字符串
    dbg!(format!("{}{}","hello,"," G.E.M."));
}

fn greet_string(s: &str){
    println!("string to str: {s}")
}

fn _slice(){
    // 切片的本质就是: "持有一个引用指向原始数组的某个元素和长度"
    // 注意: 切片本质上是进行了"借用"!!!, 如果切片指向的原始类型进行了可变, 那么这会违反 "不可变借用和可变借用同时存在" 的问题
    //
    // 例如以下:
    // line自身是可变的, s借用了line的[0,5]共5长度的字符, 而且s属于不可变借用.
    // 此时在最后一次使用s前再次修改line, 将无法编译
    let mut line = String::from("hello,rust");
    let _s = &line[0..5];
    // line.clear();   // 无法编译, 违反 "不可变借用和可变借用同时存在" 的问题
    let s = &line[..line.len()];
    println!("string slice: {s}");
    line.clear();   // 可以编译, 因为已经在最后一次s使用之后(超出了其借用作用域)

    // 字符串字面量切片
    // 该切片指向了程序可执行文件中的某个点, 这也是为什么字符串字面量是不可变的, 因为 &str 是一个不可变引用
    // 实际上这部分字面量的生命周期与程序相同, 我们可以理解为在程序运行时, 程序的某个地方就存放着这个字面量值
    // **"str 类型是硬编码进可执行文件，也无法被修改"**
    let ss: &str = "Hello,golang";
    println!("ss: {ss}");
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
    // 而 username 和 email 具备所有权, 在赋值的时候转移给了u2,
    // 所以u1无法用 u1.username,u1.email 了(但u1的其他实现了copy特征的数据类型还是可以使用)
    //
    // 我们可以理解为: 我的儿子拥有了玩具, 我拥有了我儿子, 那么我肯定拥有了我儿子的玩具
    let _u2 = User{
        active: u1.active,
        // 简短的赋值语法糖
        ..u1    // u1中具有所有权的数据类型被 move 到了u2中, 此时只能使用u1中不具备所有权的数据
    };
    println!("[Read and Mut]: {:?}",u1.active);

    // 4). 元组结构体(结构体有名称, 但是结构体的字段没有名称)
    let color = Color(1,2,3);
    println!("[tuple struct]: {:?}",color);
    dbg!(&color); // dbg 默认会拿走所有权再返回出来, 如果我们不希望其拿走所有权, 则我们传递一个借用
    println!("[tuple struct]: {:?}",color);

    // 5). 在函数内部创建一个 struct, 然后返回给调用者。 实际上这个struct还是存储在栈上, rust会自动处理struct的拷贝
    // 在返回的时候所有权交给调用者, 这部分会进行结构体拷贝（因为整个过程中 struct 都是存在栈上, 栈上分配效率高）
    // 如果我们想使其存在堆上, 我们会需要智能指针
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
    username: String,   // 这代表User拥有了其字段的所有权, 当然我们也可以借用其他数据, 此时需要声明 "生命周期".
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);


// 单元结构体
// 还记得之前讲过的基本没啥用的单元类型吧？单元结构体就跟它很像，没有任何字段和属性，但是好在，它还挺有用。
//
// 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：
struct UnitStruct;

// 我们不在意内容, 只在意行为
// impl Clone for UnitStruct {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
//
// impl Copy for UnitStruct{
//
// }