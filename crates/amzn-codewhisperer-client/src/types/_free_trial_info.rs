// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FreeTrialInfo {
    /// Status of the free trial for this customer
    pub free_trial_status: ::std::option::Option<crate::types::FreeTrialStatus>,
    /// Unix timestamp of free trial expiry in seconds
    pub free_trial_expiry: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// Current free trial usage
    pub current_usage: ::std::option::Option<i32>,
    /// Free trial usage limit
    pub usage_limit: ::std::option::Option<i32>,
}
impl FreeTrialInfo {
    /// Status of the free trial for this customer
    pub fn free_trial_status(&self) -> ::std::option::Option<&crate::types::FreeTrialStatus> {
        self.free_trial_status.as_ref()
    }

    /// Unix timestamp of free trial expiry in seconds
    pub fn free_trial_expiry(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.free_trial_expiry.as_ref()
    }

    /// Current free trial usage
    pub fn current_usage(&self) -> ::std::option::Option<i32> {
        self.current_usage
    }

    /// Free trial usage limit
    pub fn usage_limit(&self) -> ::std::option::Option<i32> {
        self.usage_limit
    }
}
impl FreeTrialInfo {
    /// Creates a new builder-style object to manufacture
    /// [`FreeTrialInfo`](crate::types::FreeTrialInfo).
    pub fn builder() -> crate::types::builders::FreeTrialInfoBuilder {
        crate::types::builders::FreeTrialInfoBuilder::default()
    }
}

/// A builder for [`FreeTrialInfo`](crate::types::FreeTrialInfo).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct FreeTrialInfoBuilder {
    pub(crate) free_trial_status: ::std::option::Option<crate::types::FreeTrialStatus>,
    pub(crate) free_trial_expiry: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) current_usage: ::std::option::Option<i32>,
    pub(crate) usage_limit: ::std::option::Option<i32>,
}
impl FreeTrialInfoBuilder {
    /// Status of the free trial for this customer
    pub fn free_trial_status(mut self, input: crate::types::FreeTrialStatus) -> Self {
        self.free_trial_status = ::std::option::Option::Some(input);
        self
    }

    /// Status of the free trial for this customer
    pub fn set_free_trial_status(mut self, input: ::std::option::Option<crate::types::FreeTrialStatus>) -> Self {
        self.free_trial_status = input;
        self
    }

    /// Status of the free trial for this customer
    pub fn get_free_trial_status(&self) -> &::std::option::Option<crate::types::FreeTrialStatus> {
        &self.free_trial_status
    }

    /// Unix timestamp of free trial expiry in seconds
    pub fn free_trial_expiry(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.free_trial_expiry = ::std::option::Option::Some(input);
        self
    }

    /// Unix timestamp of free trial expiry in seconds
    pub fn set_free_trial_expiry(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.free_trial_expiry = input;
        self
    }

    /// Unix timestamp of free trial expiry in seconds
    pub fn get_free_trial_expiry(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.free_trial_expiry
    }

    /// Current free trial usage
    pub fn current_usage(mut self, input: i32) -> Self {
        self.current_usage = ::std::option::Option::Some(input);
        self
    }

    /// Current free trial usage
    pub fn set_current_usage(mut self, input: ::std::option::Option<i32>) -> Self {
        self.current_usage = input;
        self
    }

    /// Current free trial usage
    pub fn get_current_usage(&self) -> &::std::option::Option<i32> {
        &self.current_usage
    }

    /// Free trial usage limit
    pub fn usage_limit(mut self, input: i32) -> Self {
        self.usage_limit = ::std::option::Option::Some(input);
        self
    }

    /// Free trial usage limit
    pub fn set_usage_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.usage_limit = input;
        self
    }

    /// Free trial usage limit
    pub fn get_usage_limit(&self) -> &::std::option::Option<i32> {
        &self.usage_limit
    }

    /// Consumes the builder and constructs a [`FreeTrialInfo`](crate::types::FreeTrialInfo).
    pub fn build(self) -> crate::types::FreeTrialInfo {
        crate::types::FreeTrialInfo {
            free_trial_status: self.free_trial_status,
            free_trial_expiry: self.free_trial_expiry,
            current_usage: self.current_usage,
            usage_limit: self.usage_limit,
        }
    }
}
