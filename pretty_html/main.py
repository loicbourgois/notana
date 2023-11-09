from bs4 import BeautifulSoup
from bs4.formatter import HTMLFormatter
formatter = HTMLFormatter(indent=4)
def read(path):
    with open(path, "r") as file:
        return file.read()
def write(path=None, content=None):
    assert path is not None
    assert content is not None
    with open(path, 'w') as file:
        file.write(content)
soup = BeautifulSoup(read("./common/src/example-generated.html"), 'html.parser')
html = soup.prettify(formatter=formatter)
write("./common/src/example-generated-2.html", html)
write(
    "./front/index.html", 
    BeautifulSoup(
        read("./front/index.html.template").replace("$BODY", html),
        'html.parser'
    ).prettify(formatter=formatter)
)
