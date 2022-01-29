FROM rust:1.58.1-slim-buster as builder

# copy files from this directory '.' to '/var/www'
ADD ./plabayo-news-web /var/www
# required for build.rs (ENV)
ADD ./.git /var/www/.git

# set cwd for RUN and CMD
WORKDIR /var/www

# might need to install some other libs to get your build done
RUN apt-get update && apt-get -y install pkg-config libssl-dev

RUN cargo fetch

# build server
RUN cargo build --release

# second stage #
FROM debian:buster-slim

RUN mkdir /var/www
WORKDIR /var/www

# adjust permissions
RUN groupadd --gid 1000 plabayo \
	&& useradd --uid 1000 --gid plabayo --shell /bin/bash --create-home plabayo

RUN chown -R plabayo:plabayo /var/www

COPY --from=builder /var/www/target/release/plabayo-news-web .
USER plabayo

EXPOSE 8888

# run the app
CMD ["./plabayo-news-web"]