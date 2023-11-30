# Pollinate

### Overview

Pollinate is a CLI program for automatically creating JSON files populated by user defined schemas. The purpose of this is to use in automation pipeline testing especially when PII information is normally included with the files you are working with. The functions in the accompanying crate were also made public in case you would like to connect the crate directly into your rust code.

### Usage

```
pollinate [OPTIONS] --input-schema <INPUT_SCHEMA> --output<OUTPUT>
- Options:
- -i, --input-schema <INPUT_SCHEMA>    Path to input schema
- -o, --output <OUTPUT>  Path to output schema
- -c, --count <COUNT>                  Number of objects to create [default: 1]
```

Pollinate takes the path to an input schema that is loosely based on the JSON Schema spec. The top level of the schema have a "type" key set to "object". It must also have a "properties" key which is set to an object containing the schema. See details for each data type below. Pollinate also takes a path to the output file, and a count determining the number of JSON objects to store in the JSON array.

### Schema requirements

##### string

- "enum" key set to an array of strings

##### integer:

One of:

- "enum" key set to an array of strings
- "minimum" and "maximum" keys denoting the inclusive range of i64 values

##### object

- same as top level, a JSON object consisting of the types defined here

##### array

- "minimum" and "maximum" keys denoting the inclusive range for the u32 size of the array
- "items" key, a JSON object contianing one of the types defined here

```
{
  "$id": "https://example.com/person.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Person",
  "type": "object",
  "properties": {
    "firstName": {
      "type": "string",
      "description": "The person's first name.",
      "enum": ["Adam", "John", "Jacob"]
    },
    "age": {
      "type": "integer",
      "description": "Age is but an int",
      "minimum": 0,
      "maximum": 100
    },
    "skills": {
      "type": "array",
      "minimum": 0,
      "maximum": 5,
      "items": {
        "type": "string",
        "enum": ["Java", "Sales", "Talking", "Drawing", "Eating"]
      }
    },
    "partner": {
      "type": "object",
      "properties": {
        "firstName": {
          "type": "string",
          "description": "The person's first name.",
          "enum": ["Adam", "John", "Jacob"]
        },
        "age": {
          "type": "integer",
          "description": "Age is but an int",
          "minimum": 0,
          "maximum": 100
        },
        "skills": {
          "type": "array",
          "minimum": 0,
          "maximum": 5,
          "items": {
            "type": "string",
            "enum": ["Java", "Sales", "Talking", "Drawing", "Eating"]
          }
        }
      }
    }
  }
}

```

### Sources

https://stackoverflow.com/questions/43704758/how-to-idiomatically-convert-between-u32-and-usize
https://users.rust-lang.org/t/how-to-add-a-trait-value-into-hashmap/6542/3
Dynamic cloning: https://stackoverflow.com/questions/69890183/how-can-i-clone-a-vecboxdyn-trait

CHAT GPT for assistance creating docs/tests.
