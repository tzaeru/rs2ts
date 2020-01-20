// extern crate we're testing, same as any other code would do.
extern crate rs2ts;

use rs2ts::ParseToTS;

#[derive(ParseToTS)]
struct TestStruct {
  x: f32,
  #[ts_type]
  y: String
}

#[test]
fn test_add() {
    assert_eq!(2 + 2, 4);
}

/*
use rs2ts_derive::UniformInterface;

#[derive(UniformInterface)]
struct SimpleUniformInterface {
  x: f32,
  #[ts_type]
  y: String
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

*/