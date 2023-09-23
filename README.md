<h2>
  A CLI to play with postman collections inspired by <a href="https://www.passwordstore.org" target="_blank">password-store</a>
</h2>

## Key Features
* ### Convert to curl
  `pocc` supports converting a postman request to a curl for easier use in the terminal.

  ### How to use

  cmd:
  ```bash
    pocc curl
  ```
  select the request:
  ```bash
  Select request from current directory:
  > TestCollection/New Folder/example.com
    TestCollection/New Folder/raw_json_body
    TestCollection/New Folder/raw_javascript_body
    TestCollection/New Folder/example.com
    TestCollection/New Folder/form-data
    TestCollection/New Folder/x-www-form-urlencoded
    TestCollection/New Folder/raw_text_body
    [↑↓ to move, enter to select, type to filter]
  ```

  output:
  ```bash
    curl --location --globoff --request POST 'https://example.com' \
    --header 'Authorization: Bearer 12345678' \
    --header 'Content-Type: text/plain' \
    --data 'hey there'
  ```
  [demo.webm](https://github.com/RitickMadaan/postman-collection-cli/assets/43561186/52116e6b-e53f-4d53-9154-5ef4f1a74e16)

* ### Convert to Curl and Copy to Clipboard

  cmd:
  ```bash
    pocc -c curl
  ```
  select the request from the same interactive UI as above and it's curl will be copied to clipbaord. 
  
## Upcoming features
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
