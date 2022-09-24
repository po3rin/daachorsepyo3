from daachorsepyo3 import substring_match

text_list = ["全世界の子供たちが", "世界は広い"]
patterns = ["世界", "子供"]

result = substring_match(
    text_list,
    patterns
)

for i, r in enumerate(result):
    print('----------')
    print(text_list[i])
    print([patterns[pattern_index] for pattern_index in r])
