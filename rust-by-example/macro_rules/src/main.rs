macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.]
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),*]
        );*

    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

#[allow(unused_must_use)]
macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

fn main() {
    foo();
    bar();
    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    calculate! {
        eval 1+2 
    }

    calculate! {
        eval (1+2) * (3/1)
    }

    let a: &[i32]
        = o_O!(10; [1,2,3]; 20; [4,5,6]);
    println!("o_O: {:?}", a);

    use std::fmt::Write;
    let mut out = String::new();
    write_html!(&mut out,
    html[
    head[title["Macros guide"]]
    body[h1["Macros are the best!"]]
    ]);
    assert_eq!(out,
    "<html><head><title>Macros guide</title></head>\
    <body><h1>Macros are the best!</h1></body></html>");
    println!("out: {}",out);
}
