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

use super::*;
use anyhow::{bail, Result};

pub fn get_task(task: &str) -> Result<SubmissionFormat> {
    let req = ApiQuery {
        action: "get",
        name: Some(task.to_string()),
        ..EMPTY_QUERY
    };

    let client = reqwest::blocking::Client::new();
    let resp = client.post(TASK_API_URL).json(&req).send()?;

    let json: ResultSubmissionFormat = resp.json()?;

    match json {
        ResultSubmissionFormat::Success(x) => Ok(x),
        ResultSubmissionFormat::Insuccess { error } => bail!("Failed to fetch task! {error}"),
    }
}
