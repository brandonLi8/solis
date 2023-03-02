# Copyright © 2022-2023 Brandon Li. All rights reserved.

## Python script that was used to generate the random* integration tests.

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
# Int Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Returns a random float literal
def get_random_float_literal(bindings):
  result = random.random() * 50.123
  return str(result), result

# Returns a random float binding
def get_random_float_binding(bindings):
  return get_random_binding(bindings, "float")

# Returns a new random float binding
def get_new_random_float_binding(bindings):
  id_name = get_unique_id()
  random_float = gen_random_float(bindings)
  result = "let " + id_name + ": float = " + random_float[0]
  bindings[id_name] = ("float", random_float[1])
  return result, random_float[1]

def get_random_float_from_binary_exp(bindings):
  operators = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', lambda a, b: a / b),
  ]
  operand_generators = [(gen_random_float, gen_random_float), (gen_random_float, gen_random_int), (gen_random_int, gen_random_float)]
  operand_generators = random.choice(operand_generators)

  random_float_1 = operand_generators[0](bindings)
  random_float_2 = operand_generators[1](bindings)

  if random_float_2[1] == 0:
    operators = operators[:-1]
  operator = random.choice(operators)

  return '(' + random_float_1[0] + ') ' + operator[0] + ' (' + random_float_2[0] + ')', operator[1](random_float_1[1], random_float_2[1])


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
  random_int_1 = random.choice([gen_random_int, gen_random_float])(bindings)
  random_int_2 = random.choice([gen_random_int, gen_random_float])(bindings)
  operator = random.choice(operators)
  return '(' + random_int_1[0] + ') ' + operator[0] + ' (' + random_int_2[0] + ')', operator[1](random_int_1[1], random_int_2[1])

def get_random_bool_from_equality(bindings):
  operators = [('==', lambda a, b: a == b), ('!=', lambda a, b: a != b)]
  operand_gen = random.choice([gen_random_int, gen_random_bool, gen_random_float])

  operand_1 = operand_gen(bindings)
  operand_2 = operand_gen(bindings)
  operator = random.choice(operators)
  return '(' + operand_1[0] + ') ' + operator[0] + ' (' + operand_2[0] + ')', operator[1](operand_1[1], operand_2[1])

#!————————————————————————————————————————————————————————————————————————————*!
# Expression Generators
#!————————————————————————————————————————————————————————————————————————————*!

# Returns a random int
def gen_random_int(bindings):
  generators = [get_random_int_literal, get_random_int_from_binary_exp]

  if has_type_ref(bindings, "int"):
    generators.append(get_random_int_binding)
    generators.append(get_random_int_binding)
    generators.append(get_random_int_binding)

  generators.append(get_new_random_int_binding)

  result = random.choice(generators)(bindings)

  if random.choice([True, False]):
    return '-(' + result[0] + ')', -result[1]
  return result

# Returns a random float
def gen_random_float(bindings):
  generators = [get_random_float_literal, get_random_float_from_binary_exp]

  if has_type_ref(bindings, "float"):
    generators.append(get_random_float_binding)
    generators.append(get_random_float_binding)
    generators.append(get_random_float_binding)

  generators.append(get_new_random_float_binding)

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
  type_ref = random.choice(["float"])
  if type_ref == "int":
    expr = gen_random_int(bindings);
    return expr[0] + ';', expr[1]
  if type_ref == "float":
    expr = gen_random_float(bindings);
    return expr[0] + ';', expr[1]
  if type_ref == "bool":
    expr = gen_random_bool(bindings);
    return expr[0] + ';', expr[1]

#!————————————————————————————————————————————————————————————————————————————*!
# Test generator
#!————————————————————————————————————————————————————————————————————————————*!

import struct

def float_printed(f):
    [d] = struct.unpack(">Q", struct.pack(">d", f))
    binary = f'{d:064b}'
    return int(binary[1:], 2) + (0 if binary[0] == '0' else -2 ** 63)

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
  result.append(" + ".join(filter(lambda n: bindings[n][0] == 'int' or bindings[n][0] == 'float' , bindings.keys())))
  expected = sum(map(lambda n: bindings[n][1], filter(lambda n: bindings[n][0] == 'int' or bindings[n][0] == 'float', bindings.keys())))

  if has_type_ref(bindings, "float"):
    expected = float_printed(expected)

  program = """# Copyright © 2022-2023 Brandon Li. All rights reserved.

##
Random test of a Solis program that uses ints, bools, and all the operators that are associated with them.
NOTE: this file was auto-generated with the `ints_and_bools_random.py` script.
##

"""

  file = open('./tests/integration/random' + i + '.sol', 'w+')
  file.write(program + "\n".join(result))
  file.close()

  expected_file = open('./tests/integration/expected/random' + i + '.out', 'w+')
  expected_file.write(str(expected))
  expected_file.close()

for i in range(1, 5):
  gen_test(str(i))
