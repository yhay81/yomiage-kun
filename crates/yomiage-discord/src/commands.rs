use serenity::{
    all::Permissions,
    builder::CreateCommand,
    model::application::{Command, InstallationContext, InteractionContext},
    prelude::Context,
};

const COMMANDS: [(&str, &str); 4] = [
    ("join", "現在のボイスチャンネルに参加します"),
    ("leave", "読み上げを終了して退出します"),
    ("skip", "現在の読み上げをスキップします"),
    ("status", "Botと読み上げキューの状態を表示します"),
];

pub fn definitions() -> Vec<CreateCommand> {
    COMMANDS
        .iter()
        .map(|(name, description)| {
            let command = CreateCommand::new(*name)
                .description(*description)
                .integration_types(vec![InstallationContext::Guild])
                .contexts(vec![InteractionContext::Guild]);
            if matches!(*name, "join" | "leave") {
                command.default_member_permissions(Permissions::MANAGE_GUILD)
            } else {
                command
            }
        })
        .collect()
}

pub async fn register(ctx: &Context) -> serenity::Result<()> {
    Command::set_global_commands(&ctx.http, definitions()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_names_are_unique() {
        let mut names = COMMANDS.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), COMMANDS.len());
    }
}
