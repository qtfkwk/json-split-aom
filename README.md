# Usage

```text
$ json-split-aom -h
Split a large JSON file containing an array of maps into multiple files

Usage: json-split-aom [OPTIONS] -a <JSON_PATH> -i <JSON_PATH> [FILES]...

Arguments:
  [FILES]...  Input file(s)

Options:
  -a <JSON_PATH>      Dotted JSON path to an array in the input file
  -i <JSON_PATH>      Dotted JSON path to the ID in the array element
  -p                  Pretty print output files
  -c                  Allow ID path collisions; still gives warnings but
                      duplicates will overwrite previous files
  -h, --help          Print help
  -V, --version       Print version

---

# Assumptions

* The JSON dotted path keys for array and ID don't have periods.

# Examples
    
1. Use `json-split-aom -a 'Apple.Banana' -i 'id' file.json` to extract objects
   to files named `Apple.Banana-id-ID.json` given a `file.json` with content:
   `{"Apple":{"Banana":[{"id":12,...},...]}}`

2. Use `json-split-aom -a 'Apple' -i 'Banana.id' file.json` to extract objects
   to files named `Apple-Banana.id-ID.json` given a `file.json` with content:
   `{"Apple":[{"Banana":{"id":12,...}},...]}`

3. Use `json-split-aom -a 'Apple.Banana' -i 'Cherry.id' file.json` to extract
   objects to files named `Apple.Banana-Cherry.id-ID.json` given a `file.json`
   with content: `{"Apple":{"Banana":[{"Cherry":{"id":12,...},...}]}}`
```

```text
$ json-split-aom -V
json-split-aom 0.1.2
```

