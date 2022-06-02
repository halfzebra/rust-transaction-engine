#[cfg(test)]
mod integraion {
    use assert_cmd::prelude::*;
    use std::process::Command;
    use std::str;

    #[test]
    fn basic() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-transaction-engine")?;
        cmd.arg("./examples/basic.csv");
        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

        insta::assert_snapshot!(stdout);

        Ok(())
    }

    #[test]
    fn same_tx() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-transaction-engine")?;
        cmd.arg("./examples/same-tx.csv");
        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

        insta::assert_snapshot!(stdout);

        Ok(())
    }
}
