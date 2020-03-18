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
