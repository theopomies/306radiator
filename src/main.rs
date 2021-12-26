use std::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use args::{get_args, Arguments};

const HELP_MESSAGE: &str = "USAGE
\t./306radiator n ir jr [i j]
DESCRIPTION
\tn\t\tsize of the room
\t(ir, jr)\tcoordinates of the radiator
\t(i, j)\t\tcoordinates of a point in the room";

mod args;

fn main() {
    match radiator() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(84)
        }
        Ok(None) => println!("{}", HELP_MESSAGE),
        _ => {}
    }
}

fn radiator() -> Result<Option<()>, String> {
    let args = get_args()?;
    let (s, ir, jr) = match args {
        Arguments::Matrix { s, ir, jr, .. } => (s, ir, jr),
        Arguments::Temperature { s, ir, jr, .. } => (s, ir, jr),
        _ => return Ok(None),
    };
    let n = s.pow(2);
    let r = ir + jr * s;
    let mut matrix = get_matrix(n, s);
    if let Arguments::Matrix { .. } = args {
        println!("{}", matrix);
        println!();
    }
    for i in 0..n {
        matrix[i].push(if i == r { -300.0 } else { 0.0 });
    }
    for i in 0..n {
        for k in i..n {
            if (matrix[k][i]).abs() > (matrix[i][i]).abs() {
                matrix.swap(k, i);
            }
        }
        for k in (i + 1)..n {
            let f = matrix[k][i] / matrix[i][i];
            for j in i..(n + 1) {
                matrix[k][j] -= f * matrix[i][j];
            }
        }
    }
    let mut vector = std::iter::repeat(0.0).take(n).collect::<Vec<_>>();
    vector[n - 1] = matrix[n - 1][n] / matrix[n - 1][n - 1];
    for i in (0..n).rev() {
        let mut c = 0.0;
        for j in (i + 1)..n {
            c += matrix[i][j] * vector[j];
        }
        vector[i] = (matrix[i][n] - c) / matrix[i][i];
    }
    if let Arguments::Matrix { .. } = args {
        for t in vector {
            println!("{:.1}", (t * 10.0).round() / 10.0);
        }
    } else if let Arguments::Temperature { i, j, .. } = args {
        println!("{:.1}", (vector[i + j * s] * 10.0).round() / 10.0)
    }
    Ok(Some(()))
}

struct Matrix<T>(Vec<Vec<T>>);

fn get_matrix(n: usize, s: usize) -> Matrix<f64> {
    Matrix(
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| get_matrix_element(i, j, s))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

fn get_matrix_element(i: usize, j: usize, s: usize) -> f64 {
    match [i % s, i / s].iter().all(|x| 1 <= *x && *x <= s - 2) {
        false if i == j => 1.0,
        false => 0.0,
        _ => match if i > j { i - j } else { j - i } {
            0 => -16.0,
            a if a == 1 || a == s => 4.0,
            _ => 0.0,
        },
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Matrix(matrix) = self;
        for a in matrix {
            writeln!(
                f,
                "{}",
                a.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("\t")
            )?;
        }
        Ok(())
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}
