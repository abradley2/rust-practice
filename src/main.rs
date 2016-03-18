#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
fn main() {
    // basic printing
    println!("Hello, world!");

    // formatted printing
    println!("Hi, my name is: {name}, I am {age} years old",
            name="Tony",
            age=24);

    // variable types
    // (notice snake_case is used for variable names)
    let my_boolean: bool = false;
    let my_decimal: f32 = 3.14;
    let my_integer: i32 = 42;
    let my_char: char = 'T';

    // variables by default are immuatable, if you want them to be mutable you must declare with mut
    let mut my_mutable_integer: i32 = 5;
    // look mom, no errors!
    my_mutable_integer = 6;
    // you still cannot change the base type of mutable data. i32 must remain an i32 for example.
    println!("here's the new integer: {:?}", my_mutable_integer);

    // a tuple is a collection of values with different types
    // often useful for functions that return multiple values
    let my_tuple = (my_boolean, my_decimal, my_integer, my_char);
    // access value of a tuple via their index
    println!("boolean: {:?}, decimal: {:?}, int: {:?}, char: {:?}",
            my_tuple.0,
            my_tuple.1,
            my_tuple.2,
            my_tuple.3);
    // tuples are also easily printable:
    println!("my tuple = {:?}", my_tuple);

    // An array is a collection of similarly-typed objects stored in contiguous memory.
    // Unlike tuples they are created with brackets
    let my_array: [i32; 3] = [56, 12, 7];
    // becayse their size and type is known at compile time, arrays are stack allocated
    println!("my array = {:?}", my_array);

    // slices are like arrays, but size is not known at compile time,
    // they can be used to borrow a secion of an array
    let my_slice: &[i32] = &my_array[0 .. 2];
    println!("my slice = {:?}", my_slice);

    // structs are nice custom types, nothing special
    struct Point {
        x: f64,
        y: f64
    }

    struct Rectangle {
        p1: Point,
        p2: Point
    }

    let my_rectange = Rectangle {
        p1: Point { x: 4.0, y: 5.0},
        p2: Point { x: 7.0, y: 8.0}
    };

    // you can also make a tuple struct
    #[derive(Debug)]
    struct TupleStruct(i32, bool);

    let my_tuple_struct = TupleStruct(92, false);
    println!("my_tuple_struct = {:?}", my_tuple_struct);

    // an enum is a struct-type that can have different variants
    enum Animal {
        Bird,
        Fish { weight: i32 },
        Mammal { is_cow: bool, weight: f64 }
    }

    let my_dog = Animal::Mammal {
        is_cow: false,
        weight: 120.63
    };

    // now you can create a function that accepts Animal::* in general
    fn describe_animal(animal: Animal){
        match animal {
            Animal::Bird => println!("This is a bird"),
            Animal::Fish {weight} => println!("This is a fish"),
            Animal::Mammal {is_cow, weight} => println!("This is a mammal")
        }
    }

    describe_animal(my_dog);

    // strings are tricky... When you want to declare a type of string and you need ownership
    // (like in a constructor) declare the type as a String

    #[derive(Debug)]
    struct Person {
        name: String,
        age: i8
    }

    let me = Person {
        name: "Tony".to_string(),
        age: 24
    };

    println!("I am: {:?}", me);

    // good post:
    // http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html

    // a string is a sequence of unicode scalar values encoded as a stream of UTF-8 bytes.

    // a string slice (&str) has a fixed size and cannot be mutated
    // declaring a string literal gives it a type of &'static str
    let my_string_slice: &'static str = "beep boop";

    fn print_me(input_string: &str) {
        println!("got a string: {:?}", input_string);
    }

    print_me(my_string_slice);

    // when passing a slice of a string with & we are merely "borrowing" it
    // only use String instead of str when ownership must be moved!

    // rust has a loop keyword to indicate an infite loop.
    // break gets out of the loop
    // continue skips to the next iteration

    let mut count: u64 = 0;

    loop {
        println!("count = {}", count);
        if count == 10 {
            break;
        }
        count += 1;
    }

    // you can break out of parent loops from nested loops by labeling the loops
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // descructuring is useful for keeping things concise
    struct Foo {
        x: (u32, u32),
        y: u32
    }
    let bar = Foo {
        x: (1, 2),
        y: 3
    };
    let Foo { x: (a, b), y } = bar;
    println!("a = {}, b = {},  y = {} ", a, b, y);
    // to ignore some variables
    // let Foo { x: (a, b), .. } = bar;
    // the .. is necessary or it will throw an error

    // oop for rust is traits, structs, implementations
    struct Dude {
        name: String
    }

    impl Dude {
        fn say_hello(self: &Self){
            println!("Hi, my name is {:?}", self.name);
        }
    }

    let tony_dude = Dude {
        name: "Tony".to_string()
    };

    tony_dude.say_hello();

    // Closures
    // closers are functions with a specialized syntax which can capture
    // the enclosing environment.
    // they use || instead of () for their parameter lists
    let my_closure = |message: &str| {
        println!("printed from closure: {:?}", message);
    };

    my_closure("hello world!");

    (|| {
        println!("what a lovely closure!");
    })();

    // capturing from closures.
    // when closures get variables from the outer scope, they
    // prioritize the way they are captured in this order
    // 1. by reference: &my_variable
    // 2. by mutable reference: &mut my_variable
    // 3. by value: my_variable
    let color = "blue";
    let print_color = || {
        // this borrows color by reference: &my_color
        println!("the color is: {:?}", color);
    };
    print_color();

    // Generics
    //A type parameter is specified as generic by
    // the use of angle brackets and camel case: <A, B, ...>.
    fn generic_test<GenericOne>(input_one: GenericOne) {
        println!("this function takes different types for args");
    }

    generic_test(235);
    generic_test("look, a string!");

    fn call_function<Func: Fn(i32)>(func: Func, input: i32){
        func(input);
    }

    call_function(|my_input: i32| {
        println!("I have been called: {:?}", my_input);
    }, 74);

    // you can only return closures via boxing
    // this also lets you create partials!!!!

    fn get_addition_partial(first_number: i32) -> Box<Fn(i32) -> i32> {
        let addition_partial = move |second_number: i32| {
            return first_number + second_number;
        };
        return Box::new(addition_partial);
    }

    let add_to_seven = get_addition_partial(7);
    let add_to_five = get_addition_partial(5);

    println!("add four to seven = {:?}", add_to_seven(4));
    println!("add three to five = {:?}", add_to_five(3));

    // notice the "move" keyword above. Putting this in front of the closure
    // means all captures occour by value, not by reference. Because you are
    // returning the closure and thus moving it outside of its scope, it will
    // be outside the scope of any references it has, and needs their values

    
}
