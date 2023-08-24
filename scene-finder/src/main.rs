use catalyst::ContentId;

use std::string::String;

use serde_json::Value;
use catalyst::Server;
use core::time;
use std::thread;

use indicatif::ProgressBar;
use std::fs::File;
use chrono::{DateTime, Utc};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use dcl_common::{Parcel, Result};
use std::env;
use serde::Serialize;


const MIN_PARCEL: i16 = -152;
const MAX_PARCEL: i16 = 152;

#[derive(Serialize)]
struct ParcelPointer<'a> {
    pointers: &'a Vec<Parcel>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  
  let filename = args.get(1).unwrap();
  

  let scenes = generate_list().await?;

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

async fn generate_list() -> Result<Vec<Scene>> {
  let server = Server::production();
  let mut result = vec![];

  let mut parcels = vec![];

  for x in MIN_PARCEL..=MAX_PARCEL {
    for y in MIN_PARCEL..=MAX_PARCEL {
      parcels.push(Parcel(x, y));
    }
  }


  let pointers = &parcels.to_vec();

  let pointers = ParcelPointer {
    pointers
  };
  
  let scenes = server.raw_post("/content/entities/active", &pointers).await?;
  let scenes = scenes.json().await?;
  
  if let Value::Array(scenes) = scenes {
    let pb = ProgressBar::new(scenes.len().try_into().unwrap());
    for scene in &scenes {
      let scene = scene.as_object().unwrap();
      if let Some(content) = scene.get("content") {
        for object in content.as_array().unwrap() {
          let object = object.as_object().unwrap();
          if let Some(Value::String(file)) = object.get("file") {
            if file.contains("2dcl/scene.2dcl") {
              if let Some(hash) = object.get("hash") {
                add_scene(hash.to_string().replace('"', ""), &server, &mut result).await?;
              }
            }
          }
        }
        pb.inc(1);
      }
      thread::sleep(time::Duration::from_millis(1));
    }
  }
  Ok(result)
}

async fn add_scene(content_id: String, server: &Server, results: &mut Vec<Scene>) -> Result<()> {
  let url = format!("/content/contents/{}", content_id);
  let response = server
    .raw_get(url)
    .await?;

  let content = response.bytes().await?;

  let downloaded_scene = dcl2d_ecs_v1::Scene::from_mp(&content.to_vec())?;

  let scene = Scene {
    last_updated: downloaded_scene.timestamp.into(),
    title: downloaded_scene.name.clone(),
    start_parcel: downloaded_scene.base.clone(),
  };

  println!("Adding Scene: ({:?}) {}", scene.start_parcel, scene.title);
  results.push(scene);

  Ok(())

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

