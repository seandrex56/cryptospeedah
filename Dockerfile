FROM rust:latest

# 1. Install dependencies
RUN apt-get update && apt-get install -y python3 python3-pip procps && rm -rf /var/lib/apt/lists/*
RUN pip3 install streamlit --break-system-packages

WORKDIR /app

# 2. Copy files
COPY . .

# 3. Build Rust
RUN cargo build --release

# 4. Rename binary para sa app.py
RUN BIN_NAME=$(grep '^name =' Cargo.toml | sed 's/.*= "//;s/"//') && \
    cp target/release/$BIN_NAME ./btc_hunter && \
    chmod +x ./btc_hunter

# 5. Fixed Port para sa Render
EXPOSE 10000

# 6. Command para pilitin ang Streamlit Dashboard
CMD ["sh", "-c", "streamlit run app.py --server.port $PORT --server.address 0.0.0.0"]
