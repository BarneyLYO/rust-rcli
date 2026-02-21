#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)] // marco
fn main1() {
    // immutable binding
    let x: i32 = 1;

    // you can use value with type suffix to do a binding
    let y = 13i32;
    let f = 1.3f64;
    println!("{} {}", f, x);

    let implicit_x = 1;
    let implicit_f = 1.3;

    let sum = x + y + 13;

    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // string literals is pointer to string
    let x: &str = "123";

    /*
    A String - a heap allocated string
    Stored as a Vec<u8> and always hold a valid UTF8 sequence
    which is not null terminated
     */
    let s: String = "hello world".to_string();

    /*
    and then we bind the reference of String to s_slice
    an immutable pair of pointers to a string
    basically the &Vec<u8>, point to the begainning of String
     */
    let s_slice: &str = &s;

    println!("{} {}", s, s_slice);

    // fix size arr, stack located
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // a dynamic array vector
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    let vector1: Vec<i32> = vec![1, 2, 3, 4, 5];
    // vector1.push(111); can not mutate immutable

    // a slice - immutable view into a vector or array
    // this is much like a string but for vector
    let slice: &[i32] = &vector;

    println!("{:?} {:?}", vector, slice); // [1,2,3,4,5] [1,2,3,4,5]

    // tuple
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // destructing tuple
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);

    // print the 1 of tuple, hello
    println!("{}", x.1);

    /*
    types
     */

    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };

    // tuple struct
    struct Point2(i32, i32);

    let origin2 = Point2(2, 3);

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two = OptionalI32::AnI32(2);
    let noting = OptionalI32::Nothing;

    struct Foo<T> {
        bar: T,
    }

    enum Optional1<T> {
        SomeValue(T),
        NoVal,
    }

    // methods
    impl<T> Foo<T> {
        fn bar(&self) -> &T {
            // self is borrowed
            &self.bar
        }
        fn bar_mut(&mut self) -> &mut T {
            // self is mutably borrowed
            &mut self.bar
        }
        fn into_bar(self) -> T {
            // here self is consumed
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    print!("{}", a_foo.bar());

    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Option::Some(self.bar)
        }
    }

    let another_foo = Foo { bar: 1 };

    println!("{:?}", another_foo.frobnicate());

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    type FunctionPointer = fn(u32) -> u32;
    let fib: FunctionPointer = fibonacci;
    println!("Fib: {}", fib(4));

    // pattern match
    let foo = OptionalI32::AnI32(1);
    match foo {
        OptionalI32::AnI32(n) => println!("is i32 {}", n),
        OptionalI32::Nothing => println!("its nothing"),
    }

    struct Foobar {
        x: i32,
        y: OptionalI32,
    }
    let bar = Foobar {
        x: 15,
        y: OptionalI32::AnI32(32),
    };

    match bar {
        Foobar {
            x: 0,
            y: OptionalI32::AnI32(0),
        } => println!("number are zero"),
        Foobar {
            x: n,
            y: OptionalI32::AnI32(m),
        } if n == m => println!("number the same"),
        Foobar {
            x: n,
            y: OptionalI32::AnI32(m),
        } => println!("different number"),
        Foobar {
            x: _,
            y: OptionalI32::Nothing,
        } => println!("second is nothing"),
    }

    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    // range
    for i in 0u32..10 {
        println!("{}", i)
    }

    if 1 == 1 {
        println!("math is working")
    } else {
        println!("eeeeee")
    }

    let value = if true { "good" } else { "bad" };

    while 1 == 1 {
        println!("1==1");
        break;
    }

    loop {
        println!("hello");
        break;
    }

    println!("Hello, rust!");
}

#[allow(dead_code)] // marco
fn add2(x: i32, y: i32) -> i32 {
    // this is function
    x + y // implicit return (no semicolon)
}
