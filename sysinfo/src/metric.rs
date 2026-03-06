use std::{collections::VecDeque, ops::{AddAssign, DivAssign, SubAssign}};

pub trait Capturer: Send + Sync {
  type Sample: for <'a> self::Sample<'a>;
  
  fn prep_capture(&mut self) {
  }
  
  fn capture(&mut self) -> Option<Self::Sample>;
}

// A metric value, it requires to implements
// subtraction, addition, division (with an usize
// scalar value)
pub trait Sample<'a>: Clone +
  SubAssign<&'a Self> +
  AddAssign<&'a Self> +
  DivAssign<f64>
  where Self: 'a
{
  // Given data like (an example not valid rust)
  // lhs: CpuUsage {
  //   core0: 0.9,
  //   core1: 0.2,
  //   core2: 0.5,
  //   core3: 0.1
  // };
  //
  // rhs: CpuUsage {
  //   core0: 0.2,
  //   core1: 0.6,
  //   core2: 0.3,
  //   core3: 0.05
  // };
  //
  // Output would be (changes the &mut self, like AddAssign/SubAssign/etc)
  // output: CpuUsage {
  //   core0: 0.2,  // Core 0 has lowest utilization on both sample, got from rhs
  //   core1: 0.2,  // Core 1 has lowest utilization on both sample, got from lhs (implicitly is &mut self)
  //   core2: 0.3,  // Core 2 has lowest utilization on both sample, got from rhs
  //   core3: 0.05  // Core 3 has lowest utilization on both sample, got from rhs
  // };
  //
  // It is performing min/max on individual field (if sample contains multiple
  // values)
  fn do_min_on_all_fields(&mut self, rhs: &Self);
  fn do_max_on_all_fields(&mut self, rhs: &Self);
}

pub struct SampleData<Capturer: self::Capturer> {
  pub samples: VecDeque<Option<Capturer::Sample>>,
  pub average: Option<Capturer::Sample>,
  pub min: Option<Capturer::Sample>,
  pub max: Option<Capturer::Sample>,
  pub delta: Option<Capturer::Sample>
}

// A metric is a value who can change overtime
// like CPU utilization, memory usage, etc
pub struct Metric<Capturer: self::Capturer> {
  capturer: Capturer,
  historical_samples: usize,
  pub data: SampleData<Capturer>
}

impl<Capturer: self::Capturer> Metric<Capturer> {
  pub fn new(capturer: Capturer, historical_samples: usize) -> Self {
    Self {
      capturer,
      historical_samples,
      data: SampleData {
        samples: VecDeque::new(),
        average: None,
        max: None,
        min: None,
        delta: None
      }
    }
  }
  
  pub fn prep_update(&mut self) {
    self.capturer.prep_capture();
  }
  
  pub fn update(&mut self) -> Option<Capturer::Sample> {
    let latest_value = self.capturer.capture();
    let data = &mut self.data;
    
    // Add sample to the queue
    while data.samples.len() >= self.historical_samples {
      // Drop old data till there enough space to fit one :3
      data.samples.pop_back();
    }
    
    data.delta = None;
    if let Some(mut latest_value) = latest_value.clone() {
      if let Some(Some(prev)) = data.samples.front() {
        latest_value -= prev;
        data.delta = Some(latest_value);
      }
    }
    
    data.samples.push_front(latest_value.clone());
    Self::calc_average(data);
    Self::calc_min(data);
    Self::calc_max(data);
    
    latest_value
  }
  
  fn process_samples<F>(data: &mut SampleData<Capturer>, mut f: F) -> Option<Capturer::Sample>
    where F: FnMut(&mut Capturer::Sample, &Capturer::Sample)
  {
    let mut sample_iterator = data.samples.iter()
      .flat_map(Option::iter);
    
    let Some(mut result) = sample_iterator.next().cloned() else {
        return None;
      };
    
    for sample in sample_iterator {
      f(&mut result, sample);
    }
    
    Some(result)
  }
  
  fn calc_average(data: &mut SampleData<Capturer>) {
    let count = data.samples.len() as f64;
    data.average = Self::process_samples(data, |prev, now| {
        *prev += now;
      })
      .map(|mut x| {
        x /= count;
        x
      });
  }
  
  fn calc_min(data: &mut SampleData<Capturer>) {
    data.min = Self::process_samples(data, Capturer::Sample::do_min_on_all_fields);
  }
  
  fn calc_max(data: &mut SampleData<Capturer>) {
    data.max = Self::process_samples(data, Capturer::Sample::do_max_on_all_fields);
  }
}


