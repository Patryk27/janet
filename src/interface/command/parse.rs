use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{
    Command,
    CommandAction,
    MergeRequestCommand,
    MergeRequestCommandContext,
    ParseError,
    ParseResult,
};
use nom::branch::alt;
use nom::combinator::value;
use nom::IResult;

mod merge_request_command;

impl MergeRequestCommand {
    #[tracing::instrument]
    pub fn parse(ctxt: MergeRequestCommandContext, cmd: &str) -> ParseResult<Command> {
        tracing::debug!("Parsing command");

        merge_request_command::parse(cmd)
            .map(|(_, cmd)| Command::MergeRequest { ctxt, cmd })
            .map_err(|_| ParseError::UnknownCommand)
    }
}

fn action(i: &str) -> IResult<&str, CommandAction> {
    let add = value(CommandAction::Add, char('+'));
    let remove = value(CommandAction::Remove, char('-'));

    alt((add, remove))(i)
}
