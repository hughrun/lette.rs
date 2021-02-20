# lette.rs

A CLI tool to make static site publishing less painful.

Built for MacOS, probably works on n*x. The idea of `lette.rs` is to enable you to set-and-forget your static site setup and commands, and focus on writing great blog posts.

This is basically the cleaner rustified version of [writenow](https://github.com/hughrun/writenow).

## Installation

### From source with cargo

```shell
cargo build --release
```
### From executable

1. download `letters` executable from [the latest release](https://github.com/hughrun/lette.rs/releases/latest)
2. symlink to somewhere on your `PATH`: `sudo ln -s /FULL/PATH/TO/letters /usr/local/bin`

## Dependencies

`lette.rs` assumes you have `rsync` on your machine.

You will also need an [API key from Unsplash](https://unsplash.com/documentation#creating-a-developer-account).

## Use

`lette.rs` is a command line program with only five commands:

* process
* publish
* setup
* test
* write

Run with `letters COMMAND`.

### setup

This command opens your config file for viewing or editing. The config file is always saved at `~/.letters.toml` - if this file does not exist a default file will be created. If it does exist, the existing file is opened.

This is what drives `lette.rs`. Basically you put all your directory references and static site generator commands in the config file, and then you never have to remember them again.

### write

You will be asked for some basic information, then `lette.rs` will fetch an image from Unsplash and open a new markdown file with all your frontmatter set up for you.

### process

Once you've finished writing your masterpiece, you need to process the markdown files into html. Who can be bothered remembering the arcane command your SSG requires? Just type `letters process`!

### test

It's always good to do a final check before publishing. Your SSG probably allows you to run your site locally. `letters test` remembers what to do, and will open your site in a browser. When you're done just head back to the command line and hit `Enter`.

### publish

Hello world! Publishing from your local machine to a remote server is a gigantic PITA. With `lette.rs` you never have to remember how to `rsync` or whatever. Just `letters publish` and move on with your life.

## License

`lette.rs` is licensed under GPL 3.0

Please note most important part of this license is Clause 15 which points out this code is not guaranteed to work at all nor non-catastrophically. I don't know what I'm doing. 😀