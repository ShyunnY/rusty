// 这意味着当前处于animals模块下, 需要把想加载的模块通过 mod 加载进来
// ** 如果新建了一个文件(模块), 一定要将其在此通过 "mod <moduleName>" 加载到 people 下, 否则无法进行使用(等于白写)
pub mod cat;
pub mod dog;
pub mod method;

mod fish;
pub mod people;
