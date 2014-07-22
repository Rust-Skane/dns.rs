# dns.rs #

dns.rs is a library/server that provides DNS services, at least in theory. It isn't ready for production or testing, but it is ready to have fun \o/.

## Build the server ##

I build with `rustc server.rs -L ../rust-http/target && sudo ./server` and that seems to work fine. (You need rust-http to talk with couch)

To fill it with data you also need a local couchdb with a database called `dns` filled with the documents couchview.json and example.json.

Then you can use a tool like askmara `askmara Zgoogle.se.`, or if you don't use maradns, `dig @127.0.0.1 google.se`, or `nslookup google.se 127.0.0.1`.

## Play ##

Please do, bug reports are welcome! Also welcome are patches, ideas, random prods on IRC (jensnockert on mozilla IRC), etc.