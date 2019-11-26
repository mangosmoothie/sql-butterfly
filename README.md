# SQL Butterfly Style Query Formatter

Format sql queries using the soothing "butterfly" format.

## Usage

```bash
echo 'select a, b, c as something from a inner join b on a.id = b.id where a = 1' | sql-butterfly

    select a
         , b
         , c as something
      from a
inner join b on a.id = b.id
     where a = 1
```
