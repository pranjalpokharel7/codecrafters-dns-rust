# Codecrafters DNS Server From Scratch

## DNS Message Format

```
+---------------------+
|        Header       | - 12 bytes (always)
+---------------------+
|       Question      | the question for the name server (response should match client's question section)
+---------------------+
|        Answer       | Resource Records (RRs) answering the question - so there is no particular answer section format - it's just a collection of resource records
+---------------------+
|      Authority      | RRs pointing toward an authority
+---------------------+
|      Additional     | RRs holding additional information
+---------------------+
```


## Compression

- Use pointers to refer to repetitive label sequences in the DNS message (across question and answer sections)
- Un-compressed labels start with two 0 bits, but compressed labels start with two 1 bits