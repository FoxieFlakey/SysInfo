use std::{fs::File, io::{self, Read}, path::Path, sync::LazyLock};

pub fn read_all(path: &Path) -> Result<String, io::Error> {
  let mut file = File::open(path)?;
  let mut str = String::new();
  file.read_to_string(&mut str)?;
  Ok(str)
}

static CLOCK_TICK: LazyLock<u32> = LazyLock::new(|| {
  unsafe {
    let tick_rate = libc::sysconf(libc::_SC_CLK_TCK);
    if tick_rate < 0 {
      panic!("Cannot get system's clock rate: {}", io::Error::last_os_error());
    }
    
    u32::try_from(tick_rate).expect("Scheduler clock ticks faster than 4 Ghz?")
  }
});

pub fn get_clock_tick_speed() -> u32 {
  *CLOCK_TICK
}

