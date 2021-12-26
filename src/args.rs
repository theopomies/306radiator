pub enum Arguments {
    DisplayHelp,
    Matrix {
        s: usize,
        ir: usize,
        jr: usize,
    },
    Temperature {
        s: usize,
        ir: usize,
        jr: usize,
        i: usize,
        j: usize,
    },
}

pub fn get_args() -> Result<Arguments, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.contains(&"-h".into()) || args.contains(&"--help".into()) {
        return Ok(Arguments::DisplayHelp);
    }
    if ![3, 5].contains(&args.len()) {
        return Err("Invalid argument count.".into());
    }
    let args: Result<Vec<_>, _> = args.iter().map(|s| s.parse::<usize>()).collect();
    if let Err(err) = args {
        return Err(err.to_string());
    }
    let args = args.unwrap();
    let s = args[0];
    let ir = args[1];
    let jr = args[2];
    if s < 3 || ![ir, jr].iter().all(|x| 1 <= *x && *x <= s - 2) {
        return Err("Invalid args".into());
    }
    if args.len() == 3 {
        return Ok(Arguments::Matrix { s, ir, jr });
    }

    let i = args[3];
    let j = args[4];
    if ![i, j].iter().all(|x| *x <= s - 1) {
        return Err("invalid arg".into());
    }

    Ok(Arguments::Temperature { s, ir, jr, i, j })
}
