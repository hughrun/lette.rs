# lette.rs

A CLI tool to make static site publishing less painful.

Built for MacOS, probably works on n*x. The idea of `lette.rs` is to enable you to set-and-forget your static site setup and commands, and focus on writing great blog posts.

## Installation

### From source with cargo

```shell
cargo build --release
```
### From executable

TBC

## Use

`lette.rs` is a command line program with only five commands:

* process
* publish
* setup
* test
* write

Run with `letters COMMAND`.

### setup

This command opens your config file for viewing or editing. The config file is always saved at `~/letters.toml` - if this file does not exist a default file will be created. If it does exist, the existing file is opened.

This is what drives `lette.rs`. Basically you put all your directory references and static site generator commands in the config file, and then you never have to remember them again.

You will need an [API key from Unsplash](https://unsplash.com/documentation#creating-a-developer-account).

### write

You will be asked for some basic information, then `lette.rs` will fetch an image from Unsplash and open a new markdown file with all your frontmatter set up for you.

### process

Once you've finished writing your masterpiece, you need to process the markdown files into html. Who can be bothered remembering the arcane command your SSG requires? Just type `letters process`!

### test

It's always good to do a final check before publishing. Your SSG probably allows you to run your site locally. `letters test` remembers what to do, and will open your site in a browser. When you're done just head back to the command line and hit `Enter`.

### publish

Hello world! Publishing from your local machine to a remote server is a gigantic PITA. With `lette.rs` you never have to remember how to `rsync` or whatever. Just `letters publish` and move on with your life.