use std::fs;
use text_io;
use tracing::{self, instrument};

#[cfg(feature = "async")]
use tokio;

const DEFAULT_CREDENTIALS_FILE: &str = "credentials";

pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[instrument]
/// Attempts to open the file given by `filename` and parse the credentials. If if None is passed, uses `DEFAULT_CREDENTIALS_FILE`.
/// If the read fails, promts the user for credentials through stdin/stdout.
pub fn get_credentials(file_name: Option<&str>) -> Result<Credentials, String> {
    let file = match file_name {
        Some(file) => file,
        None => {
            tracing::info!("Using default credentials file: {}", DEFAULT_CREDENTIALS_FILE);
            DEFAULT_CREDENTIALS_FILE
        },
    };

    let contents = fs::read_to_string(file);
    match contents {
        Ok(contents) => {
            tracing::debug!("Successfully read credentials from {}", file);
            let mut lines = contents.lines();
            let username = lines.next().unwrap();
            let password = lines.next().unwrap();
            Ok(Credentials {
                username: username.to_string(),
                password: password.to_string(),
            })
        },
        Err(e) => {
            tracing::warn!("Failed to read credentials from {}: {}", file, e);
            tracing::debug!("Promting user for credentials");

            tracing::trace!("Promting user for username");
            println!("username: ");
            let usr = text_io::read!("{}\n");
            tracing::trace!("Promting user for password");
            println!("password: ");
            let pwd = text_io::read!("{}\n");
            tracing::info!("Successfully read credentials from stdin");

            tracing::debug!("Saving credentials to {}", file);
            let res = fs::write(file, format!("{}\n{}", usr, pwd));
            if let Err(e) = res {
                tracing::warn!("Failed to save credentials to {}: {}", file, e);
                Ok(Credentials {
                    username: usr,
                    password: pwd,
                })
            } else {
                tracing::info!("Successfully saved credentials to {}", file);
                Ok(Credentials {
                    username: usr,
                    password: pwd,
                })
            }


        },
    }
}

#[cfg(feature = "async")]
#[instrument]
/// Attempts to open the file given by `filename` and parse the credentials. If if None is passed, uses `DEFAULT_CREDENTIALS_FILE`.
/// If the read fails, promts the user for credentials through stdin/stdout.
pub async fn async_get_credentials(file: Option<&str>) -> Result<Credentials, String> {
    let file = match file {
        Some(file) => file,
        None => {
            tracing::info!("Using default credentials file: {}", DEFAULT_CREDENTIALS_FILE);
            DEFAULT_CREDENTIALS_FILE
        },
    };

    let contents = tokio::fs::read_to_string(file).await;
    match contents {
        Ok(contents) => {
            tracing::debug!("Successfully read credentials from {}", file);
            let mut lines = contents.lines();
            let username = lines.next().unwrap();
            let password = lines.next().unwrap();
            Ok(Credentials {
                username: username.to_string(),
                password: password.to_string(),
            })
        },
        Err(e) => {
            tracing::warn!("Failed to read credentials from {}: {}", file, e);
            tracing::debug!("Promting user for credentials");

            tracing::trace!("Promting user for username");
            println!("username: ");
            let usr = text_io::read!("{}\n");
            tracing::trace!("Promting user for password");
            println!("password: ");
            let pwd = text_io::read!("{}\n");
            tracing::info!("Successfully read credentials from stdin");

            tracing::debug!("Saving credentials to {}", file);
            let res = tokio::fs::write(file, format!("{}\n{}", usr, pwd)).await;
            if let Err(e) = res {
                tracing::warn!("Failed to save credentials to {}: {}", file, e);
                Ok(Credentials {
                    username: usr,
                    password: pwd,
                })
            } else {
                tracing::info!("Successfully saved credentials to {}", file);
                Ok(Credentials {
                    username: usr,
                    password: pwd,
                })
            }
        }
    }
} 