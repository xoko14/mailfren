# mailfren

send a fren in your mail and let the recipients greet them.

(the fren may or may not come back with some info about them)

![](https://i.kym-cdn.com/entries/icons/original/000/029/323/doggo.jpg)

Mailfren creates links to a 1x1px transparent png. Information about any request made to these links is saved.

## Requirements
- Rust toolchain
- Depending on OS openssl/sqlite3 libraries may also be needed for compiling.

## Setup
Init/migrate database:
```
cargo run --bin migrate
```

Run:
```
cargo run
```

## API

### Create fren
A fren is a representation of a link.
```http
POST /v1/frens
Content-Type: application/json

{
    "code": "test_fren",
    "description": "a fren ready to recieve greetings"
}
```
`200 OK`
```json
{
    "id": 33,
    "code": "test_fren",
    "description": "a fren ready to recieve greetings",
    "url": "https://example.com/v1/frens/33/image.png"
}
```

### Get fren
```http
GET /v1/frens/33
```
`200 OK`
```json
{
    "id": 33,
    "code": "test_fren",
    "description": "this is a test",
    "url": "https://example.com/v1/frens/33/image.png"
}
```

### Get fren greeting
A greeting is a visit to the fren's image url.

```http
GET /v1/frens/33/greetings
```
`200 OK`
```json
[
    {
        "id": 1,
        "date": "2024-03-01T09:13:49",
        "ip": "127.0.0.1:49068",
        "headers": "{\"connection\":\"close\",\"accept-encoding\":\"gzip, deflate, br\",\"x-real-ip\":\"[redacted]\",\"x-forwarded-server\":\"example.com\",\"user-agent\":\"Mozilla/5.0 (Windows NT 5.1; rv:11.0) Gecko Firefox/11.0 (via ggpht.com GoogleImageProxy)\",\"x-forwarded-for\":\"[redacted]\",\"host\":\"example.com\",\"x-forwarded-host\":\"example.com\",\"x-forwarded-proto\":\"https\"}"
    }
]
```
Redacted example. Mailfren was accesible through a reverse proxy. The request was made through Gmail.
