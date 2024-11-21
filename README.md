# URL Shortener

A simple URL shortener service built with Rust, Rocket, and PostgreSQL.

### Usage

#### Shorten a URL

# With default generated alias

```
curl -X POST -d "url=https://example.com" https://shorten-dat-url.shuttleapp.rs
```

# With custom alias

```
curl -X POST \
  -d "url=https://example.com" \
  -d "shorten_to=example" \
  https://shorten-dat-url.shuttleapp.rs
```
#### Access shortened URL

Visit the shortened URL to be redirected to the original link:

```
https://shorten-dat-url.shuttleapp.rs/{alias}
```


## Built With

- [Rocket](https://rocket.rs/) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [nanoid](https://github.com/mrdimidium/nanoid) - ID generation
- [Shuttle](https://www.shuttle.dev/) - Deployment platform

## License

[MIT License](https://spdx.org/licenses/MIT.html)