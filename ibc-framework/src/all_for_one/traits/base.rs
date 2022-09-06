use crate::all_for_one::traits::error::AfoErrorContext;
use crate::all_for_one::traits::event::AfoEventContext;
use crate::core::traits::client::HasAnyClientMethods;
use crate::core::traits::client_reader::HasAnyClientReader;
use crate::core::traits::client_writer::HasAnyClientWriter;
use crate::core::traits::event::HasEventEmitter;
use crate::core::traits::handlers::update_client::HasAnyUpdateClientHandler;
use crate::core::traits::host::HasHostMethods;
use crate::core::traits::ibc::HasIbcMethods;
use crate::core::traits::messages::ibc::HasIbcMessages;
use crate::core::traits::messages::update_client::HasUpdateClientMessageHandler;

pub trait AfoChainContext:
    AfoErrorContext
    + AfoEventContext
    + HasIbcMethods
    + HasAnyClientMethods
    + HasAnyClientReader
    + HasAnyClientWriter
    + HasAnyUpdateClientHandler
    + HasHostMethods
    + HasIbcMessages
    + HasEventEmitter
    + HasUpdateClientMessageHandler
{
}

impl<Context> AfoChainContext for Context where
    Context: AfoErrorContext
        + AfoEventContext
        + HasIbcMethods
        + HasAnyClientMethods
        + HasAnyClientReader
        + HasAnyClientWriter
        + HasAnyUpdateClientHandler
        + HasHostMethods
        + HasIbcMessages
        + HasEventEmitter
        + HasUpdateClientMessageHandler
{
}