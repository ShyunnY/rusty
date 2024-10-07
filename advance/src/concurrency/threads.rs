use core::time;
use std::thread;

#[allow(dead_code)]
pub fn hello() {
    // (1) 创建线程
    // 使用 `thread::spawn`` 可以创建一个子线程 (spawn意味着产生!)
    // 线程调度的方式往往取决于你使用的操作系统,
    // 我们 "千万不要依赖线程的执行顺序" ! 线程的调度是不可预估的!
    {
        thread::spawn(|| {
            for ele in 1..=3 {
                println!("sub thread: {}", ele);
            }
        });

        // 由于main线程结束会导致子线程可能无法调度, 所以使main线程休眠让core调度子线程
        println!("main thread sleep...");
        thread::sleep(time::Duration::from_millis(100));
    }
    /*
       有几点需要注意!
       1.线程内部的代码"使用闭包函数来执行"
       2.main 线程一旦结束, 程序就"立刻结束". 因此需要保持它的存活, 直到其它子线程完成自己的任务
       3.`thread::sleep`` 会让当前线程休眠指定的时间, 随后其它线程会被调度运行,
          因此就算你的电脑只有一个 CPU 核心, 该程序也会表现的如同多 CPU 核心一般, 这就是并发！
    */
}
