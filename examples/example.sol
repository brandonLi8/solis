let a: int = let b: int = a

##
let a: int = 0
let b: int = 0
let c: int = 0

let d: int = if (if true { c <= 0 } else { 1 + 2 + 3 < 6 }) {
  a + b
}
else if (if false { false } else { true }) {
  2
}
else {
  let d: int = b + c + 2
}



let a = 0
let b = 0
let c = 0

let temp2 = if true {                {a, b, c}
  c <= 0                             {temp2, a, b, c}
}
else {
  let temp0 = 1 + 2                  {temp2, a, b, c}
  let temp1 = temp0 + temp3          {temp0, temp2, a, b, c}
  temp1 < 6                          {temp1, temp2, a, b, c}
}

let d = if temp2 {                                             {temp2, a, b, c}
  a + b                              {d, a, b}
}
else {
  let temp3 = if false {             {d, b, c}
    false
  }
  else {
    true
  }

  if temp3 {                         {d, b, c, temp3}
    2                                {d}
  } else {
    let temp4 = b + c                {d, b, c}
    let e = temp4 + 2                {d, temp4}
    e                                {e, d}
  }
}

d                                                               {d}
##