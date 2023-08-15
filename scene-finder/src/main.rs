
use std::fs::File;
use chrono::{DateTime, Utc};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use dcl_common::Parcel;
use std::env;


fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  
  let filename = args.get(1).unwrap();

  // TODO(fran): Update this with the actual list generation.
  // let scenes = generate_list();
  let scenes = vec![Scene {
    title: "Test Scene".to_string(),
    start_parcel: Parcel(10,10),
    last_updated: Utc::now()
  }];

  let channel = generate_rss(&scenes);

  let file = File::create(filename)?;
  channel.write_to(file).unwrap(); 
  Ok(())
}


struct Scene {
  title: String,
  start_parcel: Parcel,
  last_updated: DateTime<Utc>
}

impl Scene {
  fn as_item(&self) -> Item {
    ItemBuilder::default()
      .title(self.title.clone())
      .link(self.link())
      .pub_date(self.last_updated.to_rfc2822())
      .build()
  }

  fn link(&self) -> String {
    format!(
      "2dcl://scenes/{}/{}", 
      self.start_parcel.0, 
      self.start_parcel.1
    )
  }
}


fn generate_list() -> Vec<Scene> {    
    vec![]
}

const RSS_TITLE : &str = "2dcl Scenes";
const RSS_LINK : &str = "https://2dcl.org/scenes";
const RSS_DESCRIPTION : &str = 
  "An RSS feed of the currently deployed 2dcl scenes. Updated every 24hs.";

fn generate_rss(scenes: &Vec<Scene>) -> Channel {
  let mut channel = ChannelBuilder::default()
    .title(RSS_TITLE)
    .link(RSS_LINK)
    .description(RSS_DESCRIPTION)
    .last_build_date(Utc::now().to_rfc2822())
    .build();

  for scene in scenes {
    channel.items.push( scene.as_item() );
  }


  channel
}
#[cfg(test)]
mod test {
    use super::*;

  #[test]
  fn generate_list_gets_info_from_catalyst() {
    // ... todo
  }
  
  #[test]
  fn as_item_returns_scene_correctly() {
    let scene = Scene {
      title: "Test Scene".to_string(),
      start_parcel: Parcel(10,10),
      last_updated: Utc::now()
    };

    let item = scene.as_item();

    assert_eq!(item.title, Some(scene.title.clone()));
    assert_eq!(item.link, Some(scene.link()));
    assert_eq!(item.pub_date, Some(scene.last_updated.to_rfc2822()));
  }

  #[test]
  fn generate_rss_creates_an_rss_feed_from_existing_scenes() {
    let scenes = vec![Scene {
      title: "Test Scene".to_string(),
      start_parcel: Parcel(10,10),
      last_updated: Utc::now()
    }];

    let result = generate_rss(&scenes);

    assert_eq!(result.title, RSS_TITLE);
    assert_eq!(result.link, RSS_LINK);
    assert_eq!(result.description, RSS_DESCRIPTION);
    assert_eq!(result.last_build_date, Some(Utc::now().to_rfc2822()));
    assert_eq!(result.items[0], scenes[0].as_item())
  }

}

