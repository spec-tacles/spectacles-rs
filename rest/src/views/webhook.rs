use futures::Future;
use reqwest::Method;
use reqwest::r#async::multipart::{Form, Part};

use spectacles_model::message::{ExecuteWebhookOptions, Message, ModifyWebhookOptions, Webhook};

use crate::{Endpoint, Error, RestClient};

/// A view for managing Discord webhooks.
pub struct WebhookView {
    id: u64,
    client: RestClient,
}

impl WebhookView {
    pub(crate) fn new(id: u64, client: RestClient) -> Self {
        Self {
            id,
            client,
        }
    }

    /// Returns a webhook object for the provided user ID.
    pub fn get(&self) -> impl Future<Item=Webhook, Error=Error> {
        self.client.request(Endpoint::new(Method::GET, format!("/webhooks/{}", self.id)))
    }

    /// Similar to [`method.get.html`], but accepts a webhook token. The returned webhook does not have a User object.
    pub fn get_with_token(&self, token: &str) -> impl Future<Item=Webhook, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/webhooks/{}/{}", self.id, token),
        ))
    }

    /// Modifies the webhook at the provided webhook ID.
    /// This endpoint requires the `MANAGE_WEBHOOKS` permission.
    pub fn modify(&self, opts: ModifyWebhookOptions) -> impl Future<Item=Webhook, Error=Error> {
        self.client.request(Endpoint::new(
            Method::PATCH,
            format!("/webhooks/{}", self.id),
        ).json(opts))
    }

    /// Similar to [`method.modify.html`], but accepts a webhook token. The returned webhook does not have a User object.
    pub fn modify_with_token(&self, token: &str, opts: ModifyWebhookOptions) -> impl Future<Item=Webhook, Error=Error> {
        self.client.request(Endpoint::new(
            Method::PATCH,
            format!("/webhooks/{}/{}", self.id, token),
        ).json(opts))
    }

    /// Permanently deletes this webhook.
    pub fn delete(&self) -> impl Future<Item=(), Error=Error> {
        self.client.request_empty(Endpoint::new(
            Method::DELETE,
            format!("/webhooks/{}", self.id),
        ))
    }

    /// Similar to [`method.delete.html`], but accepts a webhook token.
    pub fn delete_with_token(&self, token: &str) -> impl Future<Item=(), Error=Error> {
        self.client.request_empty(Endpoint::new(
            Method::DELETE,
            format!("/webhooks/{}/{}", self.id, token),
        ))
    }

    /// Executes the provided webhook, with the provided options.
    pub fn execute(&self, token: &str, opts: ExecuteWebhookOptions, wait: bool) -> impl Future<Item=Option<Message>, Error=Error> {
        let endpt = Endpoint::new(Method::POST, format!("/webhooks/{}/{}", self.id, token));
        let json = serde_json::to_string(&opts).expect("Failed to serialize webhook message");
        if let Some((name, file)) = opts.file {
            self.client.request(endpt.multipart(
                Form::new()
                    .part("file", Part::bytes(file).file_name(name))
                    .part("payload_json", Part::text(json))
            ).query(json!({ "wait": wait })))
        } else {
            self.client.request(endpt.json(opts).query(json!({
                "wait": wait
            })))
        }
    }
}