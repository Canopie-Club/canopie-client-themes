FROM gemini-cli-sandbox

# Update default packages
RUN apt-get -qq update

# RUN curl https://sh.rustup.rs -sSf | sh
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
