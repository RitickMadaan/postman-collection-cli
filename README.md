<h2>
  A CLI to play with postman collections inspired by <a href="https://www.passwordstore.org" target="_blank">password-store</a>
</h2>

## Key Features
* ### Convert to curl
  `pocc` supports converting a postman request to a curl for easier use in the terminal.

  ### How to use

  cmd:
  ```bash
    pocc -c "<CollectionName>/<Folder>/*/<RequestName>"
  ```
  output: 
  ```bash
    curl --location --globoff --request POST 'https://example.com' \
    --header 'Authorization: Bearer 12345678' \
    --header 'Content-Type: text/plain' \
    --data 'hey there'
  ```
## Upcoming features
* auto completion
* insert in a collection from curl
    
and more, take a look at [issues](https://github.com/RitickMadaan/postman-collection-cli/issues) to find out

## Installation

### From cargo

to install through cargo run:

```bash
cargo install pocc
```
NOTE: below installation methods coming in soon
* brew
* nix
* apt

## Project goals

`pocc`'s fundamental objective is to empower users to seamlessly interact with Postman collections, all within the convenience of their terminal.
