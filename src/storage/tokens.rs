// Copyright (C) 2025  laxenta
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Default)]
struct TokenStore {
    accounts: HashMap<String, crate::spotify::auth::Tokens>,
}

fn get_tokens_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home.join(".spotify-sync");
    
    //  dir if it doesnt exist
    fs::create_dir_all(&config_dir)?;
    
    Ok(config_dir.join("tokens.json"))
}

pub fn save_tokens(account_name: &str, tokens: &crate::spotify::auth::Tokens) -> Result<()> {
    let path = get_tokens_path()?;
    
    // existing tokens?
    let mut store: TokenStore = if path.exists() {
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)?
    } else {
        TokenStore::default()
    };

    // Add/update account
    store.accounts.insert(account_name.to_string(), tokens.clone());

    // Save back
    let json = serde_json::to_string_pretty(&store)?;
    fs::write(&path, json)?;

    Ok(())
}

pub fn load_tokens(account_name: &str) -> Result<crate::spotify::auth::Tokens> {
    let path = get_tokens_path()?;
    let content = fs::read_to_string(&path)?;
    let store: TokenStore = serde_json::from_str(&content)?;

    store.accounts
        .get(account_name)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Account '{}' not found", account_name))
}

pub fn list_accounts() -> Result<Vec<String>> {
    let path = get_tokens_path()?;
    
    if !path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&path)?;
    let store: TokenStore = serde_json::from_str(&content)?;

    Ok(store.accounts.keys().cloned().collect())
}