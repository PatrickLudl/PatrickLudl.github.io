# Patrick Ludl's Website

Accessible via https://patrickludl.github.io/.

## Create html files showing source code

* Create markdown file containing source code in code block.
* Convert the markdown file to html using pandoc: ```pandoc -s -f markdown -t html --highlight-style=pygments --metadata title="page_title" -o output.html markdown_source.md```
