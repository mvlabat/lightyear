use crate::prelude::{Channel, ChannelKind, Message};
use crate::shared::replication::network_target::NetworkTarget;
use bevy::prelude::Resource;
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) trait MessageSend: Resource {
    type Error: Error;
    fn send_message_to_target<C: Channel, M: Message>(
        &mut self,
        message: &M,
        target: NetworkTarget,
    ) -> Result<(), Self::Error>;

    fn erased_send_message_to_target<M: Message>(
        &mut self,
        message: &M,
        channel_kind: ChannelKind,
        target: NetworkTarget,
    ) -> Result<(), Self::Error>;
}
