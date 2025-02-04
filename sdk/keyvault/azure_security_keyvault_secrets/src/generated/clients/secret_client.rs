// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::models::{
    BackupSecretResult, DeletedSecretBundle, DeletedSecretListResult, SecretBundle,
    SecretListResult, SecretRestoreParameters, SecretSetParameters, SecretUpdateParameters,
};
use azure_core::credentials::TokenCredential;
use azure_core::{
    BearerTokenCredentialPolicy, ClientMethodOptions, ClientOptions, Context, Method, Pager,
    Pipeline, Policy, Request, RequestContent, Response, Result, Url,
};
use std::sync::Arc;
use typespec_client_core::fmt::SafeDebug;
use typespec_client_core::http::PagerResult;
use typespec_client_core::json;

pub struct SecretClient {
    api_version: String,
    endpoint: Url,
    pipeline: Pipeline,
}

#[derive(Clone, SafeDebug)]
pub struct SecretClientOptions {
    pub api_version: String,
    pub client_options: ClientOptions,
}

impl SecretClient {
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<SecretClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint)?;
        endpoint.set_query(None);
        let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
            credential,
            vec!["https://vault.azure.net/.default"],
        ));
        Ok(Self {
            endpoint,
            api_version: options.api_version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::default(),
                vec![auth_policy],
            ),
        })
    }

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Backs up the specified secret.
    ///
    /// Requests that a backup of the specified secret be downloaded to the client. All versions of the secret will be downloaded.
    /// This operation requires the secrets/backup permission.
    pub async fn backup_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientBackupSecretOptions<'_>>,
    ) -> Result<Response<BackupSecretResult>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}/backup");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Deletes a secret from a specified key vault.
    ///
    /// The DELETE operation applies to any secret stored in Azure Key Vault. DELETE cannot be applied to an individual version
    /// of a secret. This operation requires the secrets/delete permission.
    pub async fn delete_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientDeleteSecretOptions<'_>>,
    ) -> Result<Response<DeletedSecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Delete);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Gets the specified deleted secret.
    ///
    /// The Get Deleted Secret operation returns the specified deleted secret along with its attributes. This operation requires
    /// the secrets/get permission.
    pub async fn get_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientGetDeletedSecretOptions<'_>>,
    ) -> Result<Response<DeletedSecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("deletedsecrets/{secret-name}");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Lists deleted secrets for the specified vault.
    ///
    /// The Get Deleted Secrets operation returns the secrets that have been deleted for a vault enabled for soft-delete. This
    /// operation requires the secrets/list permission.
    pub fn get_deleted_secrets(
        &self,
        options: Option<SecretClientGetDeletedSecretsOptions<'_>>,
    ) -> Result<Pager<DeletedSecretListResult>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        first_url = first_url.join("deletedsecrets")?;
        first_url
            .query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        if let Some(maxresults) = options.maxresults {
            first_url
                .query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        let api_version = self.api_version.clone();
        Ok(Pager::from_callback(move |next_link: Option<Url>| {
            let url = match next_link {
                Some(next_link) => {
                    let qp = next_link
                        .query_pairs()
                        .filter(|(name, _)| name.ne("api-version"));
                    let mut next_link = next_link.clone();
                    next_link
                        .query_pairs_mut()
                        .clear()
                        .extend_pairs(qp)
                        .append_pair("api-version", &api_version);
                    next_link
                }
                None => first_url.clone(),
            };
            let mut request = Request::new(url, Method::Get);
            request.insert_header("accept", "application/json");
            let ctx = options.method_options.context.clone();
            let pipeline = pipeline.clone();
            async move {
                let rsp: Response<DeletedSecretListResult> =
                    pipeline.send(&ctx, &mut request).await?;
                let (status, headers, body) = rsp.deconstruct();
                let bytes = body.collect().await?;
                let res: DeletedSecretListResult = json::from_json(bytes.clone())?;
                let rsp = Response::from_bytes(status, headers, bytes);
                Ok(match res.next_link {
                    Some(next_link) => PagerResult::Continue {
                        response: rsp,
                        continuation: next_link.parse()?,
                    },
                    None => PagerResult::Complete { response: rsp },
                })
            }
        }))
    }

    /// Get a specified secret from a given key vault.
    ///
    /// The GET operation is applicable to any secret stored in Azure Key Vault. This operation requires the secrets/get permission.
    pub async fn get_secret(
        &self,
        secret_name: &str,
        secret_version: &str,
        options: Option<SecretClientGetSecretOptions<'_>>,
    ) -> Result<Response<SecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}/{secret-version}");
        path = path.replace("{secret-name}", secret_name);
        path = path.replace("{secret-version}", secret_version);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// List all versions of the specified secret.
    ///
    /// The full secret identifier and attributes are provided in the response. No values are returned for the secrets. This operations
    /// requires the secrets/list permission.
    pub fn get_secret_versions(
        &self,
        secret_name: &str,
        options: Option<SecretClientGetSecretVersionsOptions<'_>>,
    ) -> Result<Pager<SecretListResult>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}/versions");
        path = path.replace("{secret-name}", secret_name);
        first_url = first_url.join(&path)?;
        first_url
            .query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        if let Some(maxresults) = options.maxresults {
            first_url
                .query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        let api_version = self.api_version.clone();
        Ok(Pager::from_callback(move |next_link: Option<Url>| {
            let url = match next_link {
                Some(next_link) => {
                    let qp = next_link
                        .query_pairs()
                        .filter(|(name, _)| name.ne("api-version"));
                    let mut next_link = next_link.clone();
                    next_link
                        .query_pairs_mut()
                        .clear()
                        .extend_pairs(qp)
                        .append_pair("api-version", &api_version);
                    next_link
                }
                None => first_url.clone(),
            };
            let mut request = Request::new(url, Method::Get);
            request.insert_header("accept", "application/json");
            let ctx = options.method_options.context.clone();
            let pipeline = pipeline.clone();
            async move {
                let rsp: Response<SecretListResult> = pipeline.send(&ctx, &mut request).await?;
                let (status, headers, body) = rsp.deconstruct();
                let bytes = body.collect().await?;
                let res: SecretListResult = json::from_json(bytes.clone())?;
                let rsp = Response::from_bytes(status, headers, bytes);
                Ok(match res.next_link {
                    Some(next_link) => PagerResult::Continue {
                        response: rsp,
                        continuation: next_link.parse()?,
                    },
                    None => PagerResult::Complete { response: rsp },
                })
            }
        }))
    }

    /// List secrets in a specified key vault.
    ///
    /// The Get Secrets operation is applicable to the entire vault. However, only the base secret identifier and its attributes
    /// are provided in the response. Individual secret versions are not listed in the response. This operation requires the secrets/list
    /// permission.
    pub fn get_secrets(
        &self,
        options: Option<SecretClientGetSecretsOptions<'_>>,
    ) -> Result<Pager<SecretListResult>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        first_url = first_url.join("secrets")?;
        first_url
            .query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        if let Some(maxresults) = options.maxresults {
            first_url
                .query_pairs_mut()
                .append_pair("maxresults", &maxresults.to_string());
        }
        let api_version = self.api_version.clone();
        Ok(Pager::from_callback(move |next_link: Option<Url>| {
            let url = match next_link {
                Some(next_link) => {
                    let qp = next_link
                        .query_pairs()
                        .filter(|(name, _)| name.ne("api-version"));
                    let mut next_link = next_link.clone();
                    next_link
                        .query_pairs_mut()
                        .clear()
                        .extend_pairs(qp)
                        .append_pair("api-version", &api_version);
                    next_link
                }
                None => first_url.clone(),
            };
            let mut request = Request::new(url, Method::Get);
            request.insert_header("accept", "application/json");
            let ctx = options.method_options.context.clone();
            let pipeline = pipeline.clone();
            async move {
                let rsp: Response<SecretListResult> = pipeline.send(&ctx, &mut request).await?;
                let (status, headers, body) = rsp.deconstruct();
                let bytes = body.collect().await?;
                let res: SecretListResult = json::from_json(bytes.clone())?;
                let rsp = Response::from_bytes(status, headers, bytes);
                Ok(match res.next_link {
                    Some(next_link) => PagerResult::Continue {
                        response: rsp,
                        continuation: next_link.parse()?,
                    },
                    None => PagerResult::Complete { response: rsp },
                })
            }
        }))
    }

    /// Permanently deletes the specified secret.
    ///
    /// The purge deleted secret operation removes the secret permanently, without the possibility of recovery. This operation
    /// can only be enabled on a soft-delete enabled vault. This operation requires the secrets/purge permission.
    pub async fn purge_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientPurgeDeletedSecretOptions<'_>>,
    ) -> Result<Response<()>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("deletedsecrets/{secret-name}");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Delete);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Recovers the deleted secret to the latest version.
    ///
    /// Recovers the deleted secret in the specified vault. This operation can only be performed on a soft-delete enabled vault.
    /// This operation requires the secrets/recover permission.
    pub async fn recover_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientRecoverDeletedSecretOptions<'_>>,
    ) -> Result<Response<SecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("deletedsecrets/{secret-name}/recover");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Restores a backed up secret to a vault.
    ///
    /// Restores a backed up secret, and all its versions, to a vault. This operation requires the secrets/restore permission.
    pub async fn restore_secret(
        &self,
        parameters: RequestContent<SecretRestoreParameters>,
        options: Option<SecretClientRestoreSecretOptions<'_>>,
    ) -> Result<Response<SecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        url = url.join("secrets/restore")?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(parameters);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Sets a secret in a specified key vault.
    ///
    /// The SET operation adds a secret to the Azure Key Vault. If the named secret already exists, Azure Key Vault creates a
    /// new version of that secret. This operation requires the secrets/set permission.
    pub async fn set_secret(
        &self,
        secret_name: &str,
        parameters: RequestContent<SecretSetParameters>,
        options: Option<SecretClientSetSecretOptions<'_>>,
    ) -> Result<Response<SecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}");
        path = path.replace("{secret-name}", secret_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(parameters);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Updates the attributes associated with a specified secret in a given key vault.
    ///
    /// The UPDATE operation changes specified attributes of an existing stored secret. Attributes that are not specified in the
    /// request are left unchanged. The value of a secret itself cannot be changed. This operation requires the secrets/set permission.
    pub async fn update_secret(
        &self,
        secret_name: &str,
        secret_version: &str,
        parameters: RequestContent<SecretUpdateParameters>,
        options: Option<SecretClientUpdateSecretOptions<'_>>,
    ) -> Result<Response<SecretBundle>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("secrets/{secret-name}/{secret-version}");
        path = path.replace("{secret-name}", secret_name);
        path = path.replace("{secret-version}", secret_version);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Patch);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(parameters);
        self.pipeline.send(&ctx, &mut request).await
    }
}

impl Default for SecretClientOptions {
    fn default() -> Self {
        Self {
            api_version: String::from("7.6-preview.1"),
            client_options: ClientOptions::default(),
        }
    }
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientBackupSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientDeleteSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetDeletedSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetDeletedSecretsOptions<'a> {
    pub maxresults: Option<i32>,
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetDeletedSecretsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetDeletedSecretsOptions<'static> {
        SecretClientGetDeletedSecretsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretVersionsOptions<'a> {
    pub maxresults: Option<i32>,
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetSecretVersionsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetSecretVersionsOptions<'static> {
        SecretClientGetSecretVersionsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretsOptions<'a> {
    pub maxresults: Option<i32>,
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetSecretsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetSecretsOptions<'static> {
        SecretClientGetSecretsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientPurgeDeletedSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientRecoverDeletedSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientRestoreSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientSetSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientUpdateSecretOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}
