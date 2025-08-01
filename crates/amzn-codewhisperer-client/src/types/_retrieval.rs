// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Retrieval {
    #[allow(missing_docs)] // documentation missing in model
    pub score: ::std::option::Option<f64>,
    #[allow(missing_docs)] // documentation missing in model
    pub content: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub uri: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub filepath: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub repository: ::std::option::Option<::std::string::String>,
}
impl Retrieval {
    #[allow(missing_docs)] // documentation missing in model
    pub fn score(&self) -> ::std::option::Option<f64> {
        self.score
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn content(&self) -> &str {
        use std::ops::Deref;
        self.content.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn uri(&self) -> ::std::option::Option<&str> {
        self.uri.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn filepath(&self) -> ::std::option::Option<&str> {
        self.filepath.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn repository(&self) -> ::std::option::Option<&str> {
        self.repository.as_deref()
    }
}
impl Retrieval {
    /// Creates a new builder-style object to manufacture [`Retrieval`](crate::types::Retrieval).
    pub fn builder() -> crate::types::builders::RetrievalBuilder {
        crate::types::builders::RetrievalBuilder::default()
    }
}

/// A builder for [`Retrieval`](crate::types::Retrieval).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RetrievalBuilder {
    pub(crate) score: ::std::option::Option<f64>,
    pub(crate) content: ::std::option::Option<::std::string::String>,
    pub(crate) uri: ::std::option::Option<::std::string::String>,
    pub(crate) filepath: ::std::option::Option<::std::string::String>,
    pub(crate) repository: ::std::option::Option<::std::string::String>,
}
impl RetrievalBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn score(mut self, input: f64) -> Self {
        self.score = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_score(mut self, input: ::std::option::Option<f64>) -> Self {
        self.score = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_score(&self) -> &::std::option::Option<f64> {
        &self.score
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.content
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.uri = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.uri = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_uri(&self) -> &::std::option::Option<::std::string::String> {
        &self.uri
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn filepath(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filepath = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_filepath(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filepath = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_filepath(&self) -> &::std::option::Option<::std::string::String> {
        &self.filepath
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn repository(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.repository = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_repository(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.repository = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_repository(&self) -> &::std::option::Option<::std::string::String> {
        &self.repository
    }

    /// Consumes the builder and constructs a [`Retrieval`](crate::types::Retrieval).
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](crate::types::builders::RetrievalBuilder::content)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::Retrieval, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Retrieval {
            score: self.score,
            content: self.content.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "content",
                    "content was not specified but it is required when building Retrieval",
                )
            })?,
            uri: self.uri,
            filepath: self.filepath,
            repository: self.repository,
        })
    }
}
