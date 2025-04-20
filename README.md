# humjson

JSON formatter that produces concise output 

```sh
cargo install --git https://github.com/PolyMeilex/humjson.git

cat ./file.json | humjson
```

```json
{
  "obj": {
    "a": 108,
    "b": "Abc",
    "c": 1745016558630455965,
    "d": { "e": 0, "f": "Cba" }
  },
  "g": [49, 50, 55, 46, 48, 46, 48, 46, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "h": 10001,
  "i": [49, 50, 55, 46, 48, 46, 48, 46, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "j": 10003
}
```
