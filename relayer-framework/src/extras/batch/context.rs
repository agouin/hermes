use async_trait::async_trait;

use crate::std_prelude::*;
use crate::traits::contexts::relay::RelayContext;
use crate::traits::core::Async;
use crate::traits::target::ChainTarget;
use crate::types::aliases::{IbcEvent, IbcMessage};

#[async_trait]
pub trait BatchContext: Async {
    type Error: Async;

    type Message;
    type Event;

    type MessagesSender: Async;
    type MessagesReceiver: Async;

    type ResultSender: Async;
    type ResultReceiver: Async;

    fn new_messages_channel(&self) -> (Self::MessagesSender, Self::MessagesReceiver);

    fn new_result_channel(&self) -> (Self::ResultSender, Self::ResultReceiver);

    async fn send_messages(
        sender: &Self::MessagesSender,
        messages: Vec<Self::Message>,
        result_sender: Self::ResultSender,
    ) -> Result<(), Self::Error>;

    async fn try_receive_messages(
        receiver: &mut Self::MessagesReceiver,
    ) -> Result<Option<(Vec<Self::Message>, Self::ResultSender)>, Self::Error>;

    async fn receive_result(
        result_receiver: Self::ResultReceiver,
    ) -> Result<Result<Vec<Vec<Self::Event>>, Self::Error>, Self::Error>;

    fn send_result(
        result_sender: Self::ResultSender,
        events: Result<Vec<Vec<Self::Event>>, Self::Error>,
    ) -> Result<(), Self::Error>;
}

pub trait HasBatchContext<Target>: RelayContext
where
    Target: ChainTarget<Self>,
{
    type BatchContext: BatchContext<
        Message = IbcMessage<Target::TargetChain, Target::CounterpartyChain>,
        Event = IbcEvent<Target::TargetChain, Target::CounterpartyChain>,
        Error = Self::Error,
    >;

    fn batch_context(&self) -> &Self::BatchContext;

    fn messages_sender(&self) -> &<Self::BatchContext as BatchContext>::MessagesSender;
}