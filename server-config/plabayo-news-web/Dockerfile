FROM rust:1.58.1-slim-buster as builder

# copy files from this directory '.' to '/var/www'
ADD ./plabayo-news-web /var/www

# set cwd for RUN and CMD
WORKDIR /var/www

# might need to install some other libs to get your build done
RUN apt-get update && apt-get -y install pkg-config libssl-dev

RUN cargo fetch

# build server
RUN cargo build --release


################
# second stage #
FROM scratch

COPY --from=builder /var/www/target/release/plabayo-news-web .

# run the app
ENTRYPOINT ["./plabayo-news-web"]
