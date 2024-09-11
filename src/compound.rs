pub fn _entry() {
    // _tuple();
    // _struct();
    // _slice();
    // _string();
    // _enum();
    // _array();
}

fn _array() {
    // 数组: 数组是长度固定, 所有元素类型相同, 线性排列的数据结构, 并且内存分配在栈上
    // vector: 与数组类似, 但是长度不固定, 可动态增长, 并且内存分配在堆上

    // 数组类型是通过方括号语法声明, i32 是元素类型, 分号后面的数字 5 是数组长度, 组类型也从侧面说明了数组的元素类型要统一, 长度要固定
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);

    // 始化一个某个值重复出现 N 次的数组
    // [3;5] -> [类型;长度] -> [3,3,3,3,3]
    let arr_1 = [3; 5];
    println!("arr1: {:?}", arr_1);

    // 访问数组元素
    // 当你尝试使用索引访问元素时, Rust 将检查你 `指定的索引是否 "小于 " 数组长度`.
    // 如果索引大于或等于数组长度，Rust 会出现 `panic`. 这种检查只能在运行时进行
    let index = 3;
    let num = arr[index];
    println!("index={index}, ele={num}");

    // 越界访问数组元素
    // let num1 = arr[index +3];
    // println!("index={index}, ele={num1}");

    // 非基本数据类型的拷贝
    // let array=[3;5]底层就是不断的Copy(深拷贝)出来的，但很可惜复杂类型都没有深拷贝. 并且基本类型在Rust中赋值是以Copy的形式存在, 复杂类型只能是move(浅拷贝)
    // let arr_string = [String::from("hello");5];
    //
    // 但是我们可以这样写
    let arr_string: [String; 5] = std::array::from_fn(|_i| String::from("Hello,Mr.!"));
    println!("arr_string: {:?}", arr_string);

    // 数组切片
    // 切片概念: 允许你引用集合中的 `部分连续片段，而不是整个集合` 对于数组也是, 数组切片允许我们引用数组的一部分
    // 切片可以是 可变/不可变的, 简而易见: 切片起始就是借用某个数组的某一段连续区间
    let mut a: [i32; 5] = [11, 22, 33, 44, 55];
    for ele in a {
        println!("ele: {ele}");
    }
    let slice = &mut a[2..5];
    // let _s = a[1..3]; // 这种是错误的, 因为不使用借用, 导致无法知道其长度, 没有类型长度. (切片是动态的, 我们需要通过引用获取到其长度)
    slice[0] = 1;
    println!("slice: {:?}", slice);

    // 字符串切片同理
    let b = String::from("abcdefg");
    let b_slice = &b[1..2];
    println!("slice: {:?}", b_slice);

    // 总结下切片的特点:
    // * 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时 `指定的起始和结束位置`
    // * 创建切片的代价非常小，因为切片只是针对底层数组的一个`引用`(我们也可以理解为借用)
    // * 切片类型[T]拥有不固定的大小，而 `切片引用类型&[T]则具有固定的大小`，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理

    // 最后总结一下:
    // 1). 数组类型容易跟数组切片混淆: [T;n]描述了一个数组的类型, 而[T]描述了切片的类型, 因为切片是运行期的数据结构, 它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
    // 2). [u8; 3]和[u8; 4]是不同的类型, " 数组的长度也是类型的一部分 "
    // 3). 在实际开发中, 使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小


    // 解释一下这个问题
    // Q: 为什么使用切片的时候需要对切片添加 '&' 进行引用？
    // A: 使用切片时需要添加 & 引用是因为切片本质上是对`数据的一种引用`, 切片（slice）是对一块数组或其他数据结构的引用, 它表示数据的一部分。
    // "切片本身并不拥有数据", 它只是一个对数据(对数组中的部分数据)的借用.
    // 因此, 切片的类型是一个引用类型, 具体来说是 &[T] 或 &mut [T].
    //
    // 1). 引用的创建: & 符号用于创建对某个值的引用. 在数组切片的情况下, 这个引用是对数组的一部分的借用。
    // 2). 所有权管理: 通过使用 & 创建引用，Rust 可以确保 `切片借用的数据在切片使用期间保持有效`(在借用期间不会被其他人更改), 并且遵循所有权和借用规则.
    // **总结: 切片是对数据的引用, 使用 & 符号是为了创建这个引用. 切片借用数据而不拥有它，这确保了切片能够安全地访问数据**

    // let gem = [1,2,3,4];
    // let g = &gem[..gem.len()];   // OK 通过
    // let e = gem[..gem.len()];    // Error 报错
}

fn _enum() {
    // 枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型, 例如扑克牌花色(枚举值只可能是其中某一个成员)
    // "枚举类型是一个类型",它会包含所有可能的枚举成员.  枚举值是该类型中的具体 "某个成员的实例 "

    let fe_male = Gender::FeMale; // 通过
    let male = Gender::Male;
    println!("fe_male: {:?}", fe_male);
    println!("male: {:?}", male);

    _print_gender(fe_male);
    _print_gender(male);

    // 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举(可以进行无限套娃)
    // 由于每个结构体都有自己的类型, 因此我们无法在需要同一类型的地方进行使用: 例如某个函数它的功能是接受消息并进行发送,
    // 那么用枚举的方式，就可以接收不同的消息. "但是用结构体, 该函数无法接受 4 个不同的结构体作为参数。"
    // 而且从代码规范角度来看，枚举的实现更简洁，代码 "内聚性更强 " ，不像结构体的实现，分散在各个地方。
    let people = Animal::People(String::from("张三"));
    let d = Animal::Dog(1);
    println!("enum people: {:?}", people);
    println!("enum d: {:?}", d);

    // 可以通过匹配模式来结构出 enum 中的值
    match people {
        Animal::People(val) => {
            println!("people name: {val}")
        }
        _ => {}
    }

    // Option枚举
    // Option 枚举包含两个成员: 一个成员表示含有值：Some(T), 另一个表示没有值：None
    // 其中 T 是泛型参数，Some(T)表示该枚举成员的数据类型是 T, 换句话说，Some 可以包含任何类型的数据
    // 1). 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
    // 2). 当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值
    //
    // 换句话说:
    // * 如果一个值不是 Option<T> 类型, 我们可以认为这个值肯定不为空
    // * 如果一个值是 Option<T> 类型, 代表我们会处理该值在 None 下的情况.(也就是说, 会强制要求我们通过match处理None时的情况)
    //
    // 我们能确保永远不会出现空值: 一个值用Option包裹处理, 要不就是 Some<T>, 要不就是 None. 无论如何都不会出现 "null" 的情况
    //
    // null 如果有值, 一切正常;   null 如果没值, 直接崩溃.
    // Option 如果有值, 一切正常; Option 如果没值, 通过 match -> None 抛给用户处理
    let _some_none: Option<String> = None;
}

fn _print_gender(gender: Gender) {
    println!("print gender: {:?}", gender);
}

// ==== enum define ====

// 任何类型的数据都可以放入枚举成员中
#[derive(Debug)]
enum Animal {
    People(String),
    Dog(u8),
}

#[derive(Debug)]
enum Gender {
    FeMale,
    Male,
}

fn _string() {
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
    _greet_string(&string_to_str);
    _greet_string(&string_to_str[6..]);

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
    line.insert_str(6, " my ");
    println!("insert after: {line}");

    // 4.3) 替换(replace)
    // replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。
    // 该方法会替换 "所有匹配到" 的字符串。"该方法是返回一个新的字符串，而不是操作原来的字符串"
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    // replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
    // "该方法是返回一个新的字符串, 而不是操作原来的字符串"
    let string_replacen = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replacen = string_replacen.replacen("rust", "RUsT", 1);
    dbg!(new_string_replacen);
    // replace_range() 接收两个参数: 第一个参数是要替换字符串的范围（Range）, 第二个参数是新的字符串
    // 该方法是直接 "操作原来的字符串", 不会返回新的字符串。该方法需要使用 mut 关键字修饰
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 4.4) 删除(Delete)
    // pop() 删除并返回字符串的最后一个字符. (这是操作原始字符串的, 需要字符串可变)
    // 存在返回值, 其返回值是一个 Option 类型. 如果字符串为空, 则返回 None. 如果不为空, 则返回删除的最后一个字符
    let mut string_pop = String::from("golang@");
    let ret1 = string_pop.pop();
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
    dbg!(format!("{}{}", "hello,", " G.E.M."));

    // 5.1 字符串转义
    let transfer_1 = "i'm \"gloria!\"";
    dbg!(transfer_1);
    // 5.2 字符串不转义, 也就是要求按照字面量值进行操作
    let transfer_2: &str = r"i'm \gloria!\";
    dbg!(transfer_2);
    // 5.3 字符串包含双引号
    let transfer_3 = r#" "hello,everybody!" "#;
    println!("{transfer_3}");

    // 6. 操作 UTF-8 字符串

    // 字符
    // 6.1 以 "Unicode字符" 的方式遍历字符串，最好的办法是使用 "chars()" 方法
    for c in "深圳南山".chars() {
        println!("c: {c}")
    }
    // 字节
    // 返回字符串的底层字节数组表现形式
    for c in "深圳南山".bytes() {
        println!("c: {c}")
    }

    // 字符串深度剖析
    // Q: 为啥 String 可变，而字符串字面值 str 却不可以？
    // A: 就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，
    // 这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性。
    // 不幸的是，我们不能为了获得这种性能，而把每一个在编译时大小未知的文本都放进内存中（你也做不到！），
    // 因为有的字符串是在程序运行得过程中动态生成的

    // 对于 String 类型，为了支持一个可变、可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容，这些都是在程序运行时完成的：
    // 1). 首先向操作系统请求内存来存放 String 对象
    // 2). 在使用完成后, 将内存释放, 归还给操作系统
    // 其中第一部分由 `String::from` 完成, 它创建了一个全新的 String. 而第二部分就交由Rust进行处理, 当变量超出其作用域时,
    // rust会自动调用 `Drop` 函数去释放内存, 所以这就引入了 "所有权的问题"
    //
    // 以下有一个例子:
    // 1). String本身是可以变化的, 但是String被绑定到一个不可变的变量ssr上, 此时ssr不允许在操作它时进行修改
    // 2). 将ssr的数据绑定(所有权移动)到可变变量sr上, 此时sr运行操作它时进行修改
    // > 我们可以理解为, 数据是单纯的, 没有这么多花花肠子. 而数据的拥有者(获取所有权的变量), 它门道比较多, 可以做决定是否修改, 什么时候回收内存等...
    // > 所以说: 所有权就是变量控制数据的手段, 可以控制是否可变, 可以控制回收内存的时机(管杀又管埋)
    let ssr = String::from("hello,golang");
    let mut sr = ssr;
    sr.push_str("!");
    dbg!(sr);
}

fn _greet_string(s: &str) {
    println!("string to str: {s}")
}

fn _slice() {
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
    line.clear(); // 可以编译, 因为已经在最后一次s使用之后(超出了其借用作用域)

    // 字符串字面量切片
    // 该切片指向了程序可执行文件中的某个点, 这也是为什么字符串字面量是不可变的, 因为 &str 是一个不可变引用
    // 实际上这部分字面量的生命周期与程序相同, 我们可以理解为在程序运行时, 程序的某个地方就存放着这个字面量值
    // **"str 类型是硬编码进可执行文件，也无法被修改"**
    let ss: &str = "Hello,golang";
    println!("ss: {ss}");
}

fn _struct() {
    // 1). 创建一个结构体
    //
    // 初始化实例时, 每个字段都需要进行初始化
    // 初始化时的字段顺序不需要和结构体定义时的顺序一致
    let mut u1 = User {
        active: true,
        username: String::from("张三"),
        email: String::from("123@wx.com"),
        sign_in_count: 1,
    };
    println!("[Create]: {:?}", u1);

    // 2). 通过 '.' 访问结构体实例的字段
    // 必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变
    // 要不就一起 "可变", 要不就一起 "不变"
    u1.username = String::from("李四");
    println!("[Read and Mut]: {:?}", u1);

    // 3). 根据已有的 struct 构建新的 "结构体实例"
    //
    // 实现了Copy特征的类型无需所有权转移, 可以直接在赋值时进行"数据拷贝",
    // 其中 bool 和 u64 类型就实现了Copy特征,
    // 因此 active 和 sign_in_count 字段在赋值给 user2 时, 仅仅发生了拷贝,而不是所有权转移.
    // 而 username 和 email 具备所有权, 在赋值的时候转移给了u2,
    // 所以u1无法用 u1.username,u1.email 了(但u1的其他实现了copy特征的数据类型还是可以使用)
    //
    // 我们可以理解为: 我的儿子拥有了玩具, 我拥有了我儿子, 那么我肯定拥有了我儿子的玩具
    let _u2 = User {
        active: u1.active,
        // 简短的赋值语法糖
        ..u1 // u1中具有所有权的数据类型被 move 到了u2中, 此时只能使用u1中不具备所有权的数据
    };
    println!("[Read and Mut]: {:?}", u1.active);

    // 4). 元组结构体(结构体有名称, 但是结构体的字段没有名称)
    let color = Color(1, 2, 3);
    println!("[tuple struct]: {:?}", color);
    dbg!(&color); // dbg 默认会拿走所有权再返回出来, 如果我们不希望其拿走所有权, 则我们传递一个借用
    println!("[tuple struct]: {:?}", color);

    // 5). 在函数内部创建一个 struct, 然后返回给调用者。 实际上这个struct还是存储在栈上, rust会自动处理struct的拷贝
    // 在返回的时候所有权交给调用者, 这部分会进行结构体拷贝（因为整个过程中 struct 都是存在栈上, 栈上分配效率高）
    // 如果我们想使其存在堆上, 我们会需要智能指针
}

fn _build_user(usr_name: String, email: String) -> User {
    // 简化 结构体struct 的构建
    User {
        active: true,
        username: usr_name,
        email, // "当函数参数和结构体字段同名时", 可以直接使用缩略的方式进行初始化
        sign_in_count: 1,
    }
}
fn _tuple() {
    // 1. 声明一个简单的元组
    let t = (1, "2", true);
    println!("Decl Tuple: {:?}", t);

    // 2. 模式匹配解构元组
    let (x, y, z) = t;
    println!("Match Tuple x: {x} y: {y} z: {z}");

    // 3. 用 ".' 访问元组
    let xx = t.0;
    let yy = t.1;
    let zz = t.2;
    println!("Dot Match Tuple x: {xx} y: {yy} z: {zz}");

    let s = String::from("hello,rust");
    let (_, length) = _tuple_example(s);
    println!("s length: {length}");
}

fn _tuple_example(s: String) -> (String, usize) {
    let length = s.len();

    (s, length) // 注意不能写成 "(s,s.len())", 因为在 s.len() 之前 s 的所有权已经交给了这个元组, 所以s已经是不能使用了(s不拥有原有的内存地址了)
}

// 类型定义区间 ==================

#[derive(Debug)]
struct User {
    active: bool,
    username: String, // 这代表User拥有了其字段的所有权, 当然我们也可以借用其他数据, 此时需要声明 "生命周期".
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

// 单元结构体
// 还记得之前讲过的基本没啥用的单元类型吧？单元结构体就跟它很像，没有任何字段和属性，但是好在，它还挺有用。
//
// 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：
struct _UnitStruct;

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