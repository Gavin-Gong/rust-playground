pub fn ownership() {
    // 堆数据 move 语义
    {
        let foo = String::from("hello");
        let _bar = foo.clone();
        println!("{}", foo);
    }

    // 栈数据 copy 语言
    {
        let foo = 1;
        let _bar = foo;
        println!("{}", foo)
    }
}
