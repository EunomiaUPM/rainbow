// Tests corresponding to 'rainbow-auth\src\ssi_auth\cmd\mod.rs'

use tokio;
use rainbow_auth::ssi_auth::cmd::AuthCliCommands;
use rainbow_auth::ssi_auth::cmd::AuthCliRoles;
use rainbow_auth::ssi_auth::cmd::AuthCli;

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;
    
    #[tokio::test]
    async fn test_provider_start_command() {
        use std::env;
        env::set_var("RUST_BACKTRACE", "1");

        let args = vec![
            "binary_name",
         "provider",
         "start",
         "--env-file",
         "provider.env"
         ];

        let cli = AuthCli::try_parse_from(args).expect("Failed to parse CLI arguments");

        match cli.role {
            AuthCliRoles::Provider(AuthCliCommands::Start(args)) => {
                assert_eq!(args.env_file, Some("provider.env".to_string()));
            }  ,
            _ => panic!("Unexpected CLI structure"),
        }
    }


    #[tokio::test]
    async fn test_provider_setup_command() {
        let args = vec![
            "binary_name",
            "provider",
            "setup",
            "--env-file",
            "provider.env"
        ];

        let cli = AuthCli::try_parse_from(args).expect("Failed to parse CLI arguments");

        match cli.role {
            AuthCliRoles::Provider(AuthCliCommands::Setup(args)) => {
                assert_eq!(args.env_file, Some("provider.env".to_string()));
            },
            _ => panic!("Unexpected CLI structure"),
        }
    }

    #[tokio::test]
    async fn test_consumer_start_command() {
        let args = vec![
            "binary_name",
            "consumer",
            "start",
            "--env-file",
            "consumer.env"
        ];

        let cli = AuthCli::try_parse_from(args).expect("Failed to parse CLI arguments");

        match cli.role {
            AuthCliRoles::Consumer(AuthCliCommands::Start(args)) => {
                assert_eq!(args.env_file, Some("consumer.env".to_string()));
            },
            _ => panic!("Unexpected CLI structure"),
        }
    }

    #[tokio::test]
    async fn test_consumer_setup_command() {
        let args = vec![
            "binary_name",
            "consumer",
            "setup",
            "--env-file",
            "consumer.env"
        ];

        let cli = AuthCli::try_parse_from(args).expect("Failed to parse CLI arguments");

        match cli.role {
            AuthCliRoles::Consumer(AuthCliCommands::Setup(args)) => {
                assert_eq!(args.env_file, Some("consumer.env".to_string()));
            },
            _ => panic!("Unexpected CLI structure"),
        }
    }
}
