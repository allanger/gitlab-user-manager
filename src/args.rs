use std::io::Result;
pub(crate) trait Arg<'a> {
    fn parse<T>(&self) -> Result<T>;
}

struct GitlabTokenArg;

#[derive(Debug)]
struct D6(u64);

impl Arg<D6> for GitlabTokenArg {
  fn roll() -> D6 {
      D6 {
          0: thread_rng().gen_range(1..=6),
      }
  }

fn parse<T>(&self) -> Result<T> {
        todo!()
    }
}