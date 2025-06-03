FROM nginx:alpine
COPY nginx.conf /etc/nginx/nginx.conf
COPY site /usr/share/nginx/html
COPY assets /usr/share/nginx/html/assets
