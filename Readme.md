<div align="center">
<img src="assets/512x512.png" alt="logo" width="150" style="border-radius: 25px"/>

[![Discord](https://img.shields.io/discord/1261282563138392117?color=5865F2&label=Discord&logo=discord&logoColor=white)](https://discord.gg/FSK25BXgdt)

</div>

# Sanchaar - A Offline REST API client

Sanchaar is a offline REST API client built using Iced in Rust. It is a simple tool to test REST APIs without the need of internet connection. It supports GET, POST, PUT, DELETE requests with path, query and header parameters.

## Screenshot

![Screenshot](./screenshots/app.png)

## Features

- Send GET, POST, PUT, DELETE requests
- Path, Query, Header params
- Multiple requests in tabs
- Save/Load requests from collections
- Create/Open collection from local disk
- Environment variables

## Roadmap

- [x] Path param support
- [x] Query param support
- [x] Header param support
- [x] Body support
  - [x] JSON
  - [x] Form
  - [x] XML
  - [x] Text
  - [x] Raw File
  - [x] Multipart (Files not supported with GET method)
- [x] Request cancellation
- [ ] Authentication
  - [x] Basic
  - [x] Bearer
  - [ ] OAuth
  - [ ] OAuth2
  - [ ] AWS
  - [ ] Digest Auth
- [x] Tab view for multiple requests
- [x] File persistence
  - [x] HCL file format
  - [x] Save/Load
  - [x] Changed indicator
  - [x] File Rename
- [ ] Collections/Folder
  - [x] Tree view
  - [x] Create/Open
  - [x] Auto Save
  - [ ] Refresh tree manually
  - [ ] Refresh tree automatically
  - [x] Remove
  - [x] Rename collection/folder
  - [ ] Export/Import
  - [ ] Settings
    - [ ] Update default env
    - [ ] Collection headers
    - [ ] Collection auth
    - [ ] Collection Headers
    - [ ] Collection Variables
    - [ ] Request template selection
- [ ] Environments
  - [x] Add/Remove/Update
  - [x] Choose environment
  - [x] Auto Save/Load current state
  - [ ] Secure environment variables
  - [ ] Variables from .env file
- [ ] Assertions
  - [x] Status code
  - [x] Response time
  - [x] Response body
  - [x] Response headers
  - [ ] GUI editor
  - [ ] GUI assertions
- [ ] Scripting
  - [ ] Pre request
  - [ ] Post request
- [ ] Settings
  - [x] Theme
  - [ ] Proxy
  - [ ] SSL
  - [ ] Timeout
- [ ] Cookies
  - [ ] List/Remove
  - [ ] Edit/Add ?
- [ ] History
  - [ ] List
  - [ ] Clear
  - [ ] Open from history
  - [ ] Auto Save/Load
- [ ] Mock APIs
- [ ] CLI
  - [x] Run request by path
  - [x] Run assertion by path/folder
  - [ ] Pretty print assertion results
  - [ ] Select environment by name
  - [ ] Run tests by path
  - [ ] Run all collection tests
- [ ] Code export
