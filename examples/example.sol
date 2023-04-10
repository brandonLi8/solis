fun fib(n: int) : int {
  if n <= 1 {
    1
  }
  else {
    fib(n - 1) + fib(n - 2)
  }
}

let a: int = 2 - fib(5) * 3