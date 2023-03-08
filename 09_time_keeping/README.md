# Chapter 09 - Time Keeping

## Some Definitions

1. **Absolute Time:** Describes the time that you would tell someone if they were to ask for the timne. Also referred to as wall clock time and calendar time.
2. **Real-time Clock:** A physical clock that's embedded in the computer's motherboard, which keeps time when the power is off. Also known as CMOS clock.
3. **System Clock:** The OS view of the time. Upon boot, the OS takes over timekeeping duties from the real-time clock.
4. **Monotonically increasing:** A clock that never provides the same time twice.
5. **Steady Clock:** Provides two guarantees: its seconds are all equal length and it is monotonically increasing.
6. **High Accuracy:** A clock is highly accurate if the length of seconds are regular. The difference between two clocks is known as skew.
7. **High Resolution:** Provides accuracy down to 10ns or below.
8. **Fast Clock:** A clock that takes little time to read the time. Fast clocks sacrifice accuracy and precision for speed, however.