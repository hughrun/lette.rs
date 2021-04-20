use chrono::{SecondsFormat, Utc};
use clap::{Arg, App, ArgMatches};
use colol::{color, close_color};
use subprocess::{Exec, ExitStatus, Popen, PopenConfig};
use itertools::join;
use reqwest;
use rss::Channel;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{BufReader, self, Read, Write};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use serde_derive::Deserialize;

// deserialize TOML file
#[derive(Deserialize)]
struct Commands {
    #[serde(default = "default_blank")]
    process: String,
    #[serde(default = "default_publish")]
    publish: String,
    #[serde(default = "default_blank")]
    test: String
}

#[derive(Deserialize)]
struct Social {
  #[serde(default = "default_blank")]
  mastodon_access_token: String,
  #[serde(default = "default_blank")]
  mastodon_base_url: String,
  #[serde(default = "default_blank")]
  twitter_consumer_key: String,
  #[serde(default = "default_blank")]
  twitter_consumer_secret: String,
  #[serde(default = "default_blank")]
  twitter_access_token: String,
  #[serde(default = "default_blank")]
  twitter_access_secret: String
  }

#[derive(Deserialize)]
struct Config {
    author: String,
    input: String,
    output: String,
    workdir: String,
    remote_dir: String,
    #[serde(default = "default_blank")]
    rss_file: String,
    #[serde(default = "default_blank")]
    unsplash_client_id: String,
    server_name: String,
    #[serde(default = "default_blank")]
    test_url: String,
    commands: Commands,
    social: Social,
    #[serde(default = "default_ssg")]
    ssg_type: String,
    #[serde(default = "default_layout")]
    default_layout: String
}

// Derive default values

fn default_blank() -> String {
    String::from("")
}

fn default_layout() -> String {
  String::from("post")
}

fn default_publish() -> String {
  String::from("rsync -az --del --quiet")
}

fn default_ssg() -> String {
  String::from("eleventy")
}

fn open_file(cmd: &str) {
  Exec::shell(cmd).join().unwrap();
}

fn setup() {

  fn prep_to_open_file() {
    let os = env::consts::OS;
    match os {
      "macos" => open_file("open ~/.letters.toml"),
      "linux" | "freebsd" | "openbsd" => open_file("xdg-open ~/.letters.toml"),
      &_ => () 
    }
  }

  fn create_file() {
    println!("Ok!");
    match fs::File::open("src/base-config.rs") {
      Ok(mut file) => {
        let mut content = String::new();
        // Read all the file content into a variable.
        file.read_to_string(&mut content).unwrap();
        // Write out to new config file
        fs::write("~/.letters.toml", content).unwrap();
        // open file
        prep_to_open_file()
      }
      Err(error) => {
        println!("Error opening file {}: {}", "src/base-config.rs", error);
      },
    }
  }

  let file = fs::OpenOptions::new()
              .write(true)
              .create_new(true)
              .open("~/.letters.toml");

  let _file = match file {
    Ok(_file) => create_file(),
    Err(_error) => prep_to_open_file()
  };
}

fn process(config: &Config)  -> subprocess::Result<bool> {

  let wd = shellexpand::full(&config.workdir)
    .expect("Error reading working directory")
    .to_string();

  // deal with possible empty config value
  // and set default command depending on ssg_type

  let cc = &config.commands.process;
  let ssg = &config.ssg_type.as_str();
  let commands;
  if cc == &String::from("") {
    commands = match ssg {
      &"hugo" => "hugo --quiet",
      _ => "eleventy --input=input --quiet"
    }
  } else {
    commands = cc.as_str();
  }

  let processed = Exec::shell(commands)
    .cwd(wd)
    .join()?;
  match processed {
    ExitStatus::Exited(code) => if code == 0 {
      Ok(true)
    } else {
      Ok(false)
    },
    _ => Ok(false)
  }
}

fn publish(config: &Config) -> subprocess::Result<bool> {
  let remote = shellexpand::full(&config.remote_dir).expect("Error reading remote directory").to_string();
  let output = [&shellexpand::full(&config.output).expect("Error reading output directory").to_string(), "/"].concat();
  let concatenated = [&config.commands.publish, " ", &output, " ", &config.server_name, ":", &remote].concat();
  let publishing = Exec::shell(&concatenated)
    .join()?;
  match publishing {
    ExitStatus::Exited(code) => if code == 0 {
      Ok(true)
    } else {
      Ok(false)
    },
    _ => Ok(false)
  }
}

fn quote(s: &str) -> String {
  let mut q = String::new();
  q.push_str("\"");
  q.push_str(&s.to_lowercase().trim());
  q.push_str("\"");
  q
}


fn test(config: &Config) -> subprocess::Result<()>{
  let wd = shellexpand::full(&config.workdir).expect("Error reading working directory").to_string();
  // string needs to be an Option<OsString> for Popen Config
  let os_string: Option<OsString> = Some(OsString::from(&wd));

  // deal with possible empty config values
  // and set defaults depending on ssg_type

  let ct = &config.commands.test;
  let test_url = &config.test_url;
  let ssg = &config.ssg_type.as_str();

  let commands;
  if ct == &String::from("") {
    commands = match ssg {
      &"hugo" => "hugo server -w --quiet",
      _ => "eleventy --input=input --quiet --serve"
    }
  } else {
    commands = ct.as_str();
  }

  let url;
  if test_url == &String::from("") {
    url = match ssg {
      &"hugo" => "http://localhost:1313",
      _ => "http://localhost:8080"
    }
  } else {
    url = test_url.as_str();
  }

  // use the original string, split on whitespace to create iterator
  let a = commands.split_whitespace();
  // collect into Vec
  let b: Vec<&str> = a.collect();
  let mut running_session = Popen::create(&b, PopenConfig { 
    cwd: os_string,
    detached: true,
    ..Default::default()
  })?;
  println!("Loading site locally in your browser, this may take a few seconds...");
  // wait on process to give it time to load
  let _waiting = running_session.wait_timeout(Duration::new(5,0));
  // open the browser to the local url
  Command::new("open")
    .arg(url)
    .output()
    .expect("Failed to open test_url");

  // Give user option to close local webserver process
  // We do this because it's running as a detached session
  // So it has to be terminated by the script rather than simply in the terminal
  color!(bold);
  color!(green);
  print!("Press Return/Enter to finish testing");
  color!(gray);
  close_color!(bold);
  io::stdout().flush().unwrap();
  let mut input = String::new();
  // when user hits Enter, terminate session
  io::stdin().read_line(&mut input).unwrap();
  running_session.terminate().unwrap();
  println!("goodbye");
  color!(reset);
  Ok(()) // return Ok to original function call
}

fn unsplash(config: &Config, topic: &str) -> (String, String) {

    // get image from unsplash
    let query = format!("https://api.unsplash.com/photos/random?query={}", topic);
    let auth = ["Client-ID ", config.unsplash_client_id.as_str()].concat();
    let resp = ureq::get(&query)
    .set("Authorization", &auth)
    .call();
    
    // response
    let json = resp.into_json().expect("Error reading Unsplash API response.");
    let mut _photo = String::from("");
    let mut _description = String::from("");

    if json["urls"]["small"].to_string() != "null" {
      // if there's a result use that
      _description = json["description"].to_string();
      _photo = json["urls"]["small"].to_string();
    } else {
      // else run unsplash query without topic
      let q = format!("https://api.unsplash.com/photos/random");
      let r = ureq::get(&q)
      .set("Authorization", &auth)
      .call();
      
      // response
      let j = r.into_json().expect("Error reading Unsplash API response.");
      _description = j["description"].to_string();
      _photo = j["urls"]["small"].to_string();
    }

    // return results as a tuple
    (_photo, _description)
}

fn write(config: &Config, no_image: bool) -> subprocess::Result<bool> {

    colol::init();
    // Title
    color!(bold);
    color!(green);
    print!("Title: ");
    color!(gray);
    close_color!(bold);
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    // Subtitle
    color!(bold);
    color!(green);
    print!("Subtitle: ");
    color!(gray);
    close_color!(bold);
    io::stdout().flush().unwrap();
    let mut subtitle = String::new();
    io::stdin().read_line(&mut subtitle).unwrap();

    // Summary
    color!(bold);
    color!(green);
    print!("Summary: ");
    color!(gray);
    close_color!(bold);
    io::stdout().flush().unwrap();
    let mut summary = String::new();
    io::stdin().read_line(&mut summary).unwrap();

    // tags
    color!(bold);
    color!(green);
    print!("Hashtags (comma separated): ");
    color!(gray);
    io::stdout().flush().unwrap();
    let mut given_tags = String::new();
    let mut vec: Vec<&str> = Vec::new();

    io::stdin().read_line(&mut given_tags).unwrap();
    color!(reset);

    if given_tags.trim().len() > 0 {
      vec = given_tags.split(",").collect(); // collect all tags if there are any
    }

    if &config.ssg_type == "eleventy" {
      vec.push(&config.default_layout);
    }

    let t = vec.iter().map(|t| quote(t)); // put quotation marks around each tag
    let tags = join(t, ","); // put a comma between each tag

    // Image search term
    fn topic(no_image: bool) -> String {
      if no_image {
        String::new() // this is not used
      } else {
        color!(bold);
        color!(green);
        print!("Image search term: ");
        color!(gray);
        close_color!(bold);
        io::stdout().flush().unwrap();
        let mut topic = String::new();
        io::stdin().read_line(&mut topic).unwrap();
        color!(reset);
        topic
      }
    }

    // unsplash search
    let unsplash = unsplash(config, &topic(no_image));

    // date
    let now = Utc::now();
    let date_string = now.to_rfc3339_opts(SecondsFormat::Secs, true);

    // write out file
    let mut contents = String::from("---\n");
    contents.push_str(&["layout: ", &config.default_layout].concat());
    contents.push_str("\ntitle: ");
    contents.push_str(&title);
    contents.push_str("subtitle: ");
    contents.push_str(&subtitle);
    contents.push_str("author: ");
    contents.push_str(&config.author);
    contents.push_str("\ntags: ");
    contents.push_str("[");
    contents.push_str(&tags);
    contents.push_str("]");
    contents.push_str("\nsummary: ");
    contents.push_str(&summary);
    contents.push_str("date: ");
    contents.push_str(&date_string);
    // this depends on ssg_type
    if !no_image {
      if &config.ssg_type == "hugo" {
        contents.push_str("\nimages: ");
        contents.push_str(&["\n  - ", &unsplash.0].concat());
      } else {
        contents.push_str("\nimage: ");
        contents.push_str("\n  photo: ");
        contents.push_str(&unsplash.0);
        contents.push_str("\n  description: ");
        contents.push_str(&unsplash.1);
      }
    }
    contents.push_str("\n---\n");

    // create filename
    let mut cloned = title.clone();
    cloned.retain(|c| c.is_alphanumeric() || c == ' ');
    let i = cloned.split_whitespace();
    let mut hyphenated = join(i, "-").to_lowercase();
    hyphenated.push_str(".md");
    let dir = config.input.as_str(); // blog input directory for markdown file
    let directory = shellexpand::full(dir).expect("Error reading input directory").to_string(); // expand to full path
    let filepath = Path::new(&directory).join(&hyphenated); // add filename to path
    fs::write(filepath, contents).expect("Error writing out file."); // write out file
    // open file
    let fp = Path::new(&directory).join(&hyphenated);
    let exit_status = Exec::cmd("open").arg(fp).join()?;
    match exit_status {
      ExitStatus::Exited(code) => if code == 0 {
        Ok(true)
      } else {
        Ok(false)
      },
      _ => Ok(false)
    }
  }

  fn get_social_post(config: &Config, msg: Option<&str>) -> String {
    // Get the last item from the RSS file
    // Normally this will be the post you just wrote
    let rss = shellexpand::full(&config.rss_file).expect("Error reading rss").to_string();
    // BUG: this panics if 'file' is incorrect or uses the default blank filepath
    let file = fs::File::open(rss).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    let last = &channel.items.last().unwrap();
    let link = &last.link().unwrap();
    let title = &last.title().unwrap();
    let mut post = String::new();
    // the text of the toot is the message if one was provided
    // otherwise we fall back to the title of the post
    let text = msg.unwrap_or(title);
    post.push_str(text);
    post.push_str("\n");
    post.push_str(link);
    // return the text of the post for use
    post
  }

  fn toot(config: &Config, msg: Option<&str>) -> Result<reqwest::blocking::Response, reqwest::Error> {

    let post = get_social_post(config, msg);

    // mastodon API access is pretty straightforward
    let mut token = String::from("Bearer ");
    token.push_str(&config.social.mastodon_access_token);
    let mut endpoint = String::from(&config.social.mastodon_base_url);
    endpoint.push_str("/api/v1/statuses");

    // Let's toot!
    let params = [("status", post)];
    let client = reqwest::blocking::Client::new();
    client.post(&endpoint)
    .form(&params)
    .header(reqwest::header::AUTHORIZATION, token)
    .send()
  }

  fn tweet(config: &Config, msg: Option<&str>) -> Result<reqwest::blocking::Response, reqwest::Error> {

    let post = get_social_post(config, msg);

    // prepare Twitter authorization info
    let consumer_key = &config.social.twitter_consumer_key;
    let consumer_secret = &config.social.twitter_consumer_secret;
    let access_token = &config.social.twitter_access_token;
    let token_secret = &config.social.twitter_access_secret;

    // We need a custom struct for oauth apparently
    #[derive(oauth::Request)]
    struct CreateTweet<'a> {
        status: &'a str,
    }
    // our actual message
    let content = CreateTweet {
      status: &post
    };

    // Twitter status posting endpoint
    let endpoint = "https://api.twitter.com/1.1/statuses/update.json";

    let token = oauth::Token::from_parts(consumer_key, consumer_secret, access_token, token_secret);
    // Create the `Authorization` header.
    let authorization_header = oauth::post(endpoint, &content, &token, oauth::HmacSha1);

    // Let's tweet!
    let params = [("status", &content.status)];
    let client = reqwest::blocking::Client::new();
    client.post(endpoint)
    .form(&params)
    .header(reqwest::header::AUTHORIZATION, authorization_header)
    .send()
  }

fn check_status(res: reqwest::blocking::Response, platform: String) {
  if res.status() == 200 {
    if platform == "twitter" {
      println!("🐦 tweeted!");
    } else {
      println!("📣 tooted!");
    }
    
  } else {
    println!("😭 {} returned error code {}", platform, res.status());
  }
}

fn publish_to_social(matches: ArgMatches, config: Config) {

  println!("Published! 🚀");

  if matches.is_present("toot") {
    let res = toot(&config, matches.value_of("message"));
    match res {
      Ok(res) => check_status(res, String::from("mastodon")),
      Err(err) => println!("😭 error tooting: {:#?}", err)
    }
  }

  if matches.is_present("tweet") {
    let res = tweet(&config, matches.value_of("message"));
    match res {
      Ok(res) => check_status(res, String::from("twitter")),
      Err(err) => println!("😭 error tweeting: {:#?}", err)
    }
  }

}

fn get_config() -> std::result::Result<Config, toml::de::Error>{
    // read config file and return result
  let fp = shellexpand::full("~/.letters.toml").expect("Error reading config file");
  let s = fs::read_to_string(&fp.into_owned()).expect("There is something wrong with your config file");
  let config: Config = toml::from_str(&s)?;
  Ok(config)
} 

fn main() {

  // read config file and provide helpful message if it is dodgy
  let conf = get_config();
  let config = match conf {
    Ok(c) => c,
    Err(e) => panic!("You have an error in your config file: {}", e)
  };

  let matches = App::new("lette.rs")
      .version("1.2.0")
      .author("Hugh Rundle")
      .about("A CLI tool to make static site publishing less painful")
      .arg(Arg::with_name("ACTION")
          .help("Action to perform")
          .required(true)
          .possible_values(&["setup", "process", "publish", "test", "write"])
          )
      .arg(Arg::with_name("no-image")
          .help("Don't get an image from Unsplash")
          .long("no-image")
          .required(false)
          .takes_value(false)
          )
      .arg(Arg::with_name("toot")
          .help("Send toot")
          .long("toot")
          .short("t")
          .required(false)
          .takes_value(false)
          )
      .arg(Arg::with_name("tweet")
          .help("Send tweet")
          .long("tweet")
          .short("w")
          .required(false)
          .takes_value(false)
          )
        .arg(Arg::with_name("message")
          .help("Message to toot/tweet")
          .long("message")
          .short("m")
          .required(false)
          .takes_value(true)
          )
      .get_matches();

  // if toot or tweet...
  if matches.is_present("toot") | matches.is_present("tweet") {
      if matches.value_of("ACTION").unwrap() == "publish" {
        match publish(&config) {
          // We do it like this so that the social post only gets published if the blog post is successfully published first
          Ok(x) => if x { publish_to_social(matches, config)} else {eprintln!("Uh oh, the 'publish' command failed!\nCheck your config file is correct.")},
          Err(err) => eprintln!("'publish' command failed!\nCheck your config file is correct.\nError: {}", err)
        }
      } else {
        println!("--toot and --tweet can only be used with publish")
      }
  } else {
    let action = matches.value_of("ACTION").unwrap();
    match action {
      "setup" => setup(),
      "process" => match process(&config) {
        Ok(x) => if x {()} else {eprintln!("Uh oh, the 'process' command failed!\nCheck your config file is correct.")},
        Err(err) => eprintln!("'process' command failed!\nCheck your config file is correct.\nError: {}", err)
      },
      "publish" => match publish(&config) {
        Ok(x) => if x {println!("Published! 🚀")} else {eprintln!("Uh oh, the 'publish' command failed!\nCheck your config file is correct.")},
        Err(err) => eprintln!("'publish' command failed!\nCheck your config file is correct.\nError: {}", err)
      },
      "test" => match test(&config) {
        Ok(_v) => (),
        Err(err) => eprintln!("'test' command failed!\nCheck your config file is correct.\nError: {}", err)
      },
      "write" => match write(&config, matches.is_present("no-image")) {
        Ok(x) => if x {()} else {eprintln!("Uh oh, the 'write' command failed!\nCheck your config file is correct.")},
        Err(err) => eprintln!("'write' command failed!\nCheck your config file is correct.\nError: {}", err)
      }
      ,
      &_ => () // this won't actually run but is needed by match
    }
  }
}