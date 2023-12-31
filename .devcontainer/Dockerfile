FROM mcr.microsoft.com/devcontainers/rust:latest

RUN sudo apt-get update

RUN echo 'bind '\''"\e[A": history-search-backward'\' >> /etc/bash.bashrc
RUN echo 'bind '\''"\e[B": history-search-forward'\' >> /etc/bash.bashrc
RUN echo 'alias ll="ls -l"' >> /etc/bash.bashrc
ARG USERNAME=vscode
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir /commandhistory \
    && touch /commandhistory/.bash_history \
    && chown -R $USERNAME /commandhistory \
    && echo "$SNIPPET" >> "/home/$USERNAME/.bashrc"

RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
RUN wget https://github.com/mozilla/grcov/releases/download/v0.8.18/grcov-x86_64-unknown-linux-gnu.tar.bz2 && tar -xvjf grcov-x86_64-unknown-linux-gnu.tar.bz2 -C /usr/local/bin
RUN rustup component add llvm-tools-preview
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo install cargo-audit && cargo install sccache
RUN echo "export SCCACHE_BUCKET=sccache-aws-cli-dev" >> "/home/$USERNAME/.bashrc" \
    && echo "export SCCACHE_S3_USE_SSL=true" >> "/home/$USERNAME/.bashrc" \
    && echo "export RUSTC_WRAPPER=/usr/local/cargo/bin/sccache" >> "/home/$USERNAME/.bashrc" \
    && echo "export SCCACHE_REGION=us-west-2" >> "/home/$USERNAME/.bashrc" \
    && echo "export CARGO_INCREMENTAL=0" >> "/home/$USERNAME/.bashrc"