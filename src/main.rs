use clap::Parser;
use actions::{rest_api::RestApiAction, ActionTrait};

mod actions;


#[derive(clap::Subcommand, Debug)]
enum Action {
   RestApi
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    // Action
    #[command(subcommand)]
    action: Action,
}

fn main(){
    let args = Args::parse();

    let target_folder = "./src".to_owned();
    match &args.action{
        Action::RestApi => {
            let action = RestApiAction::new(target_folder);
            action.create_folder();
        },
        
    }
}