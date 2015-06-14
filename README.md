# How to use
First, start the search engine:
```
$ cargo run --release
```
It defaults to localhost on port 3000.

Next, insert data:
```
$ curl -XPOST 'localhost:3000/index' -d '{
  "id": "test",
  "content": "How to run this search engine"
}'
```

Then, search in one of two ways:

1. `$ curl -XGET 'localhost:3000/search?q=how'`
2. Go to localhost:3000 and use the search bar
