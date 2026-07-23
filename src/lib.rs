use anyhow::Result;

pub fn add(left: u64, right: u64) -> Result<u64> {
    Ok(left + right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let result = add(2, 2)?;
        assert_eq!(result, 4);

        Ok(())
    }
}
