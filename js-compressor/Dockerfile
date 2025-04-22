# Stage 1: Build stage
FROM node:16-alpine AS builder

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

# Stage 2: Production stage
FROM  scratch

WORKDIR /usr/src/app

# Copy application files
COPY . .

# Set the entrypoint to our CLI
ENTRYPOINT ["node", "index.js"]

EXPOSE 8000

CMD ["node", "index.js"]