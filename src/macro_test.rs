macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    )
}

macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

pub fn run() {
    create_function!(foo);
    foo();
}