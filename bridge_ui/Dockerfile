# syntax=docker.io/docker/dockerfile:1.3@sha256:42399d4635eddd7a9b8a24be879d2f9a930d0ed040a61324cfdf59ef1357b3b2

# Derivative of ethereum/Dockerfile, look there for an explanation on how it works.
FROM node:16-alpine@sha256:f21f35732964a96306a84a8c4b5a829f6d3a0c5163237ff4b6b8b34f8d70064b as builder

RUN apk add g++ make 
RUN mkdir -p /app
WORKDIR /app

#COPY package.json package-lock.json ./
COPY package.json .

COPY . .
RUN npm install
RUN npm run build

#RUN --mount=type=cache,uid=1000,gid=1000,target=/home/node/.npm \
#    npm ci
# add app
#COPY . ./


# => Run container
FROM nginx:1.15.2-alpine

# Nginx config
RUN rm -rf /etc/nginx/conf.d
COPY conf /etc/nginx

# Static build
COPY --from=builder /app/build /usr/share/nginx/html/

# Default port exposure
EXPOSE 80

# Copy .env file and shell script to container
WORKDIR /usr/share/nginx/html
COPY ./env.sh .
COPY .env .

# Add bash
RUN apk add --no-cache bash

# Make our shell script executable
RUN chmod +x env.sh

# Start Nginx server
# CMD ["/bin/sh", "-c", "nginx -g \"daemon off;\""]
CMD ["/bin/sh", "-c", "/usr/share/nginx/html/env.sh && sleep 30 && nginx -g \"daemon off;\""]
