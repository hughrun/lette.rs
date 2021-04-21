author = "" # your name as you want it to appear on your posts
input = "" # the input directory for your site i.e. where your markdown files go
output = "" # the directory for processed files for your site i.e. where your html files go. Do NOT include a trailing slash as lette.rs will add this for you
workdir = "" # the base directory for calling your static site commands. Probably the root directory for eleventy, Hugo etc
remote_dir = "/var/www/blog" # the directory to rsync files to, on your remote server.
rss_file = "~/blog/_site/rss/rss.xml" # filepath to the RSS file in your output directory
server_name = "blogserver" # this could also be an IP address
# unsplash_client_id = "" # unsplash client ID string (if you don't want this, leave it and use --no-image)
# test_url = "" # if your SSG serves your site locally this should be the URL where you can see it. eleventy and hugo will use their respective defaults.
# ssg_type = "" # your static site generator. Options that will do something are "hugo" or "eleventy" but you can try something else and see if it works
# default_layout = "" # use any string, this will be the value of "layout" in your frontmatter

[commands]
# You can override the defaults by setting one of the values below
# process = "" # command to process files
# publish = "" # don't change this unless you know what you're doing
# test = "" # command to serve site locally (if your SSG enables that)

[social]
# uncomment and set values below as needed
# mastodon_access_token = "" 
# mastodon_base_url = "https://example.com"
# twitter_consumer_key = ""
# twitter_consumer_secret = ""
# twitter_access_token = ""
# twitter_access_secret = ""