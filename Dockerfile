FROM debian:bullseye-slim as runner
LABEL stage=publish

WORKDIR /app

RUN apt-get update
RUN apt-get install -y libcurl4

COPY /target/release/zircon-api .
COPY config config

CMD ["./zircon-api"]
