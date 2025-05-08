```
import io
import enum

attr Function

attr AddFn(arg: int): Function {
    if arg < __arg1__ {
        std::io::eprint("Error")
    }
}

#[Function]
#[AddFn(100)]
fn add<T: std::num::Number>(l: T, r: T) -> T {
    return l + r;
}

add<int>(10, 20) // 30

add<int>(101, 20) // Error!
```

