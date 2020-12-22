use clap::{Arg, App};
use colol::{color, close_color};
use subprocess::{Exec, Popen, PopenConfig};
use itertools::join;
use std::ffi::OsString;
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use serde_derive::Deserialize;

// deserialize TOML file
#[derive(Deserialize)]
struct Config {
    input: String,
    output: String,
    workdir: String,
    remote_dir: String,
    unsplash_client_id: String,
    server_name: String,
    test_url: String,
    commands: Commands
}

#[derive(Deserialize)]
struct Commands {
    process: String,
    publish: String,
    test: String
}

fn config() {
  // TODO
}

fn process(config: &Config) {
  // TODO
}

fn publish(config: &Config) {
  let remote = shellexpand::full(&config.remote_dir).expect("Error reading remote directory").to_string();
  let output = [&shellexpand::full(&config.output).expect("Error reading output directory").to_string(), "/"].concat();
  let concatenated = [&config.commands.publish, " ", &output, " ", &config.server_name, ":", &remote].concat();
  Exec::shell(&concatenated).join().expect("Something went wrong trying to publish");
  println!("Published! 🚀")
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
  // use the original string, split on whitespace to create iterator
  let a = config.commands.test.split_whitespace();
  // collect into Vec
  let b: Vec<&str> = a.collect();
  let mut running_session = Popen::create(&b, PopenConfig { 
    cwd: os_string,
    detached: true,
    ..Default::default()
  })?;
  println!("Loading site locally...");
  // wait on process to give it time to load
  let _waiting = running_session.wait_timeout(Duration::new(5,0));
  // open the browser to the local url
  Command::new("open")
    .arg(&config.test_url)
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

fn write(config: &Config) {

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
    vec.push("post"); // always add "post" tag
    let t = vec.iter().map(|t| quote(t)); // put quotation marks around each tag
    let tags = join(t, ","); // put a comma between each tag

    // Image search term
    color!(bold);
    color!(green);
    print!("Image search term: ");
    color!(gray);
    close_color!(bold);
    io::stdout().flush().unwrap();
    let mut topic = String::new();
    io::stdin().read_line(&mut topic).unwrap();
    color!(reset);

    // unsplash search
    let unsplash = unsplash(config, &topic);

    // write out file
    let mut contents = String::from("---\n");
    contents.push_str("layout: post"); // default to post layout
    contents.push_str("\ntitle: ");
    contents.push_str(&title);
    contents.push_str("subtitle: ");
    contents.push_str(&subtitle);
    contents.push_str("author: Hugh Rundle");
    contents.push_str("\ntags: ");
    contents.push_str("[");
    contents.push_str(&tags);
    contents.push_str("]");
    contents.push_str("\nsummary: ");
    contents.push_str(&summary);
    contents.push_str("image: ");
    contents.push_str("\n  photo: ");
    contents.push_str(&unsplash.0);
    contents.push_str("\n  description: ");
    contents.push_str(&unsplash.1);
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
    Exec::cmd("open").arg(fp).join().expect("Error opening file");

  }

fn main() {
  // read config file
  let fp = shellexpand::full("~/.letters.toml").expect("Error reading config file");
  let s = fs::read_to_string(&fp.into_owned()).unwrap();
  let config: Config = toml::from_str(&s).unwrap();

  let matches = App::new("lette.rs")
      .version("1.0.0")
      .author("Hugh Rundle")
      .about("A CLI tool to make static site publishing less painful")
      .arg(Arg::with_name("ACTION")
          .help("Action to perform")
          .required(true)
          .possible_values(&["config", "process", "publish", "test", "write"])
          )
      .get_matches();

  let action = matches.value_of("ACTION").unwrap();
  match action {
    "config" => println!("The action is CONFIG!"),
    "process" => println!("The action is PROCESS!"),
    "publish" => publish(&config),
    "test" => test(&config).unwrap(),
    "write" => write(&config),
    &_ => () // this won't actually run but is needed by match
  }
}