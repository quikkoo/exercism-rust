Exercism Rust
=============

[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

[![CircleCI](https://circleci.com/gh/quikkoo/exercism-rust.svg?style=svg)](https://circleci.com/gh/quikkoo/exercism-rust)
[![Codeship Status for quikkoo/exercism-rust](https://app.codeship.com/projects/8cf1fd10-31da-0135-4ad1-12facb29b565/status?branch=master)](https://app.codeship.com/projects/225907)

Rust exercises from [exercism.io](http://exercism.io/)

This is a multi module project for all exercises in rust language,
it makes easier to manage some of the common features and tasks between all exercises,
but each one can still be built individually.

### Check code

```sh
make check
```

### Run tests

```sh
make test
```

### Build package

```sh
make
```


Exercises details
-----------------

Eventually, some of these exercises are not exactly what you fetch using 
`exercism fetch`, and there are different reasons for that:

- Some languages have different tests compared to others, 
  so i modify the test suite by adding or removing test cases
- The project is structured using some standard tools, 
  specifically for each technology
- The code is written aiming to apply some common patterns
  used by the target language community

Despite these differences, all test suites are essentially the same, it keeps 
the same logic and consistency (in most cases, i hope).

Feel free to clone this repository and use it to learn or improve your 
development skills, and i also encourage you to create an account on 
[exercism.io](http://exercism.io/)
and share your own solutions and doubts about these exercises and others.

You can also follow the discussion on the exercism track 
[exercism.io/quikkoo](http://exercism.io/quikkoo), 
to suggest me some improvements or to ask questions.
