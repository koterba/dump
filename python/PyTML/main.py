
example_script = """\

html>lang|en {
    head {

    }

    body.hello {
        script>src|index.js {

        }
    }
}
"""

result = ""
indent_level = 0
prev_tags = []


for original_line in example_script.split("\n"):
    line = original_line.strip().split()

    if len(line) == 0:
        result += "\n"
        continue

    pure_tag = line[0]
    tag = (pure_tag + '.')[:-1]

    if tag[0] == "|":
        result += "".join("    " for _ in range(indent_level)) + f"{original_line.strip()[1:]}\n"
        continue

    if ">" in tag:
        pure_tag, CUSTOM_TAG = tag.split(">")
        TAG_NAME, CUSTOM_TAG = CUSTOM_TAG.split("|")
        tag = f"{pure_tag} {TAG_NAME}={CUSTOM_TAG}"

    elif "#" in tag:
        pure_tag, ID = tag.split("#")
        tag = f"{pure_tag} id={ID}"

    elif "." in tag:
        pure_tag, CLASS = tag.split(".")
        tag = f"{pure_tag} class={CLASS}"

    if tag != "}":
        next_line = "".join("    " for _ in range(indent_level)) + f"<{tag}>\n"
        indent_level += 1
        result += next_line
        prev_tags.append(pure_tag)
    else:
        closing_tag = prev_tags.pop()
        indent_level -= 1
        next_line = "".join("    " for _ in range(indent_level)) + f"</{closing_tag}>\n"
        result += next_line


print(result)
