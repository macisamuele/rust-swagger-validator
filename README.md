[WIP] Rust Swagger Validator
============================

***This repository is meant as personal exercise for learning [Rust](https://www.rust-lang.org/) and python bindings via
[PyO3](https://github.com/PyO3/pyo3).***

![https://travis-ci.com/macisamuele/rust-swagger-validator](https://img.shields.io/travis/macisamuele/rust-swagger-validator/master.svg?logo=travis&label=Linux+%26+Mac)
![https://ci.appveyor.com/project/macisamuele/rust-swagger-validator](https://img.shields.io/appveyor/ci/macisamuele/rust-swagger-validator/master.svg?logo=appveyor&label=Windows)

Rationale
---------
Performing JSON and/or Swagger validation in python is generally slow, especially if we consider CPython implementation.
At the moment the *best* python library available is [jsonschema](https://github.com/Julian/jsonschema) but it is still
slow on CPython. The performance impact is mainly caused by the fact that JSON schema is extremely flexible and so a good
number of operations have to ensure that all the pre-conditions are respected.

The idea of this repository is to have a python library able to perform Swagger validation via a more performant validation
library eventually based on [valico](https://github.com/rustless/valico).

NOTE: [Swagger 2.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md) is the current target for the
experimentation. If the results will be promising and the APIs are stable I will extend the library to handle generic JSON
schemas and [OpenAPI 3.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md) specifications.

Benchmarking
------------
As mentioned this library should be able to perform validation of Swagger Spec and Swagger Objects, as the effort is aimed to
provide a more performant solution respect jsonschema benchmarking tests cases will be defined to identify whether the new
library is faster respect jsonschema and/or in which conditions that is true.

Expected Features
-----------------
This list is not meant to be complete as this is a *Work In Progress* repository.

Definitely I have in mind to support the following
- [ ] Load and validation of swagger specs from multiple resources (file/HTTP/HTTPS URIs and from as dictionaries from python
code) with multiple formats (JSON and YAML)
- [ ] Validation of a given JSON Object/Array/Primitive against part of the schema identified via JSON Reference in the
Swagger Specs (the API should answer to "*is OBJ valid according to the schema of the body parameter in endpoint E?*")
- [ ] Support for custom formats defined by the users (something similar to [FormatChecker](http://python-jsonschema.readthedocs.io/en/latest/validate/#jsonschema.FormatChecker)
in jsonschema)


Contributing
------------
The project is not implemented yet and a lot of features are missing, so please keep it in mind while opening Issues or Pull Requests.

ℹ️ issues requiring features will be appreciated but I would not guarantee that those will be implemented on the first iteration.
