use std::thread;
use std::time::Duration;

fn main() {
    // 1. 需要调用 thread::spawn 函数并传递一个闭包
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 2. 用join 等待所有线程结束

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap(); // 主线程等待上面的全部完成 才会继续执行

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 3. move 我们在一个线程中使用另一个线程的数据 move 关键字强制闭包获取其使用的环境值的所有权

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);  // move了 所以不能在主线程drop v

    handle2.join().unwrap();
}

