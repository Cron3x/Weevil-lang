# Weevilang
---

A very stupid language, with a lots of bugs... you understand

types: 
 - Strings: str
 - Void:    void
 - Integer: int
 - float:   float
 - boolean: bool

```ini
;; This what can be parsed allready

entry!<>:int (
    puts! "Hello, World!"
    some_fn! 0
    => 0
)

```

```ini
;; Thast what a simple script should look like in the future
need <std>

entry! (
    let a = 1
    let b = 13
    let rt = ""
    if a < b (
        rt = cool_struct$ num: a, st: "rolf"
    )
    else (
        rt = cool_struct$ num: b, st: "torben"
    )

    puts! rt
)

cool_struct$ (
    num: int
    st: str
    
    new!<num: int, st: str>: self(
        self.a = a
        self.b = b

        => self
    )

    concat_num_str!<self>: str (
        let a = self.num.to_str!
        => a + self.st
    )
)

cool_enum#(
    Variant(int)
    Variant2(String)
    Variant3(float)
    Variant4
)
    
```

Tables of reminding:

|   name   | indicator | sure about | implemented |
|----------|-----------|------------|-------------|
| function | suffix ´!´|            |      ✅     |
| struct   | suffix ´$´|     ❌     |      ❌     |
| keyword  | no suffix |            |      ✅     |
| vars     | no suffix |     ✅     |      ❌     |


| my type | rust type | implemented |
|---------|-----------|-------------|
| int     | i32       |      ✅     | 
| str     | String    |      ✅     |
| void    | ()        |      ✅     |
| float   | f64       |      ❌     |
| bool    | bool      |      ❌     |

'return' = '=>'
