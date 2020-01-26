// extern crate we're testing, same as any other code would do.
extern crate rs2ts_proc;

use rs2ts_proc::ParseToTS;

#[derive(ParseToTS)]
struct TestStruct {
  x: f32,
  y: f32,
  #[ts_type]
  name: String,
  active: bool
}

#[test]
fn test_add() {
    assert_eq!(2 + 2, 4);
}
