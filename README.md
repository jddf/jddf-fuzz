# jddf-fuzz

`jddf-fuzz` generates random JSON documents that satisfy a JDDF schema. It's
useful for doing quick-and-dirty testing or for generating somewhat realistic
data.

## Usage

See `jddf-fuzz --help` for detailed instructions, but essentially you run it
like this:

```text
$ echo '{ "type": "timestamp" }' | jddf-fuzz -n 5
"1950-09-01T00:06:56+00:00"
"1983-04-06T13:12:43+00:00"
"1904-12-21T19:35:20+00:00"
"1937-02-13T07:59:45+00:00"
"1971-10-19T17:26:48+00:00"
```

By default, `jddf-fuzz` reads a JDDF schema from STDIN, and then produces an
infinite number of JSON lines satisfying that schema to STDOUT. To use a file
instead of STDIN, you can call `jddf-fuzz` as:

```text
jddf-fuzz schema.json
```

## Demo

Here's a schema that uses all jddf features:

```json
{
  "discriminator": {
    "tag": "type",
    "mapping": {
      "primitives": {
        "properties": {
          "any": {},
          "bool": { "type": "boolean" },
          "num": { "type": "number" },
          "str": { "type": "string" },
          "ts": { "type": "timestamp" }
        },
        "optionalProperties": {
          "opt_bool": { "type": "boolean" }
        }
      },
      "array": {
        "properties": {
          "arr": {
            "elements": { "type": "boolean" }
          }
        }
      },
      "map": {
        "properties": {
          "map": {
            "values": { "type": "boolean" }
          }
        }
      }
    }
  }
}
```

Here are some sample values generated from that schema by `jddf-fuzz`:

```json
{"arr":[true,false,true,false],"type":"array"}
{"any":"BU%ZgN","bool":false,"num":0.6692874298834272,"str":"5gMB7c","ts":"1988-12-05T19:30:08+00:00","type":"primitives"}
{"arr":[true,true,true,true],"type":"array"}
{"map":{},"type":"map"}
{"any":false,"bool":true,"num":0.6408832653608904,"opt_bool":false,"str":"KZb9","ts":"1958-06-04T21:54:30+00:00","type":"primitives"}
{"any":"rQ\\f0","bool":true,"num":0.9750290489672578,"str":" 3`#}","ts":"1904-01-07T23:44:50+00:00","type":"primitives"}
{"map":{"@50FA":true,"QpYtf}":true,"w\"hF":true},"type":"map"}
{"map":{},"type":"map"}
{"any":null,"bool":false,"num":0.9484149665040986,"str":"2czfNnd","ts":"1947-12-19T08:32:00+00:00","type":"primitives"}
{"any":null,"bool":true,"num":0.9497698629380723,"str":"OEV","ts":"1983-10-22T07:48:54+00:00","type":"primitives"}
{"map":{"/g":false,"PG}Ax":true,"^'X:Q":true,"k^@A":true,"wMk":true,"~V":false},"type":"map"}
{"arr":[false,true,true,false,false],"type":"array"}
{"map":{"":false,":_)f3&>":true,"Em16\"":true,"WP$Ad":true,"_B6yL}":false,"p":false,"x;H":true},"type":"map"}
{"map":{"'C&m2":true,"4wZ)t":true,"P`":false,"a;&b]n":false},"type":"map"}
{"map":{"9'EA6":false},"type":"map"}
{"map":{"Ss7aK":false},"type":"map"}
{"arr":[true,true,true,true,false,true],"type":"array"}
{"arr":[false],"type":"array"}
{"any":null,"bool":false,"num":0.26690805500121584,"opt_bool":false,"str":".","ts":"1980-08-17T04:26:32+00:00","type":"primitives"}
{"map":{"":true},"type":"map"}
```
