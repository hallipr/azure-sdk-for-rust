// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::headers::{self, Header};
#[cfg(doc)]
use crate::http::request::Request;

/// Set the "content-length" header in a [`Request`].
#[derive(Debug, Clone, Copy)]
pub struct ContentLength(i32);

impl ContentLength {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl Header for ContentLength {
    fn name(&self) -> headers::HeaderName {
        headers::CONTENT_LENGTH
    }

    fn value(&self) -> headers::HeaderValue {
        let count = if self.0 < 0 { -1 } else { self.0 };
        format!("{count}").into()
    }
}
