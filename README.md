### WIP

# Da heck is dis?
Small thing to make HTTP requests from a JSON file and store it as a file tree.

### Example config:
```config.json```
```json
{
  "dst": "path/to/destination/root",
  "requests": {
    "headers": {},
    "endpoints": [
      {
        "url": "https://dog.ceo/api/breeds/image/random",
        "headers": {}
      },
      {
        "url": "https://dog.ceo/api/breed/Affenpinscher/images/random"
      },
      {
        "url": "https://dog.ceo/api/breed/African/images/random"
      }
    ]
  }
}
```

### Will result in:
![Example structure](sample/sample.png)

### Usage:
`./data_fetcher path/to/config.json`

### Config overview:
- `dst`[required] - Destination root where responses tree will start from
- `requests`[required] - An object that contain info about the requests
  - `headers`[optional] - optional map of headers that will be attached to each request
  - `endpoints`[required] - array of actual request objects
    - Example of endpoint object:
    - `url`[required] - url of http request
    - `headers`[optional] - headers of particular request (will be merged with parent headers if present)