use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

use ps2api::events::{self, api_command::Subscribe, EventClient};
use ps2api::utils::CensusError;

#[tokio::main]
async fn main() {
    let mut achievement_ids: HashSet<u64> = HashSet::new();

    let mut fs_opts = fs::OpenOptions::new();
    fs_opts.read(true);
    fs_opts.write(true);
    fs_opts.create(true);
    let path = Path::new("./achievement_ids.txt");
    let mut file = fs_opts.open(path).unwrap();

    {
        let mut file_string = String::new();
        let _ = file.read_to_string(&mut file_string);
        let mut itr = file_string.split('\n');

        loop {
            let opt = itr.next();

            match opt {
                None => {
                    break;
                }
                Some(id_str) => {
                    let id = id_str.parse::<u64>();
                    if id.is_err() {
                        break;
                    }

                    achievement_ids.insert(id.unwrap());
                }
            }
        }
    }

    println!("Loaded {} ids", achievement_ids.len());


    let mut evc = {
        let mut try_evc = events::connect(events::environments::PC, "example")
            .await;
        match try_evc {
            Ok(e) => {
                e
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    };

    let server_ids = Some(vec!["all".to_string()]);

    let subs = vec![events::api_events::event_types::ApiSubscriptionName::AchievementEarned];

    let character_ids = Some(vec!["all".to_string()]);

    let sub = Subscribe {
        subscription_names: subs,
        character_ids,
        server_ids,
        match_chars_and_world: Some(true),
    };

    let _ = evc.send(&sub).await;

    println!("Listening for new achievement ids");
    loop {
        let event = evc.next_event().await;

        match event {
            Ok(e) => {
                if let events::api_events::event_types::ApiEvent::AchievementEarned(a) = e {
                    if !achievement_ids.contains(&a.achievement_id) {
                        let msg = "[".to_string()
                            + &SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string()
                            + "] New id found: "
                            + &a.achievement_id.to_string();
                        println!("{}", msg);
                        achievement_ids.insert(a.achievement_id);
                        file.write((a.achievement_id.to_string() + "\n").as_bytes());
                    }
                }
            }
            Err(err) => {
                println!("Census Error: {:?}", err);
            }
        }
    }
}
