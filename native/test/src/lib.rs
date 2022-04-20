#[derive(Debug)]
pub enum MyError {
    MyError,
}

type Result<T> = std::result::Result<T, MyError>;

#[derive(rustler::NifMap)]
struct TestStruct {
    a: i32,
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!("Elixir.Test", [add]);
