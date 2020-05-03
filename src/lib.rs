use anyhow::{anyhow, Error, Result};

pub trait BeauCollector<I, T>
where
    I: std::iter::FromIterator<T>,
{
    fn bcollect(self) -> Result<I>;
}

impl<I, T, U> BeauCollector<I, T> for U
where
    U: Iterator<Item = Result<T>>,
    I: std::iter::FromIterator<T>,
    T: std::fmt::Debug,
{
    fn bcollect(self) -> Result<I> {
        let (good, bad): (I, Vec<Error>) = (|(g, b): (Vec<_>, Vec<_>)| {
            (
                g.into_iter().map(Result::unwrap).collect(),
                b.into_iter().map(Result::unwrap_err).collect(),
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
    fn into_vec() -> Result<()> {
        let x: Vec<Result<()>> = vec![Ok(()), Err(anyhow!("woops")), Err(anyhow!("woopsie"))];

        let y: Result<Vec<()>> = x.into_iter().bcollect();

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
        let x = vec![Ok((1, 10)), Ok((2, 20)), Ok((3, 30))];

        let y: Result<std::collections::HashMap<usize, usize>> = x.into_iter().bcollect();

        assert!(y.is_ok());

        Ok(())
    }

    #[test]
    fn into_yaml_mapping_ok() -> Result<()> {
        use serde_yaml::{Mapping, Value};
        let x = vec![
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
