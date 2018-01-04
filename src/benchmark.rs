pub struct BenchmarkConfig {
    pub threads: i32,
    pub outer_reps: i32,
    pub test_time: f32,
    pub delay: f32,
}

pub struct Benchmark {
    pub config: BenchmarkConfig,
    pub sample_size: i32,
    pub avg: f32,
    pub min: f32,
    pub max: f32,
    pub sd: f32, // standard deriviation?
    pub time: f32,
    pub overhead: f32,
}

fn empty_benchmark_config() -> BenchmarkConfig {
    BenchmarkConfig {
        threads: 0,
        outer_reps: 0,
        test_time: 0.0,
        delay: 0.0
    }
}

fn empty_benchmark() -> Benchmark {
    Benchmark {
        config: empty_benchmark_config(),
        sample_size: 0,
        avg: 0.0,
        min: 0.0,
        max: 0.0,
        sd: 0.0,
        time: 0.0,
        overhead: 0.0
    }
}

pub fn create_benchmark_from_data(data: String) -> Result<Benchmark, &'static str> {
    let mut benchmark = empty_benchmark();
    let mut line = data.lines();

    // get the config from the beginning of the data
    let mut content = line.next().expect("data for benchmark creation was empty");
    if !content.contains("OpenMP benchmark") {
        return Err("not a OpenMP result file");
    }
    
    for i in 1 .. 6 {
        match line.next() {
            Some(x) => content = x,
            None    => return Err("invalid header")
        }
        let mut split = content.split_whitespace();
        if let Some(value) = split.next() {
            if i == 1 {
               // println!("1: {}", value);
                benchmark.config.threads = value.parse::<i32>().unwrap();
            } else if i == 2 {
               // println!("2: {}", value);
                benchmark.config.outer_reps = value.parse::<i32>().unwrap();
            } else if i == 3 {
               // println!("3: {}", value);
                benchmark.config.test_time = value.parse::<f32>().unwrap();
            } else if i == 4 {
               // println!("4: {}", value);
                continue
            } else if i == 5 {
               // println!("5: {}", value);
                benchmark.config.delay = value.parse::<f32>().unwrap();
            }
        }
    }
    line.next();
    line.next();

  //  while let Some(content) = line.next() {

 //   }

    Ok(benchmark)
}
