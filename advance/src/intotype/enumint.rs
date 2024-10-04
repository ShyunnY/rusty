use num_enum::TryFromPrimitive;

#[allow(dead_code)]
pub fn hello() {
    // 枚举和整数
    /*
       一个真实场景的需求:
       在实际场景中, 从整数到枚举的转换有时还是非常需要的, 例如你有一个枚举类型, 然后需要从外面传入一个整数
       用于控制后续的流程走向, 此时就需要用整数去匹配相应的枚举(你也可以用整数匹配整数-_-, 看看会不会被喷)

       总结: 我们希望将整数匹配到对应的枚举上
    */
    {
        // 在rust中, 枚举可以转换为基础类型, 但是基础类型不能转换成枚举
        // (-_-!)

        enum Point {
            First = 1,
            Second,
        }

        // 可以转换, 正常编译
        let p = Point::Second;
        let second = p as u8;
        println!("second: {}", second);

        // 无法转换, 无法编译
        // match 1 {
        //     Point::First => todo!(),
        //     Point::Second => todo!(),
        // }
    }

    // (1). 第三方包, 我就依赖外部 :)
    // 此处使用了 num_enum 外部crate
    {
        // Zero = 1 语法相当于 Golang 中 Zero iota = 1
        #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
        #[repr(u8)]
        enum Counter {
            Zero = 0,
            One,
            Two,
            Three,
        }

        let x = 3;
        match Counter::try_from_primitive(x) {
            Ok(v) => match v {
                Counter::Zero => println!("0"),
                Counter::One => println!("1"),
                Counter::Two => println!("2"),
                Counter::Three => println!("3"),
            },
            Err(e) => println!("convert err: {}", e),
        }
    }

    // (2). 使用宏来编排, 这个太难了。。。目前不会呢
}
