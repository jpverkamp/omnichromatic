import sys

r = 0
g = 0
b = 0

while True:
    command = sys.stdin.readline()
    
    if command.startswith('get-color'):
        response = f'color {r} {g} {b}\n'

        r += 1
        if r >= 255:
            r = 0
            g += 1

        if g >= 255:
            g = 0
            b += 1

        sys.stdout.write(response)
        sys.stdout.flush()
