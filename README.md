# Gabriel's Blog System

Gabs is a stupid system for creating websites with
[Github Flavored Markdown](https://github.github.com/gfm/). It consists of only
a single command to create a website and build it.

## Installation

Install it from [crates.io](crates.io), from source or from the
[releases](https://github.com/gboncoffee/gabs/releases) page:

```shell
$ cargo install gabs
```

Or:

```shell
$ git clone https://github.com/gboncoffee/gabs
$ cargo build --release
$ # then link the executable to somewhere in your path
```

## Usage

First, inside the directory of your website, run the command to create the
`_gabs` directory and build a simple default example.

Inside the `_gabs` directory, you place your files. HTML files are templates,
and Markdown files will be build with them. Stylesheets and scripts will be just
copied to their location and linked to their templates. Examples:

- The file `_gabs/index.md` will build to `index.html`  
- The file `_gabs/posts/post.md` will build to `posts/post.html`  
- The file `_gabs/index.html` is a template with name "index"  
- The file `_gabs/index.css` will be the stylesheet for any file with the
  "index" template and will be copied to `index.css`.  

If there are files called `footer.html` and/or `header.html`, they'll be added
to the bottom and the top of the `<body>` of every document. Of course, no
template can be named "footer" or "header".

If there are files called `global.css` or `global.js`, they'll be linked to
every document.

Note that the filename of Markdown files DOES NOT MEAN ANYTHING. To define the
template for a Markdown file, read below:

### The Gabs header for Markdown files

Markdown files can have a special first line called the Gabs header. It looks
like this:

```
#!gabs <template>: <title>
```

This line defines the template used to build it's file and the title for that
specific page. For example, `#!gabs index: Hello, World!` declares that the file
should be build against the "index" template and have the title "Hello,
World!". Of course, this line is not copied to the final HTML file.

### The Gabs template

Templates are normal HTML files. Inside them, a comment like "<!\-\- gabs \-\->"
will be replaced by the HTML produced from the Markdown file. For example, in
the template "post" defined at `post.html`, this:

```html
<div id="post">
  <!-- gabs -->
</div>
```

Will put the generated HTML of every document with the template "post" inside a
div with id "post". Note that whitespace around the comment doesn't matter, but
the comment itself should exactly match "<!\-\- gabs \-\->".
