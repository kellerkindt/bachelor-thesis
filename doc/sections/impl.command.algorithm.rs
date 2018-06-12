enum Command<U: Send+Debug, I: Send+Debug+Sized+'static> {
    Update(Box<U>),
    Publish(RawMessage),
    SubscribeEnvironmentModel(I, EnvironmentListener),
    UnsubscribeEnvironmentModel(I),
    SubscribeListenerCount(I, CountListener),
    UnsubscribeListenerCount(I),
    ActivateEnvironmentModelSubscription(I),
    DeactivateEnvironmentModelSubscription(I),
}
