use actions::{rest_api::RestApiAction, ActionTrait};
use clap::Arg;
mod actions;

 

fn main(){

    let cmd = clap::Command::new("cargo")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            clap::command!("tanker").arg(
                Arg::new("action")
            ),
        );
    let matches = cmd.get_matches();
    
    let matches = match matches.subcommand() {
        Some(("tanker", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };


    let action = match matches.get_one::<String>("action") {
        Some(action) => action,
        _ => panic!("Invalid action")
    };

    let target_folder = "./src".to_owned();

    if action == "rest-api"{
        let action = RestApiAction::new(target_folder);
        action.create_folder();
    }
    
}