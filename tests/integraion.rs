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

    #[test]
    fn locked() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-transaction-engine")?;
        cmd.arg("./examples/locked.csv");
        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

        insta::assert_snapshot!(stdout);

        Ok(())
    }

    #[test]
    fn dispute_withdrawal_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-transaction-engine")?;
        cmd.arg("./examples/dispute-withdrawal-empty.csv");
        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

        insta::assert_snapshot!(stdout);

        Ok(())
    }

    #[test]
    fn long_number_format() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-transaction-engine")?;
        cmd.arg("./examples/long-number-format.csv");
        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

        insta::assert_snapshot!(stdout);

        Ok(())
    }
}
