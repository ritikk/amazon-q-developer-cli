// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use ::aws_smithy_runtime_api::client::endpoint::{
    EndpointFuture,
    SharedEndpointResolver,
};
pub use ::aws_smithy_types::endpoint::Endpoint;

#[cfg(test)]
mod test {}

/// Endpoint resolver trait specific to Amazon Q Developer
pub trait ResolveEndpoint: ::std::marker::Send + ::std::marker::Sync + ::std::fmt::Debug {
    /// Resolve an endpoint with the given parameters
    fn resolve_endpoint<'a>(
        &'a self,
        params: &'a crate::config::endpoint::Params,
    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a>;

    /// Convert this service-specific resolver into a `SharedEndpointResolver`
    ///
    /// The resulting resolver will downcast `EndpointResolverParams` into
    /// `crate::config::endpoint::Params`.
    fn into_shared_resolver(self) -> ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver
    where
        Self: Sized + 'static,
    {
        ::aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver::new(DowncastParams(self))
    }
}

#[derive(Debug)]
struct DowncastParams<T>(T);
impl<T> ::aws_smithy_runtime_api::client::endpoint::ResolveEndpoint for DowncastParams<T>
where
    T: ResolveEndpoint,
{
    fn resolve_endpoint<'a>(
        &'a self,
        params: &'a ::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams,
    ) -> ::aws_smithy_runtime_api::client::endpoint::EndpointFuture<'a> {
        let ep = match params.get::<crate::config::endpoint::Params>() {
            Some(params) => self.0.resolve_endpoint(params),
            None => ::aws_smithy_runtime_api::client::endpoint::EndpointFuture::ready(Err(
                "params of expected type was not present".into(),
            )),
        };
        ep
    }
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Configuration parameters for resolving the correct endpoint
pub struct Params {}
impl Params {
    /// Create a builder for [`Params`]
    pub fn builder() -> crate::config::endpoint::ParamsBuilder {
        crate::config::endpoint::ParamsBuilder::default()
    }
}

/// Builder for [`Params`]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ParamsBuilder {}
impl ParamsBuilder {
    /// Consume this builder, creating [`Params`].
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::config::endpoint::Params, crate::config::endpoint::InvalidParams> {
        Ok(
            #[allow(clippy::unnecessary_lazy_evaluations)]
            crate::config::endpoint::Params {},
        )
    }
}

/// An error that occurred during endpoint resolution
#[derive(Debug)]
pub struct InvalidParams {
    field: std::borrow::Cow<'static, str>,
}

impl InvalidParams {
    #[allow(dead_code)]
    fn missing(field: &'static str) -> Self {
        Self { field: field.into() }
    }
}

impl std::fmt::Display for InvalidParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a required field was missing: `{}`", self.field)
    }
}

impl std::error::Error for InvalidParams {}
