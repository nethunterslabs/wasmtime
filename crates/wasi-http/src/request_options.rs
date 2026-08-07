use http_acl::HttpAcl;
use std::sync::Arc;
use std::time::Duration;

/// The concrete type behind a `wasi:http/types.request-options` resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RequestOptions {
    /// How long to wait for a connection to be established.
    pub connect_timeout: Option<Duration>,
    /// How long to wait for the first byte of the response body.
    pub first_byte_timeout: Option<Duration>,
    /// How long to wait between frames of the response body.
    pub between_bytes_timeout: Option<Duration>,
    /// The ACL to check the outgoing request against, attached by
    /// [`crate::p2::http_impl`]/[`crate::p3::host::handler`] so it reaches
    /// [`crate::default_send_request`] regardless of which `send_request`
    /// implementation ultimately handles the request. Not part of the
    /// `wasi:http/types.request-options` resource itself - a guest can't see
    /// or set this field.
    pub(crate) acl: Option<Arc<HttpAcl>>,
}
