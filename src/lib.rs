use smb::{Client, ClientConfig, UncPath};

use std::str::FromStr;

pub fn connect(
    unc: String,
    user: String,
    password: String,
) -> Result<(UncPath, Client), Box<dyn std::error::Error>> {
    // instantiate the client
    let client = Client::new(ClientConfig::default());

    // Connect to a share
    let target_path = match UncPath::from_str(&unc) {
        Ok(target_path) => target_path,
        Err(e) => return Err(Box::new(e)),
    };
    client.share_connect(&target_path, &user, password)?;

    Ok((target_path, client))
}

#[test_log::test]
fn connection() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let unc = "\\\\172.21.202.7\\MyShare\\bla\\".to_string();
    let user = "LocalAdmin".to_string();
    let password = "123456".to_string();

    for i in 1..101 {
        println!("Run {}", i);
        match connect(unc.clone(), user.clone(), password.clone()) {
            Ok((_unc, client)) => {
                client.close()?;
            }
            Err(e) => {
                return Err(e);
            }
        }
        println!("finished");
    }

    Ok(())
}
