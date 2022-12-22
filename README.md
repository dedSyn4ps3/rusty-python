# 🐍 rusty-python 🦀

## A basic project to demonstrate building Python modules implemented in Rust

<br>

> This crate was written to test with the [`python-with-rust`]("https://github.com/dedsyn4ps3/python-with-rust") command line application 

<br>

## Overview 🔍
While this project is by no means extensive, it's meant to at least serve as a starting point for those looking to improve their Python applications by writing and implementing custom modules in Rust.

<br>

## Building Blocks 🧱
This module utilizes the command line tool `maturin` and the `pyo3` crate to create Python functions, which in turn can be imported into and called from regular Python applications.

The process of creating a new Python-compatible Rust module is relatively straightforward:

 - Install the `maturin` command line tool -> `pipx install maturin`
 - Create a new module project set to use `pyo3` -> `maturin new -b pyo3 some-module`
 - Open up `lib.rs` and define a couple functions annotate with the `pyfunction` attribute
 - Add the new functions to a separate function declaration annotated with the `pymodule` attribute

<br>

## Next Steps 🗺
After getting your module all put together, it's time to upload it to a version control system (i.e `Github`) to allow for easy installing and implementation of the package in some Python code!

Once your code is available on `Github` ( or whatever platform you decided to use ), all that's left to do is actually install it for use in a Python project. This can be achieved by using `pip` just like you would any other Python package, with one caveat; you'll need to tell `pip` that it's installing a package from version control by passing a special argument to the command:

```
pip install git+https://github.com/<username>/<repo>@main
```

<br>

>**NOTE:** It's important that you include the `@main` at the end of the command to ensure you're installing the most current version of your package.

<br>

<br>

## Additional Info 💬
For more detailed information on how to develop a Rust-Python project from start to be, be sure to check out Parts 1 & 2 of `Make your Python Faster with Rust` on [**Medium**](https://www.medium.com/@erutherford_nullreturn)
