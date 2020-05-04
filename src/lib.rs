//!  Collect _all_ the errors from an iterator of `Result`s
//!  into a `Result` with a single `Error`: `Result<T, Error>`.
//!
//!  The resultant Error has an error message with all the
//!  errors' messages each on a newline.
//!
//!  Examples:
//!
//!  ```
//!  use anyhow::{anyhow, Result};
//!  use beau_collector::BeauCollector as _;
//!
//!  let x = vec![Ok(()), Err(anyhow!("woops")), Err(anyhow!("woops again"))];
//!
//!  let y: Result<Vec<()>> = x.into_iter().bcollect();
//!
//!  assert_eq!(y.unwrap_err().to_string(), "woops\nwoops again")
//!  ```
//!
//!  ,
//!
//!  ```
//!  use anyhow::{anyhow, Result};
//!  use std::collections::HashMap;
//!  use beau_collector::BeauCollector as _;
//!
//!  let x = vec!["one", "two", "three", "four"];
//!
//!  let y = x
//!      .iter()
//!      .map(|name: &&str| -> Result<(String, usize)> {
//!          let length = name.len();
//!          if length < 4 {
//!             Ok((name.to_string(), length))
//!          } else {
//!             Err(anyhow!("name \"{}\" has {} characters", name, length))
//!          }
//!      })
//!      // Turbofish ::<> gives the type to be collected into the Ok()
//!      // variant of the Result.
//!      .bcollect::<HashMap<_, _>>();
//!
//!  assert_eq!(
//!      y.unwrap_err().to_string(),
//!      "name \"three\" has 5 characters\nname \"four\" has 4 characters")
//!  ```
use anyhow::{anyhow, Error, Result};

pub trait BeauCollector<T> {
    fn bcollect<I>(self) -> Result<I>
    where
        I: std::iter::FromIterator<T>;
}

impl<T, U, E> BeauCollector<T> for U
where
    U: Iterator<Item = Result<T, E>>,
    E: std::convert::Into<Error>,
{
    #[allow(clippy::redundant_closure_call)]
    fn bcollect<I>(self) -> Result<I>
    where
        I: std::iter::FromIterator<T>,
    {
        let (good, bad): (I, Vec<Error>) = (|(g, b): (Vec<_>, Vec<_>)| {
            (
                g.into_iter()
                    .map(|res| match res {
                        Ok(x) => x,
                        Err(_) => panic!(),
                    })
                    .collect(),
                b.into_iter()
                    .map(|res| match res {
                        Ok(_) => panic!(),
                        Err(e) => e,
                    })
                    .map(Into::into)
                    .collect(),
            )
        })(self.partition(Result::is_ok));

        if bad.is_empty() {
            Ok(good)
        } else {
            use itertools::Itertools as _;
            Err(anyhow!(
                "{}",
                bad.iter().map(|e| e.to_string()).format("\n")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_vec_fully_qualified() -> Result<()> {
        let x = vec![Ok(()), Err(anyhow!("woops")), Err(anyhow!("woopsie"))];

        let y = BeauCollector::<()>::bcollect::<Vec<()>>(x.into_iter());

        assert!(y.is_err());

        Ok(())
    }

    #[test]
    fn into_vec() -> Result<()> {
        let x = vec![Ok(()), Err(anyhow!("woops")), Err(anyhow!("woopsie"))];

        let y: Result<_> = x.into_iter().bcollect::<Vec<()>>();

        assert!(y.is_err());

        Ok(())
    }

    #[test]
    fn into_hashmap() -> Result<()> {
        let x = vec![
            Ok((1, 10)),
            Err(anyhow!("hi")),
            Ok((2, 20)),
            Err(anyhow!("there")),
            Ok((3, 30)),
        ];

        let y: Result<std::collections::HashMap<usize, usize>> = x.into_iter().bcollect();

        assert!(y.is_err());

        Ok(())
    }

    #[test]
    fn into_yaml_mapping() -> Result<()> {
        use serde_yaml::{Mapping, Value};
        let x = vec![
            Ok((
                Value::String("one".to_string()),
                Value::String("ten".to_string()),
            )),
            Err(anyhow!("hey")),
            Ok((
                Value::String("two".to_string()),
                Value::String("twenty".to_string()),
            )),
            Err(anyhow!("soul")),
            Ok((
                Value::String("three".to_string()),
                Value::String("thirty".to_string()),
            )),
            Err(anyhow!("sister")),
        ];

        let y: Result<Mapping> = x.into_iter().bcollect();

        assert!(y.is_err());

        Ok(())
    }

    #[test]
    fn into_vec_ok() -> Result<()> {
        let x: Vec<Result<()>> = vec![Ok(()), Ok(())];

        let y: Result<Vec<()>> = x.into_iter().bcollect();

        assert!(y.is_ok());

        Ok(())
    }

    #[test]
    fn into_hashmap_ok() -> Result<()> {
        let x: Vec<Result<_>> = vec![Ok((1, 10)), Ok((2, 20)), Ok((3, 30))];

        let y = x
            .into_iter()
            .bcollect::<std::collections::HashMap<usize, usize>>();

        assert!(y.is_ok());

        Ok(())
    }

    #[test]
    fn into_yaml_mapping_ok() -> Result<()> {
        use serde_yaml::{Mapping, Value};
        let x: Vec<Result<_>> = vec![
            Ok((
                Value::String("one".to_string()),
                Value::String("ten".to_string()),
            )),
            Ok((
                Value::String("two".to_string()),
                Value::String("twenty".to_string()),
            )),
            Ok((
                Value::String("three".to_string()),
                Value::String("thirty".to_string()),
            )),
        ];

        let y: Result<Mapping> = x.into_iter().bcollect();

        assert!(y.is_ok());

        Ok(())
    }
}
