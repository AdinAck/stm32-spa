update:
    #!/bin/bash

    cd g4/out
    cargo clean
    cargo update cortex-m-spa proto-hal proto-hal-build
    cd ../model
    cargo clean
    cargo update proto-hal-model
    cd ../macros
    cargo clean
    cargo update proto-hal-build proto-hal-macros
