use crate::discord_bot_types;
use crate::lol;
use std::env;

pub async fn execute_played_command(lol_api_fetcher: &lol::api_fetcher::BoundedHttpFetcher, played_command: discord_bot_types::Command) -> Result<String, discord_bot_types::BotError> {
    
    println!("Executing played command");
    let command = build_played_command(played_command.options)?;

    let api_key = env::var("LOL_API_KEY").map_err(|err| discord_bot_types::BotError {
        statusCode: 500,
        body: "Missing LOL API key".to_string()
    })?;

    let puuid = lol::get_puuid(&lol_api_fetcher, "euw1", &command.player_name, &api_key).await?;
    let game_ids = lol::get_game_ids(&lol_api_fetcher, &api_key, "europe", &puuid, command.days).await?;
    let models = lol::fetch_game_summaries(&lol_api_fetcher, &api_key, "europe", &puuid, game_ids).await?;

    let played_for: u64 = calculate_time_played(&models);
    let time_played_string: String = create_time_played_string(played_for);
    let wins = calculate_wins(&models);
    let loses = calculate_loses(&models);

    // TODO: Add in-game scores and link to more detailed view of game
    return Ok(format!("{} has played for {} over {} days\n
    They won {} games and lost {}
    ", command.player_name, time_played_string, command.days, wins, loses));
}

fn calculate_time_played(summaries: &Vec<lol::models::UserGameSummary>) -> u64 {
    return summaries.iter().map(|x| x.game_duration_millis).sum();
}

fn calculate_wins(summaries: &Vec<lol::models::UserGameSummary>) -> u64 {
    return summaries.iter().map(|x| if x.participant.win == true {1} else {0}).sum();
}

fn calculate_loses(summaries: &Vec<lol::models::UserGameSummary>) -> u64 {
    return summaries.iter().map(|x| if x.participant.win == true {0} else {1}).sum();
}

fn create_time_played_string(millis: u64) -> String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    
    let minutes = minutes % 60;
    return format!("{} hours and {} minutes", hours, minutes);
}

fn build_played_command(command_options: Vec<discord_bot_types::CommandOption>) -> Result<discord_bot_types::PlayedCommand, discord_bot_types::BotError> {
    let player_name = command_options.iter().find_map(|x| match x {
        discord_bot_types::CommandOption::NumberCommandOption(x) => None,
        discord_bot_types::CommandOption::StringCommandOption(option) => {
            if option.name == "user" {Some(&option.value)} else {None}
        },
    }).ok_or(discord_bot_types::BotError {
        statusCode: 500,
        body: "Could not find player name".to_string()
    })?;

    let days_requested = command_options.iter().find_map(|x| match x {
        discord_bot_types::CommandOption::NumberCommandOption(option) => {if option.name == "days" {Some(option.value)} else {None}},
        discord_bot_types::CommandOption::StringCommandOption(option) => None,
    }).ok_or(discord_bot_types::BotError {
        statusCode: 500,
        body: "Could not find player name".to_string()
    })?;

    return Ok(discord_bot_types::PlayedCommand {
        player_name: player_name.to_string(),
        days: days_requested
    });
}