* fn do1(c: String) {}：表示实参会将所有权传递给 c
* fn do2(c: &String) {}：表示实参的不可变引用（指针）传递给 c，实参需带 & 声明
* fn do3(c: &mut String) {}：表示实参可变引用（指针）传递给 c，实参需带 let mut 声明，且传入需带 &mut
* fn do4(mut c: String) {}：表示实参会将所有权传递给 c，且在函数体内 c 是可读可写的，实参无需 mut 声明
* fn do5(mut c: &mut String) {}：表示实参可变引用指向的值传递给 c，且 c 在函数体内部是可读可写的，实参带 let mut 声明，且传入需带 &mut

一句话总结：在函数参数中，冒号左边的部分，如：mut c，这个 mut 是对🪄函数体内部有效🪄；冒号右边的部分，如：&mut String，这个 &mut 是针对🪄**外部实参传入时的形式（声明）说明**🪄。

## 闭包中的模式匹配

当参数复杂时, 我们常常会进行闭包中的模式匹配

* 当前有 param = &i32
* 现在闭包参数声明为 x, 那么 `x type=&i32`。 因为 `x 匹配 &i32, x=&i32`
* 现在闭包参数声明为 &x, 那么 `x type=i32`。 因为 `&x 匹配 &i32, x=i32`

```rust
{
        let arr = vec![1, 2, 3];
        let result: Vec<i32> = arr
            .into_iter()
            .filter(|x| {
                // 一般来说, 在闭包中都会用到一个叫模式匹配的东西, 尤其是匹配闭包参数
                // 例如
                // param = &i32
                // &x = param
                // 所以x=i32(实现Copy)

                x % 2 == 0
            })
            .collect();
        println!("result: {:?}", result);
    }
```