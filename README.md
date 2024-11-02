### Fourier Series for Complex Functions

#### Overview

This project visualises a Fourier serie decomposition of a complex function. 
It creates a sum of rotating vectors with different sizes and speeds, that combined together create a approximation of the given function. 

The red circle on the axis represents the last coordinate of the sum of vectors.

#### Math behind the program

Every vector is described by it's coefficient :

$$
C_n = \int_0 ^1 f(t)dt
$$

The rotation of each vector is controlled by this equation :

$$
C_n \times e^{-n 2\pi it}
$$

As t goes from 0 to 1.1

Here :
- n represents the number of vectors
- t represents the time going from 0 to 1

Example :

if $n = 10$

The $C_n$'s would be :

$$
C_{-10} = \int_0^1{f(t)}
$$


$$
C_0 = \int_0^1f(t)
$$



$$
C_{-10} = \int_0^1{f(t)}
$$


So when calculating the exact position of a vector tip when t goes from 0 to 1 :

$$
V_{-10} = \int_0 ^1 f(t)\times e^{10\times 2\pi i t}
$$

$$

V_{0} = \int _0 ^1 f(t)
$$

$$
V_{10} = \int_0 ^1 f(t)\times e^{-10\times 2\pi i t}
$$

(for more info watch this [video](https://www.youtube.com/watch?v=r6sGWTCMz2k) by 3b1b)

#### Using the Program

To input you own function f(t) just modify the function `f_of_t()` 

#### Example Case for a square function 

```Rust
fn f_of_t(t: f32) -> Complex<f32> {
        // square wave

        if t <= 0.5 {
            return { Complex::new(0.0, 1.0) };
        } else 
            return { Complex::new(1.0, -1.0) };
        }
    }
```

(try to keep the value returned by the function < 1)

#### Credits

The biggest credits go to 3Blue1Brown's video which provided the majority of the math used in this program

The rest of the credits go to me.
