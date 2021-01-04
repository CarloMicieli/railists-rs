# RaiLists.rs

![Rust](https://github.com/CarloMicieli/railists-rs/workflows/Rust/badge.svg)

A small CLI to manage model railways collection (from yaml files).

## How to build it

The application requires a working Rust installation.

```
$ cargo build --release
```

## How to use

```
$ railists -h
railists 0.1.0
CarloMicieli <piovarolo@gmail.com>
Model railway collection manager

USAGE:
    railists [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    collection    List all elements in the collection
    help          Prints this message or the help of the given subcommand(s)

$ railists collection -h
railists-collection 
List all elements in the collection

USAGE:
    railists collection [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    csv      Export the collection as csv file
    depot    Extract the depot information for locomotives
    help     Prints this message or the help of the given subcommand(s)
    list     List the collection elements
    stats    Calculate the collection statistics
```

`collection list` will list all elements in the current collection:

```
$ railists collection list -f lista.yaml
+-----+-------------+-------------+-------+----+------+----------------------------------------------------+-------+------------+------------+---------------------+
| #   | Brand       | Item number | Scale | PM | Cat. | Description                                        | Count | Added      | Price      | Shop                |
+-----+-------------+-------------+-------+----+------+----------------------------------------------------+-------+------------+------------+---------------------+
| 1   | ACME        | 40152       | H0    | DC |  F   | Bagagliaio ad assi per treni merci tipo Dm 98813   |     1 | 2011-03-19 |  35.00 EUR | Treni&Treni         |
+-----+-------------+-------------+-------+----+------+----------------------------------------------------+-------+------------+------------+---------------------+
| 2   | ACME        | 40153       | H0    | DC |  F   | Bagagliaio ad assi con terrazzino frenatore        |     1 | 2012-07-18 |  34.00 EUR | Treni&Treni         |
+-----+-------------+-------------+-------+----+------+----------------------------------------------------+-------+------------+------------+---------------------+
| 3   | ACME        | 40241       | H0    | DC |  F   | Carro chiuso FS tipo Ghkrs a passo lungo con 12... |     1 | 2019-01-08 |  29.90 EUR | Tecnomodel          |
+-----+-------------+-------------+-------+----+------+----------------------------------------------------+-------+------------+------------+---------------------+
```

`collection stats` will display the statistics for the current collection:

```
$ railists collection stats -f lista.yaml
Total value........... 99999.99 EUR
Rolling stocks/sets... 999
+-------+-------------------+-------------------+--------------+--------------+----------------------+----------------------+--------------------+--------------------+-------------+-------------+
| Year  | Locomotives (no.) | Locomotives (EUR) | Trains (no.) | Trains (EUR) | Passenger Cars (no.) | Passenger Cars (EUR) | Freight Cars (no.) | Freight Cars (EUR) | Total (no.) | Total (EUR) |
+-------+-------------------+-------------------+--------------+--------------+----------------------+----------------------+--------------------+--------------------+-------------+-------------+
| 2005  |                 0 |                 0 |            0 |            0 |                   14 |              1000.00 |                  4 |              52.50 |          10 |      687.30 |
+-------+-------------------+-------------------+--------------+--------------+----------------------+----------------------+--------------------+--------------------+-------------+-------------+
| 2006  |                 3 |          99999.99 |            0 |            0 |                   20 |               100.00 |                  2 |              10.00 |          25 |     1516.75 |
+-------+-------------------+-------------------+--------------+--------------+----------------------+----------------------+--------------------+--------------------+-------------+-------------+
| TOTAL |                 3 |          99999.99 |            0 |            0 |                   34 |              1100.00 |                  6 |              62.50 |          35 |    99999.99 |
+-------+-------------------+-------------------+--------------+--------------+----------------------+----------------------+--------------------+--------------------+-------------+-------------+
```

`collection depot` will display the list of locomotives in the current collection:

```
$ railists collection depot -f lista.yaml
108 locomotive(s)
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
| #   | Class name | Road number | Series               | Livery                                         | Brand     | Item Number | With decoder | DCC    |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
|  1  | D.141      | D.141 1004  |                      | verde/giallo                                   | Piko      | 52444       |      N       | PLUX22 |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
|  2  | D.143      | D.143 3019  |                      |                                                | ACME      | 60250       |      N       |        |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
|  3  | D.143      | D.143 3022  |                      | verde                                          | ACME      | 60253       |      N       |        |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
|  4  | D.145      | D.145 2004  |                      | arancio                                        | Piko      | 52846       |      N       |        |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
|  5  | D.145      | D.145 2016  |                      | arancio                                        | Piko      | 52844       |      N       |        |
+-----+------------+-------------+----------------------+------------------------------------------------+-----------+-------------+--------------+--------+
```

