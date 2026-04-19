with open('icon.svg', 'r') as f:
    svg = f.read()

with open('index.html.template', 'r') as f:
    template = f.read()

output = template.replace('{{ICON_SVG}}', svg)

with open('index.html', 'w') as f:
    f.write(output)

print('index.html generated!')
