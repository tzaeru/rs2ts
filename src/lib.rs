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
