[Personal blog](https://roadmap.sh/projects/personal-blog) project for roadmap.sh  written in Rust.

# Prerequisites
- [Rust](https://www.rust-lang.org/tools/install).
- `openssl` (or any other tool that can generate 256-bit base64 keys).
  
# Installation
1. Clone this github repository `git clone https://github.com/dmxmss/dmx.blog.git`
2. generate rocket's 256-bit base64 secret key, e.g. with `openssl rand -base64 32`
3. Paste this value in Rocket.toml file (create it):
```
[default]
secret_key = <secret key>
```
4. Create App.toml file and paste there admin password and server secret:
```
[global]
admin_password = <pass>
server_secret = <secret>
```
5. Run server with `cargo run`. Default address is `127.0.0.1:8000`.

# Server routes
- `/`: home guest route
- `/article/<id>`: article route
- `/login`: login page
- `/refresh`: route needed for refreshing tokens
- `/admin`: admin dashboard
- `/admin/new`: create new article
- `/admin/edit/<id>`: edit article
- `/admin/delete/<id>`: delete article by id
