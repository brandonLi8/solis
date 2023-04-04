fun fib(n: int) : int {
  let a: int = true
  if n <= 1 {
    1
  }
  else {
    fib(n - 1) + fib(n - 2)
  }
}

fun a() : int {
  2
}

fun a() : () {
  let a: bool = false
}

let a: bool = fib(1)