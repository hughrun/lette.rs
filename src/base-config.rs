author = "" # your name as you want it to appear on your posts
input = "" # the input directory for your site i.e. where your markdown files go
output = "" # the directory for processed files for your site i.e. where your html files go. Do NOT include a trailing slash as lette.rs will add this for you
workdir = "" # the base directory for calling your static site commands. Probably the root directory for eleventy, Hugo etc
remote_dir = "/var/www/blog" # the directory to rsync files to, on your remote server.
unsplash_client_id = "" # unsplash client ID string
server_name = "blogserver" # this could also be an IP address
test_url = "http://localhost:8080" # if your SSG serves your site locally this should be the URL where you can see it

[commands]
# commands listed below are examples only, though if you use eleventy you can probably leave them 😆
process = "eleventy --input=input --quiet" # command to process files
publish = "rsync -az --del --quiet" # don't change this unless you know what you're doing
test = "eleventy --input=input --quiet --serve" # command to serve site locally (if your SSG enables that)