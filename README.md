## Firmware:
### NOTE: keep comments on top of the code, removal seems to lead to
```
#[instability::unstable]
     |     ------------------------ private method defined here
```
- clone esp-hal
- replace `gpio_interrupt.rs` example with `main.rs`
- ```shell
    cargo xtask run-example esp-hal esp32c3 gpio_interrupt
  ```

## Hardware:
- motor
- 2N2222 (NPN transistor)
- esp32c3
### Wiring:
- capacitor `104 250V` between motor terminals
- resistors for both motor terminals
- emitter -> esp gnd
- base -> 1kO -> esp pin4
- motor+ -> collector
- motor- -> esp 5V (or external power)
