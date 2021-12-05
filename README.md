# STM32 HDMI CEC

[![CI](https://github.com/newAM/stm32-cec/workflows/CI/badge.svg)](https://github.com/newAM/stm32-cec/actions?query=branch%3Amain)
[![nightly-docs](https://img.shields.io/badge/docs-nightly-black)](https://newam.github.io/stm32-cec/stm32_cec/index.html)

HDMI CEC driver for STM32 microcontrollers.

This has been tested on the STM32H7, but it should also work for other chips that have this peripheral.

## Status

This will not be actively maintained.

At the time I wrote this there were no other HDMI CEC drivers written in rust, I uploaded this for other people to reference.

## Limitations

* All it can do is turn my TV on and off.
* Probably not does not work with proper addressing.
  * Only tested with broadcast for the src/dst addresses.
* Does not handle out-of-order replies.
  * This is more of a TODO for myself because at the moment because turning my TV on and off does not generate any replies.
