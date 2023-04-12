FROM node:latest
COPY ./src /app/
COPY ./package.json /app/
WORKDIR /app
EXPOSE 3000
RUN npm install
RUN npm install pm2@latest -g 
CMD ["pm2-runtime", "index.js"]

