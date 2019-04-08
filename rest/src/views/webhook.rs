use futures::Future;
use reqwest::async::multipart::{Form, Part};
use reqwest::Method;

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
            format!("/webhooks/{}/{}", self.id.token),
        ).json(opts))
    }

    /// Permanently deletes this webhook.
    pub fn delete(&self) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/webhooks/{}", self.id),
        ))
    }

    /// Similar to [`method.delete.html`], but accepts a webhook token.
    pub fn delete_with_token(&self, token: &str) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/webhooks/{}/{}", self.id.token),
        ).json(opts))
    }

    /// Executes the provided webhook, with the provided options.
    pub fn execute(&self, opts: ExecuteWebhookOptions, wait: bool) -> impl Future<Item=Option<Message>, Error=Error> {
        let endpt = Endpoint::new(Method::POST, format!("/webhooks/{}/{}", self.id.token));
        let query = json!({
            wait: bool
        });
        let json = serde_json::to_string(&opts).expect("Failed to serialize webhook message");
        if let Some((name, mut file)) = opts.file {
            let mut chunks = vec![];
            file.read_buf(&mut chunks).expect("Failed to process provided file attachment");

            self.client.request(endpt.multipart(
                Form::new()
                    .part("file", Part::bytes(chunks).file_name(name))
                    .part("payload_json", Part::text(json))
            ))
        } else {
            self.client.request(endpt.json(opts))
        }
    }
}