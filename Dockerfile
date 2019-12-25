FROM alpine:edge AS builder

# Frustratingly I can't use this yet as rocket requires the nightly compiler
# and that doesn't seem to fly in alpine yet :(
RUN apk --no-cache add bash ca-certificates cargo openssl-dev postgresql-dev rust

WORKDIR /usr/src/app

# Do the dependency resolution and pull before we bring in the whole app so we
# can cache these layers during builds
COPY Cargo.toml /usr/src/app/Cargo.toml
COPY Cargo.lock /usr/src/app/Cargo.lock

RUN cargo fetch

COPY . /usr/src/app
RUN cargo build --release --frozen

FROM alpine:edge
RUN apk --no-cache add bash ca-certificates libgcc postgresql-client tini

ENV MINDSHARK_ADDRESS [::]:9292
ENV RUST_LOG info

USER nobody

COPY migrations/ /srv/mindshark_membership/migrations/

WORKDIR /srv/mindshark_membership

COPY --from=builder /usr/src/app/target/release/mindshark_membership /usr/bin/mindshark_membership

ENTRYPOINT ["/sbin/tini", "--"]
