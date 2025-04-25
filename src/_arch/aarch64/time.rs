use crate::warn;
use aarch64_cpu::{asm::barrier, registers::*};
use core::{
    num::{NonZeroU128, NonZeroU32, NonZeroU64},
    ops::{ Add, Div },
    time::Duration,
};
use tock_registers::interfaces::Readable;

//const NANOSEC_PER_SEC: u64 = 1_000_000_000;
const NANOSEC_PER_SEC: NonZeroU64 = NonZeroU64::new(1_000_000_000).unwrap();

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct GenericTimerCounterValue(u64);

/// written with CNTFREQ_EL0 in boot.s
#[no_mangle]
static ARCH_TIMER_COUNTER_FREQUENCY: NonZeroU32 = NonZeroU32::MIN;

fn arch_timer_counter_frequency() -> NonZeroU32 {
    // read_volatile so compiler doesn't optimize this away
    unsafe { core::ptr::read_volatile(&ARCH_TIMER_COUNTER_FREQUENCY) }
}

impl GenericTimerCounterValue {
    pub const MAX: Self = GenericTimerCounterValue(u64::MAX);
}

impl Add for GenericTimerCounterValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        GenericTimerCounterValue(self.0.wrapping_add(rhs.0))
    }
}

impl From<GenericTimerCounterValue> for Duration {
    fn from(counter_value: GenericTimerCounterValue) -> Self {
        if counter_value.0 == 0 {
            return Duration::ZERO;
        }
        
        let frequency: NonZeroU64 = arch_timer_counter_frequency().into();
        
        let secs = counter_value.0.div(frequency);
        
        let sub_second_counter_value = counter_value.0 % frequency;
        let nanos = unsafe { sub_second_counter_value.unchecked_mul(u64::from(NANOSEC_PER_SEC))}.div(frequency) as u32;
        
        Duration::new(secs, nanos)
    }
}

fn max_duration() -> Duration {
    Duration::from(GenericTimerCounterValue::MAX)
}

impl TryFrom<Duration> for GenericTimerCounterValue {
    type Error = &'static str;
    
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        if d < resolution() {
            return Ok(GenericTimerCounterValue(0));
        }
        if d > max_duration() {
            return Err("Duration out of range");
        }
        
        let frequency: u128 = u32::from(arch_timer_counter_frequency()) as u128;
        let duration: u128 = d.as_nanos();
        
        let counter_value = unsafe { duration.unchecked_mul(frequency)}.div(NonZeroU128::from(NANOSEC_PER_SEC));
        
        Ok(GenericTimerCounterValue(counter_value as u64))
    }
}

#[inline(always)]
fn read_cntpct() -> GenericTimerCounterValue {
    barrier::isb(barrier::SY);
    let cnt = CNTPCT_EL0.get();
    
    GenericTimerCounterValue(cnt)
}


pub fn resolution() -> Duration {
    Duration::from(GenericTimerCounterValue(1))
}

pub fn uptime() -> Duration {
    read_cntpct().into()
}

pub fn spin_for(duration: Duration) {
    let curr_counter_value = read_cntpct();
    
    let counter_value_delta: GenericTimerCounterValue = match duration.try_into() {
        Err(msg) => {
            warn!("spin_for: {}. Skipping", msg);
            return;
        }
        Ok(val) => val,
    };
    let counter_value_target = curr_counter_value + counter_value_delta;
    
    while GenericTimerCounterValue(CNTPCT_EL0.get()) < counter_value_target {}
}
