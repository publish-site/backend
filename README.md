# Backend

This is the backend part of the https://github.com/publish-site/action workflow.
> [!WARNING] 
> MUST RUN behind reverse proxy and use mTLS!!! This is configured in the docker image.

## Project

[Quick start guide](#quick-start-guide)
[TO-DO](#to-do)
[Credits](#credits)

## Quick-start guide

### Docker compose

```yml
services:
  deploy-server:
    image: backend
    ports: 
      - "443:443"
    environment:
      PORT:    443
      API_URL: "localhost.rvid.eu"
      WEB_URL: "a.rvid.eu" # optional
      ## These will be the certificates for the frontend / user interface
      FULLCHAIN: # Base64-encoded private key here
      PRIVKEY: # Base64-encoded private key here
    ## Alternatively, you can mount the certificates
    #volumes:
     #- /your/certificate/path/fullchain.pem:/etc/nginx/ssl/fullchain.pem:ro
     #- /your/certificate/path/privkey.pem:/etc/nginx/ssl/privkey.pem:ro
```
Base64 encode your (.pem) web certificates and put them in the environment variables. Alternatively you can directly mount them from your system.
This should be the structure of the certificates:
```
-----BEGIN PRIVATE KEY-----
{string}
-----END PRIVATE KEY-----
```

## Configuration

Configuration is located at the [wiki](https://github.com/publish-site/backend/wiki). Are you looking for the [actions workflow](https://github.com/publish-site/action)?

## TO DO:
* Configure docker image
* Add signal handling
* Download file and extract

## Credits

* Thanks to Rust for providing a web server tutorial in their [book](https://doc.rust-lang.org/book/)
* Thanks to stackoverflow