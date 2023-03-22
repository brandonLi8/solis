fun fib(n: int, a: bool) : int {
  if n <= 1 {
    1
  }
  else {
    fib(n - 1) + fib(n - 2)
  }
}

let a: int = 2 - fib(5, false) * 3