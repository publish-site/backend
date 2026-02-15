# Website publishment (backend)

This project provides a simple and secure way to deploy a website directly from a Git Repository. It's made to be simple, and easy to configure. For a quick start guide, go [here](https://publish-site.rvid.eu/quick-start).   

This is the backend part of the https://github.com/publish-site/action workflow.
> [!WARNING] 
> MUST RUN behind reverse proxy and use mTLS!!! This is configured in the docker image.

![documentation](https://publish-site.rvid.eu/)
At the moment only docker is supported.

The project is split into two repositories, the action (this repo), and the [backend](https://github.com/publish-site/backend/).

You can access a small demo [here](https://publish-site.rvid.eu/demo/). If you wanna see how the workflow itself is used, go [here](https://github.com/publish-site/docs/actions/workflows/workflow.yml)  
[Documentation](https://publish-site.rvid.eu)

## Deployment

<img width="1550" height="232" alt="image" src="https://github.com/user-attachments/assets/78bf28bf-9078-470c-82ce-12f4073bd4ca" />

You should go to the [quick start guide](https://publish-site.rvid.eu/quick-start). 
```yaml { .copy }
services:
    deploy-server:
        image: ghcr.io/publish-site/backend:latest
        ports:
        - "443:443"
        environment:
            API_URL: "changeme"
            ## Instead of mounting the TLS certificates you can base64 them and do inline certificates.
            #FULLCHAIN:
            # PRIVKEY:

            CLIENT_CA: # The base64 string from PKI script
        volumes:
            - /your/certificate/path/fullchain.pem:/etc/nginx/ssl/fullchain.pem:ro
            - /your/certificate/path/privkey.pem:/etc/nginx/ssl/privkey.pem:ro
            ## You can also mount the CA directly 
            #- /your/certificate/path/ca.pem:/etc/nginx/ssl/ca.pem:ro
            - changeme/web/dir/:/var/www/html # for persistence between container reboots
```

## Limitations

* Non-repudiation (for the server party) and integrity checks has not been implemented yet.

## License

The project is licensed under The Unlicense.
