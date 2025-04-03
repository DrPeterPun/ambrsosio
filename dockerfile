# Stage 1: Build Rust App
FROM rust:1.85 as builder
WORKDIR /usr/src/ambrsosio
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    curl
    
# Install Chrome
RUN wget -qO- https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb > chrome.deb \
    && apt install -y ./chrome.deb \
    && rm chrome.deb

# Install Chromedriver
RUN wget -qO- https://chromedriver.storage.googleapis.com/$(curl -sS https://chromedriver.storage.googleapis.com/LATEST_RELEASE)/chromedriver_linux64.zip > chromedriver.zip \
    && unzip chromedriver.zip \
    && mv chromedriver /usr/local/bin/ \
    && rm chromedriver.zip

# Copy the Rust app
COPY --from=builder /usr/src/ambrsosio/target/release/ambrsosio /usr/local/bin/ambrsosio/

# Run Chromedriver in the background and start the app
CMD chromedriver --port=54321 & ambrsosio 
