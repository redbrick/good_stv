# good_stv

A tool for evaluating elections using
[Single Transferable Vote](https://en.wikipedia.org/wiki/Single_transferable_vote).

[![Build Status](https://travis-ci.org/redbrick/good_stv.svg?branch=master)](https://travis-ci.org/redbrick/good_stv)

## Usage

good_stv expects to receive CSV-formatted data, either on stdin or a file. The
only required argument is a positive integer representing the number of seats to
fill in the election. Example invocations are:

```sh
$ good_stv -f test.csv 3
```

```sh
$ good_stv 3 < test.csv
```

### CSV format

The input data is expected to be in the following format:

One (1) header line, with a list of candidates to be elected, in any order. This
is followed by any number of body lines, each containing a list of candidates,
in order of preference from highest to lowest, left to right, representing a
single vote. Any candidates listed in the body who are not also in the header
will be ignored. Each vote does not need to include every candidate, although
votes listing zero (0) candidates will be ignored.

#### Example

```csv
alice,bob,charlie
alice,bob
bob,charlie,alice
bob
```

### Web App

**Still under development**

To run the web app you first need to build it with yarn.

```shell
yarn
yarn build
```

Then run `cargo run -- --server` to start the server on `localhost:8000`

## License

```text
good_stv - a good single transferable vote utility.
Copyright (C) 2017 Terry Bolt

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
