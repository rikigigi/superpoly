# superpoly

This produces a polynomial with the following:
 - p(x) = p(-x)
 - p(1) = 0
 - p'(1) = 0
 - p'(3) = 0
 - p(0) = h_0
 - p(3) = h_1
and then, for an arbitrary number of points it is possible to specify the second derivative to fix oscillating issues

The polynomial has the following form:

p(x) = a_2 (x^2-1)^2 + a_3 (x^2-1)^2 + ...

# example run

```
cargo run <<< '1                             0:31:29
3
2
2.19
0
3
0
' > out
```

```
  ┌                                                                                           ┐
  │                 1                -1                 1                -1                 1 │
  │                64               512              4096             32768            262144 │
  │                16               192              2048             20480            196608 │
  │           23.9444      186.22375926 1195.035123715048 6868.591676247881 36745.90928649707 │
  │                44               672              8704            102400           1130496 │
  └                                                                                           ┘



  ┌   ┐
  │ 1 │
  │ 3 │
  │ 0 │
  │ 0 │
  │ 0 │
  └   ┘


values of coefficiente a_2 ... a_6

  ┌                        ┐
  │     0.6905920267814301 │
  │   -0.26137231007916456 │
  │    0.04425794189392759 │
  │  -0.003658592241500982 │
  │ 0.00011912900397681239 │
  └                        ┘
```

plot of the polynomial:

![plot of p(x)](poly.png)

