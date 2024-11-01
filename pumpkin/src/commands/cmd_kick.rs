use async_trait::async_trait;
use pumpkin_core::text::color::NamedColor;
use pumpkin_core::text::TextComponent;

use crate::commands::arg_player::parse_arg_player;
use crate::commands::tree::CommandTree;
use crate::commands::tree_builder::argument;

use super::arg_player::PlayerArgumentConsumer;
use super::CommandExecutor;

const NAMES: [&str; 1] = ["kick"];
const DESCRIPTION: &str = "Kicks the target player from the server.";

const ARG_TARGET: &str = "target";

struct KickExecutor {}

#[async_trait]
impl CommandExecutor for KickExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut super::CommandSender<'a>,
        server: &crate::server::Server,
        args: &super::tree::ConsumedArgs<'a>,
    ) -> Result<(), super::dispatcher::InvalidTreeError> {
        let target = parse_arg_player(sender, server, ARG_TARGET, args).await?;
        target
            .kick(TextComponent::text("Kicked by an operator"))
            .await;

        sender
            .send_message(
                TextComponent::text("Player has been kicked.").color_named(NamedColor::Blue),
            )
            .await;

        Ok(())
    }
}

pub fn init_command_tree<'a>() -> CommandTree<'a> {
    CommandTree::new(NAMES, DESCRIPTION)
        .with_child(argument(ARG_TARGET, &PlayerArgumentConsumer {}).execute(&KickExecutor {}))
}
