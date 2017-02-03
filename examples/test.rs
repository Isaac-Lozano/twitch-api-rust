extern crate twitch_api;

use twitch_api::TwitchApi;

const CLIENT_ID: &'static str = "8t59b442f4twnhw5ur8vw7vf5muoauf";

fn main()
{
    let tapi = TwitchApi::new(CLIENT_ID);
    let user = tapi.get_user("milessmb").unwrap().users.get(0).unwrap().clone();
    println!("{:#?}", user);
    println!("{:#?}", tapi.get_subscriber_badges(user.id).unwrap());
    println!("{:#?}", tapi.get_emoticons(vec![0]).unwrap());
}
