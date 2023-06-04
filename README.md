# pyo3

A small code bit to illustrate we can publish rust libraries as a python package
Taking simple use case like a module to count the words or a sum of integers we create a virtual  python environment. Our rust code can be complied into a shared library which our python code cana import. 
Using maturin we build our rust code in python library and install it in our virtual python environment.

