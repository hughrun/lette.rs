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
2. Add to PATH, either:
   1. save the executable file to somewhere like `/usr/local/bin` or `/usr/bin`; or
   2. save the executable file somewhere else and symlink to somewhere on your `PATH` e.g. `sudo ln -s /FULL/PATH/TO/letters /usr/local/bin`

## Dependencies

`lette.rs` assumes you have `rsync` on your machine.

Unless you use `--no-image` will also need an [API key from Unsplash](https://unsplash.com/documentation#creating-a-developer-account).

If you use `--toot` you need [a Mastodon access token](https://shkspr.mobi/blog/2018/08/easy-guide-to-building-mastodon-bots/).

If you use `--tweet` you need [Twitter OAuth 1.1 credentials](https://developer.twitter.com/en/docs/authentication/oauth-1-0a).

## Configuration

TODO:

## Use

`lette.rs` is a command line program.

Run with `letters COMMAND [--option]`.

### commands

#### setup

This command opens your config file for viewing or editing. The config file is always saved at `~/.letters.toml` - if this file does not exist a default file will be created. If it does exist, the existing file is opened.

This is what drives `lette.rs`. Basically you put all your directory references and static site generator commands in the config file, and then you never have to remember them again.

#### write

You will be asked for some basic information, then `lette.rs` will fetch an image from Unsplash and open a new markdown file with all your frontmatter set up for you.

#### process

Once you've finished writing your masterpiece, you need to process the markdown files into html. Who can be bothered remembering the arcane command your SSG requires? Just type `letters process`!

#### test

It's always good to do a final check before publishing. Your SSG probably allows you to run your site locally. `letters test` remembers what to do, and will open your site in a browser. When you're done just head back to the command line and hit `Enter`.

#### publish

Hello world! Publishing from your local machine to a remote server is a gigantic PITA. With `lette.rs` you never have to remember how to `rsync` or whatever. Just `letters publish` and move on with your life.

### options

#### --no-image

Used with `write`, this bypasses the creation of image frontmatter. Use if you don't want images or don't want to use Unsplash.

#### --toot

Used with `publish`, this will send a toot from your [Mastodon](https://joinmastodon.org) account, with a link to your most recent post (i.e. the one you just published).

If text is provided with `--message` that will be the message text, otherwise the title of the post is used.

e.g. if your latest post is called "Rust 101" and the URL is "https://myblog.rocks/rust-101":

`letters publish --toot` will toot:

```
Rust 101
https://myblog.rocks/rust-101
```

`letters publish --toot --message 'Learning Rust is hard :rustacean:` will toot:

```
Learning Rust is hard :rustacean:
https://myblog.rocks/rust-101
```

Requires these values in your settings:
+ `mastodon_access_token`
+ `mastodon_base_url`
+ `rss_file`

#### --tweet

Used with `publish`, this will send a toot from your [Twitter](https://twitter.com) account, with a link to your most recent post (i.e. the one you just published).

If text is provided with `--message` that will be the message text, otherwise the title of the post is used.

e.g. if your latest post is called "101 ways with broccoli" and the URL is "https://myblog.rocks/101-ways-with=broccoli":

`letters publish --tweet` will tweet:

```
101 ways with broccoli
https://myblog.rocks/101-ways-with=broccoli
```

`letters publish --tweet --message 'I love broccoli` will tweet:

```
I love broccoli
https://myblog.rocks/101-ways-with=broccoli
```

Requires these values in your settings:
+ `twitter_consumer_key`
+ `twitter_consumer_secret`
+ `twitter_access_token`
+ `twitter_access_secret`
+ `rss_file`

#### --message

Use with `--toot` or `--tweet` as described above.

You can also use the short form of these commands and combine them:

```
letters publish -tw -m 'Check out my latest blog post'
```

## License

`lette.rs` is licensed under GPL 3.0

Please note most important part of this license is Clause 15 which points out this code is not guaranteed to work at all nor non-catastrophically. I don't know what I'm doing. 😀