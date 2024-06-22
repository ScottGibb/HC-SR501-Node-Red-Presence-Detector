FROM debian:bookworm-slim as builder



FROM builder as image-dev



FROM builder as image-prod
