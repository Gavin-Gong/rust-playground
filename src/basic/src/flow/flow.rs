pub fn conditional_feat() {
    //
    {
        let age = 5;
        if age > 5 {
            println!("You are old");
        } else if age < 100 {
            println!("You are young");
        } else {
            println!("You are died");
        }
    }

    {
        let age = 5;
        let is_old = if age > 5 { true } else { false };
        println!("{}", is_old)
    }
}

pub fn loop_feat() {
    // while

    {
        let mut i = 10;
        while i > 0 {
            i = i - 1;
            println!("{}", i);
        }
    }

    // for...in
    for n in 1..101 {
        println!("{}", n);
        if n > 80 {
            break;
        }
    }

    // loop
    let mut foo = 0;
    loop {
        foo = foo + 1;
        if foo > 100 {
            break;
        }
    }
}
