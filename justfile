update:
    #!/bin/bash

    cd g4/out
    cargo clean
    cargo update cortex-m-spa proto-hal
    cd ../model
    cargo clean
    cargo update proto-hal-build
