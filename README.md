# TinySSG

Tiny and fast static site generator.

TinySSG generates lighting fast static website. It is easily customizable with themes.


## Introduction
> **This Project is Work in Progress. Don't expect anything fancy _yet_.**

TinySSG is born out of pure curiosity. I could use a static site generator like [hugo](https://gohugo.io/) or [gatsby](https://www.gatsbyjs.com/) but nope, i choose to write my own one. It just is more fun to write stuff by your self, and it helps to understand how things work.


## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

The things you need before building tinyssG.

* git
* cargo

### Installation

A step by step guide that will tell you how to get the development environment up and running.

```sh
$ git clone https://github.com/b3yc0d3/tinyssg
$ cd tinyssg
$ cargo build
```

## Usage

```
tssg [options] source

Arguments:
  source    Path of source folder. If not specified, current
             working directory will be used.

Options:
  -o www    Set output path.
```

### Example

A example of how to generate a website with tinyssg.

```sh
tssg -o /out/path /path/to/raw/site
```

<!--## Deployment

Additional notes on how to deploy this on a live or release system. Explaining the most important branches, what pipelines they trigger and how to update the database (if anything special).

### Server

* Live:
* Release:
* Development:

### Branches

* Master:
* Feature:
* Bugfix:
* etc...

## Additional Documentation and Acknowledgments

* Project folder on server:
* Confluence link:
* Asana board:
* etc... -->