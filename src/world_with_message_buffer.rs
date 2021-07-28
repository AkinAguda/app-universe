use crate::{AppWorld, AppWorldWrapper};
use std::ops::Deref;

/// Allows us to optionally store sent `Msg`'s instead of passing them on to state.
///
/// This is useful for testing that an event handler dispatched the `Msg`'s that we expected it to.
pub struct WorldWithMessageBuffer<W: AppWorld> {
    world: W,
    #[cfg(feature = "test-utils")]
    message_buffer: Vec<W::Message>,
    #[cfg(feature = "test-utils")]
    capture_messages: bool,
}

impl<W: AppWorld> WorldWithMessageBuffer<W> {
    /// Create a new state and message buffer.
    pub fn new(state: W) -> Self {
        WorldWithMessageBuffer {
            world: state,
            #[cfg(feature = "test-utils")]
            message_buffer: Vec::new(),
            #[cfg(feature = "test-utils")]
            capture_messages: false,
        }
    }
}

#[cfg(feature = "test-utils")]
impl<W: AppWorld> AppWorldWrapper<W> {
    /// Set whether or not messages get pushed to the message buffer.
    pub fn set_capture_messages(&mut self, capture: bool) {
        self.world.write().unwrap().capture_messages = capture;
    }
}

#[cfg(feature = "test-utils")]
impl<W: AppWorld> WorldWithMessageBuffer<W> {
    /// After calling `StateWithMessageBuffer.capture_messages(true)`, all .msg() calls will push to this
    /// buffer.
    pub fn message_buffer(&self) -> &Vec<W::Message> {
        &self.message_buffer
    }
}

impl<W: AppWorld> WorldWithMessageBuffer<W> {
    pub(super) fn message_maybe_capture(
        &mut self,
        message: W::Message,
        world_wrapper: AppWorldWrapper<W>,
    ) {
        #[cfg(feature = "test-utils")]
        if self.capture_messages {
            return self.message_buffer.push(message);
        }

        self.world.msg(message, world_wrapper);
    }
}

impl<W: AppWorld> Deref for WorldWithMessageBuffer<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}
