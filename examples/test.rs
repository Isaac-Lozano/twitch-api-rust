extern crate twitch_api;

use twitch_api::TwitchApi;

/* Add your client id here */
const CLIENT_ID: &'static str = "";

fn main()
{
    let tapi = TwitchApi::new(CLIENT_ID).unwrap();
    let user = tapi.get_user("darktwinge").unwrap().users.get(0).unwrap().clone();
    println!("{:#?}", user);
    println!("{:#?}", tapi.get_subscriber_badges(user.id).unwrap());
}
