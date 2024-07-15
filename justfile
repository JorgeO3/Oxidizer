default:
    echo 'Hello, world!'

oxi *args:
    cargo r --bin oxi -- {{ args }}