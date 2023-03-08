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

## Sending NTP requests and interpreting responses
Let's consider a client-server situation where your computer wants to correct its own
time. For ever computer that you check with -- let's call these time servers -- there are
two messages:
- The message from your computer to each time server is the request
- The reply is known as the response

These two messages generate four time points. Note that these occur in serial:
- T1 - The client's timestamp for when the request was sent. Referred to as t1 in code.
- T2 - The time server's timestamp for when the request was received. Referred to as t2 in code.
- T3 - The time server's timestamp for when it send its response. Referred to as t3 in code.
- T4 - The time client's timestamp for when the response was received. Referred to as t4 in code.


```shell
  +-----------+                         +-----------+
  |           | (T1) -----------> (T2)  |           |
  |  Client   |                         |   Server  |  
  |           | (T4) <----------- (T3)  |           |
  *-----------+                         +-----------+

```
The names T1 - T4 are designated by the RFC 2030 specification.