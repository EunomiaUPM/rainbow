// Tests corresponding to 'rainbow-auth\src\ssi_auth\cmd\mod.rs'

#[cfg(test)]
mod tests {
    use std::fs;

    use assert_cmd::cargo::cargo_bin_cmd;

    #[test]
    fn test_init_command_line_provider_start_success() {
        let env_path = "test_provider_start.env";
        fs::write(env_path, "DUMMY_KEY=DUMMY_VALUE").unwrap();

        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("provider").arg("start").arg("--env-file").arg(env_path).assert().success();

        fs::remove_file(env_path).unwrap();
    }

    #[test]
    fn test_init_command_line_provider_setup_success() {
        let env_path = "test_provider_setup.env";
        fs::write(env_path, "DUMMY_KEY=DUMMY_VALUE").unwrap();

        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("provider").arg("setup").arg("--env-file").arg(env_path).assert().success();

        fs::remove_file(env_path).unwrap();
    }

    #[test]
    fn test_init_command_line_consumer_start_success() {
        let env_path = "test_consumer_start.env";
        fs::write(env_path, "DUMMY_KEY=DUMMY_VALUE").unwrap();

        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("consumer").arg("start").arg("--env-file").arg(&env_path).assert().success();

        fs::remove_file(env_path).unwrap();
    }

    #[test]
    fn test_init_command_line_consumer_setup_success() {
        let env_path = "test_consumer_setup.env";
        fs::write(env_path, "DUMMY_KEY=DUMMY_VALUE").unwrap();

        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("consumer").arg("setup").arg("--env-file").arg(env_path).assert().success();

        fs::remove_file(env_path).unwrap();
    }

    #[test]
    fn test_invalid_subcommand_should_fail() {
        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("provider").arg("Foo").assert().failure();
    }

    #[test]
    fn test_incomplete_command_should_fail() {
        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("consumer").assert().failure();
    }

    #[test]
    fn test_invalid_role_should_fail() {
        let mut cmd = cargo_bin_cmd!("rainbow_auth");
        cmd.env("TEST_MODE", "true").arg("InvalidRole").assert().failure();
    }
}
