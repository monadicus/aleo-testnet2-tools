FROM rust:1 as build-env
WORKDIR /app
COPY . /app
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build --release --target web

FROM nginx:stable
COPY --from=build-env /app/pkg /usr/share/nginx/html/pkg
COPY --from=build-env /app/index.html /usr/share/nginx/html/index.html
COPY --from=build-env /app/script.mjs /usr/share/nginx/html/script.mjs

# in /etc/nginx/mime.types add "application/javascript                           mjs;"
#               after the line "application/javascript                           js;"
RUN sed -i 's/application\/javascript \+js;/application\/javascript                           js;\n    application\/javascript                           mjs;/g' /etc/nginx/mime.types
