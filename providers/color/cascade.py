import sys

r = 0
g = 0
b = 0

while True:
    command = sys.stdin.readline()
    
    if command.startswith('set-size'):
        _, width, height = command.strip().split()
        width = int(width)
        height = int(height)
        increment = int(256.0 / (width * height) ** (1/3))

    elif command.startswith('get-color'):
        response = f'color {r} {g} {b}\n'

        r += increment
        if r >= 255:
            r = 0
            g += increment

        if g >= 255:
            g = 0
            b += increment

        sys.stdout.write(response)
        sys.stdout.flush()
