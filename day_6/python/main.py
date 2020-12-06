from collections import Counter

data: str = open('input').read()
clock = 0

for group in data.split('\n\n'):
    counter = Counter()

    for text in group.split('\n'):
        if text.strip() == '':
            continue

        counter += Counter(text)

    for number in counter.values():
        if number == len(group.split('\n')):
            clock += 1

print(clock)