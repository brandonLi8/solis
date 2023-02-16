# Copyright © 2022-2023 Brandon Li. All rights reserved.

## Python script that was used to generate the ints_and_bools_random_* integration tests.

import random
random.seed(0)

id_count = -1
def get_unique_id():
  global id_count
  id_count += 1
  return chr(ord('a') + id_count % 24) + str(id_count // 24 + 1)

# Returns a random id in bindings where the type_ref matches
def get_random_binding(bindings, type_ref):
  choices = []
  for name in bindings:
    if bindings[name][0] == type_ref:
      choices.append(name)
  result = random.choice(choices)
  return result, bindings[result][1]

# returns if there are any bindings of type type_ref
def has_type_ref(bindings, type_ref):
  for name in bindings:
    if bindings[name][0] == type_ref:
      return True
  return False

#!————————————————————————————————————————————————————————————————————————————*!
# Int Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Returns a random int literal
def get_random_int_literal(bindings):
  result = random.randrange(1000)
  return str(result), result

# Returns a random int binding
def get_random_int_binding(bindings):
  return get_random_binding(bindings, "int")

# Returns a new random int binding
def get_new_random_int_binding(bindings):
  id_name = get_unique_id()
  random_int = gen_random_int(bindings)
  result = "let " + id_name + ": int = " + random_int[0]
  bindings[id_name] = ("int", random_int[1])
  return result, random_int[1]

def get_random_int_from_binary_exp(bindings):
  def floor_div(a, b):
    return int(a / b)

  def floor_mod(a, b):
    return a - int(a/b) * b

  operators = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', floor_div),
    ('%', floor_mod)
  ]
  random_int_1 = gen_random_int(bindings)
  random_int_2 = gen_random_int(bindings)

  if random_int_2[1] == 0:
    operators = operators[:-2]
  operator = random.choice(operators)

  return '(' + random_int_1[0] + ') ' + operator[0] + ' (' + random_int_2[0] + ')', operator[1](random_int_1[1], random_int_2[1])

#!————————————————————————————————————————————————————————————————————————————*!
# Bool Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Returns a random bool literal
def get_random_bool_literal(bindings):
  return random.choice([("true", True), ("false", False)])

# Returns a random bool binding
def get_random_bool_binding(bindings):
  return get_random_binding(bindings, "bool")

# Returns a new random bool binding
def get_new_random_bool_binding(bindings):
  id_name = get_unique_id()
  random_bool = gen_random_bool(bindings)
  result = "(let " + id_name + ": bool = " + random_bool[0] + ")"
  bindings[id_name] = ("bool", random_bool[1])
  return result, random_bool[1]

def get_random_bool_from_binary_exp(bindings):
  operators = [
    ('<', lambda a, b: a < b),
    ('<=', lambda a, b: a <= b),
    ('>', lambda a, b: a > b),
    ('>=', lambda a, b: a >= b)
  ]
  random_int_1 = gen_random_int(bindings)
  random_int_2 = gen_random_int(bindings)
  operator = random.choice(operators)
  return '(' + random_int_1[0] + ') ' + operator[0] + ' (' + random_int_2[0] + ')', operator[1](random_int_1[1], random_int_2[1])

def get_random_bool_from_equality(bindings):
  operators = [('==', lambda a, b: a == b), ('!=', lambda a, b: a != b)]
  operand_gen = random.choice([gen_random_int, gen_random_bool])

  operand_1 = operand_gen(bindings)
  operand_2 = operand_gen(bindings)
  operator = random.choice(operators)
  return '(' + operand_1[0] + ') ' + operator[0] + ' (' + operand_2[0] + ')', operator[1](operand_1[1], operand_2[1])

#!————————————————————————————————————————————————————————————————————————————*!
# Expression Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Returns a random int
def gen_random_int(bindings, allow_let=True):
  generators = [get_random_int_literal, get_random_int_from_binary_exp]

  if has_type_ref(bindings, "int"):
    generators.append(get_random_int_binding)
    generators.append(get_random_int_binding)
    generators.append(get_random_int_binding)

  if allow_let:
    generators.append(get_new_random_int_binding)

  result = random.choice(generators)(bindings)

  if random.choice([True, False]):
    return '-(' + result[0] + ')', -result[1]
  return result


# Returns a random bool
def gen_random_bool(bindings):
  generators = [get_random_bool_literal, get_new_random_bool_binding, get_random_bool_from_binary_exp, get_random_bool_from_equality]

  if has_type_ref(bindings, "bool"):
    generators.append(get_random_bool_binding)

  result = random.choice(generators)(bindings)

  if random.choice([True, False]):
    return '!(' + result[0] + ')', not result[1]
  return result


# Returns a random let expression
def gen_random_expr(bindings):
  type_ref = random.choice(["int"])
  if type_ref == "int":
    expr = gen_random_int(bindings, allow_let=True);
    return expr[0] + ';', expr[1]
  if type_ref == "bool":
    expr = gen_random_bool(bindings);
    return expr[0] + ';', expr[1]

#!————————————————————————————————————————————————————————————————————————————*!
# Test generator
#!————————————————————————————————————————————————————————————————————————————*!

def gen_test(i):
  global id_count
  id_count = -1

  # the bindings defined in the scope
  bindings = {} # map name to (type, value)
  result = []

  for _ in range(200):
    expr = gen_random_expr(bindings)
    result.append(expr[0])

  # result is a summation of all
  result.append(" + ".join(filter(lambda n: bindings[n][0] == 'int', bindings.keys())))

  program = """# Copyright © 2022-2023 Brandon Li. All rights reserved.

##
Random test of a Solis program that uses ints, bools, and all the operators that are associated with them.
NOTE: this file was auto-generated with the `ints_and_bools_random.py` script.
##

"""

  file = open('./tests/integration/ints_and_bools_random_' + i + '.sol', 'w+')
  file.write(program + "\n".join(result))
  file.close()

  expected_file = open('./tests/integration/expected/ints_and_bools_random_' + i + '.out', 'w+')
  expected_file.write(str(sum(map(lambda n: bindings[n][1], filter(lambda n: bindings[n][0] == 'int', bindings.keys())))))
  expected_file.close()


for i in range(1, 5):
  gen_test(str(i))
