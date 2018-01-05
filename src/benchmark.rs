use std::fmt;

pub struct BenchmarkConfig {
    pub threads: i32,
    pub outer_reps: i32,
    pub test_time: f32,
    pub delay: f32,
}

impl fmt::Display for BenchmarkConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Threads: {}\r\nReps: {}\r\nTime: {}\r\nDelay: {}", self.threads, self.outer_reps, self.test_time, self.delay)
    }
}

pub struct Section {
    pub name: String,
    pub sample_size: i32,
    pub avg: f32,
    pub min: f32,
    pub max: f32,
    pub sd: f32, // standard deriviation?
    pub outliers: i32,
    pub time: f32,
    pub time_deriv: f32,
    pub overhead: f32,
    pub overhead_deriv: f32
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\r\nSample size: {}\r\nAvg: {}\r\nMin: {}\r\nMax: {}\r\nS.D.: {}\r\nOutliers: {}\r\nTime: {} +/-{}\r\nOverhead: {} +/-{}",
            self.name, self.sample_size, self.avg, self.min, self.max, self.sd, self.outliers, self.time, self.time_deriv, self.overhead, self.overhead_deriv
        )
    }
}

pub struct Benchmark {
    pub config: BenchmarkConfig,
    pub sections: Vec<Section>
}

impl fmt::Display for Benchmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut secstr = String::new();
        for sec in self.sections.iter() {
            secstr = secstr + "\r\n------------------------------\r\n" + &sec.to_string();
        }
        write!(f, "{}\r\n{}", self.config, secstr)
    }
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
        sections: Vec::new()
    }
}

fn empty_section() -> Section {
    Section {
        name: String::new(),
        sample_size: 0,
        avg: 0.0,
        min: 0.0,
        max: 0.0,
        sd: 0.0,
        outliers: 0,
        time: 0.0,
        time_deriv: 0.0,
        overhead: 0.0,
        overhead_deriv: 0.0
    }
}

pub fn create_benchmark_from_data(data: &str) -> Result<Benchmark, &'static str> {
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

    let mut data : Vec<&str> = Vec::new();

    while let Some(content) = line.next() {
        if content.trim() == "" {
            continue;
        } else if content.contains("----------") { // the benchmarks are sperated by a lot of "-" in the file
            let section = create_section_from_data(&data);
            benchmark.sections.push(section);
            data = Vec::new();
        } else {
            data.push(content);
        }
    }
    let section = create_section_from_data(&data);
    benchmark.sections.push(section);

    Ok(benchmark)
}

fn create_section_from_data(data: &Vec<&str>) -> Section {
    let mut section = empty_section();
    let mut cur = data.iter();
    let mut head = cur.next().unwrap().split_whitespace();
    let mut name = String::new();
    if head.clone().count() > 5 {
        for i in 1 .. head.clone().count() - 4 {
            if i == 1 {
                name = head.nth(i).unwrap().to_string();
            } else {
                name = name + " " + head.next().unwrap();
            }
        }
    }
    section.name = name;
    cur.next(); // skip over the heading for the benchmark data
    let mut nums = cur.next().unwrap().split_whitespace();
    section.sample_size = nums.next().unwrap().parse::<i32>().unwrap();
    section.avg         = nums.next().unwrap().parse::<f32>().unwrap();
    section.min         = nums.next().unwrap().parse::<f32>().unwrap();
    section.max         = nums.next().unwrap().parse::<f32>().unwrap();
    section.sd          = nums.next().unwrap().parse::<f32>().unwrap();
    section.outliers    = nums.next().unwrap().parse::<i32>().unwrap();

    let mut timestr = cur.next().unwrap().split_whitespace();
    let index = timestr.clone().count() - 4;
    section.time = timestr.nth(index).unwrap().parse::<f32>().unwrap();
    section.time_deriv = timestr.last().unwrap().parse::<f32>().unwrap();

    let overhead = cur.next();
    if overhead != None {
        let mut overhead = overhead.unwrap().split_whitespace();
        let index = overhead.clone().count() - 4;
        section.overhead = overhead.nth(index).unwrap().parse::<f32>().unwrap();
        section.overhead_deriv = overhead.last().unwrap().parse::<f32>().unwrap();
    }

    return section;
}