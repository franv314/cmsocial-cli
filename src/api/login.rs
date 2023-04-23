/*  Copyright 2023 Francesco Vercellesi
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use super::USER_API_URL;
use crate::error;
use serde_json::json;

pub fn login(username: &str, password: &str) -> error::Result<String> {
    let req = json!({
        "action": "login",
        "keep_signed": "true",
        "username": username,
        "password": password,
    });

    let client = reqwest::blocking::Client::new();
    let resp = client.post(USER_API_URL).json(&req).send()?;

    let token = resp
        .headers()
        .get("set-cookie")
        .ok_or(error::Error::Api(String::from("Failed to login!")))?;

    Ok(String::from(token.to_str().unwrap()))
}
