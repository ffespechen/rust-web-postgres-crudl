# --- Etapa 1: Builder ---
FROM rust:1.92-slim AS builder

WORKDIR /app
# Variables para compilar sqlx offline
ENV SQLX_OFFLINE=true

# Instalamos dependencias del sistema para compilar sqlx y otros
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copiamos archivos de configuración
COPY Cargo.toml Cargo.lock .env ./
COPY migrations ./migrations

# Creamos un proyecto vacío para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Ahora copiamos el código real y compilamos
COPY src ./src
COPY templates ./templates
COPY ./.sqlx ./sqlx

# El touch asegura que cargo detecte cambios en main.rs
RUN touch src/main.rs && cargo build --release

# --- Etapa 2: Runtime ---
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copiar binario y recursos
COPY --from=builder /app/target/release/web-postgres-crudl ./web-postgres-crudl
COPY --from=builder /app/templates ./templates
COPY .env .env

# Copiar el script de entrada y darle permisos
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh

# Creamos carpeta para uploads
RUN mkdir -p uploads
COPY uploads/portada_generica.jpg uploads/portada_generica.jpg

EXPOSE 3000

ENTRYPOINT ["./entrypoint.sh"]