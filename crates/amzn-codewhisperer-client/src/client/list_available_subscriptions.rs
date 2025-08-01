// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`ListAvailableSubscriptions`](crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder takes no input, just
    ///   [`send`](crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsFluentBuilder::send)
    ///   it.
    /// - On success, responds with
    ///   [`ListAvailableSubscriptionsOutput`](crate::operation::list_available_subscriptions::ListAvailableSubscriptionsOutput)
    ///   with field(s):
    ///   - [`subscription_plans(Vec::<SubscriptionPlan>)`](crate::operation::list_available_subscriptions::ListAvailableSubscriptionsOutput::subscription_plans): (undocumented)
    /// - On failure, responds with [`SdkError<ListAvailableSubscriptionsError>`](crate::operation::list_available_subscriptions::ListAvailableSubscriptionsError)
    pub fn list_available_subscriptions(
        &self,
    ) -> crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsFluentBuilder {
        crate::operation::list_available_subscriptions::builders::ListAvailableSubscriptionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
