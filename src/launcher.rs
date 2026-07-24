use lighty_launcher::{Authenticator, UserProfile, auth::{self, AuthProvider, ExposeSecret, SecretString}};


pub async fn online_login() -> anyhow::Result<UserProfile> {
    let mut auth = auth::MicrosoftAuth::new(env!("CLIENT_ID"))
    .with_keyring("FerrumLauncher");


    auth.set_device_code_callback(|code, url| {
        println!("{code} -- {url}");
    });

    let profile = match load_refresh_token() {
        Some(rt) => {
            match auth.authenticate_with_refresh_token(&rt, None).await {
                Ok(profile) => {
                    Some(profile)
                }
                Err(_) => {
                    None
                }
            }
        }
        None => None
    };

    let profile = match profile {
        Some(profile) => profile,
        None => auth.authenticate(None).await?
    };

    if let AuthProvider::Microsoft {refresh_token: Some(rt), .. } = &profile.provider {
        save_refresh_token(rt)?;
    }

    Ok(profile)
}



pub async fn offline_login(username: String) -> anyhow::Result<UserProfile>  {
    
    if !username.is_empty() {
        let mut auth = auth::OfflineAuth::new(username);

        match auth.authenticate(None).await {
            Ok(r) => {
                Ok(r)
            }
            Err(err) => {
                Err(err.into())
            }
        }
    } else {
        Err(anyhow::Error::msg("username is empty"))
    }

    
}

fn load_refresh_token() -> Option<SecretString> {
    let entry = keyring::Entry::new("FerrumLauncher", "microsoft_refresh_token").ok()?;
    let token = entry.get_password().ok()?;
    Some(SecretString::from(token))
}

fn save_refresh_token(token: &SecretString) -> anyhow::Result<()> {
    let entry = keyring::Entry::new("FerrumLauncher", "microsoft_refresh_token")?;
    entry.set_password(token.expose_secret())?;
    Ok(())
}