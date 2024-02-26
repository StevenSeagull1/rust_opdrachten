# Gebruik de officiële Rust-image als basis
FROM rust:latest as builder

# Stel de werkdirectory in binnen de container
WORKDIR /usr/src/app

# Kopieer de inhoud van je huidige directory naar de werkdirectory in de container
COPY . .

# Bouw je Rust-applicatie binnen de Docker-container
RUN cargo build --release

# Stel de tweede fase in voor het verminderen van de imagegrootte
FROM debian:buster-slim

# Installeer en configureer de vereiste pakketten voor de runtime-omgeving
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Kopieer het uitvoerbare bestand van de eerste fase naar de werkdirectory in de container
COPY --from=builder /usr/src/app/target/release/docker /usr/local/bin/docker

# Expose een poort (optioneel)
EXPOSE 8080

# Voer je Rust-applicatie uit
CMD ["docker"]
