# Mail Server Setup (Systemd)

```sh
# Install the app
go install github.com/p-tupe/code-examples/go/mail-server

# Pull .env.sample
curl -fsSL -o .env \
    https://raw.githubusercontent.com/p-tupe/code-examples/refs/heads/main/go/mail-server/.env.sample

# Modify the variables
$EDITOR .env

# Pull the service file
curl -fsSL -o go-mail-server.service \
    https://raw.githubusercontent.com/p-tupe/code-examples/refs/heads/main/go/mail-server/go-mail-server.service


# Modify the exec path and environ file var
$EDITOR go-mail-server.service

# Copy service file to systemd dir
sudo cp mail-server.service /etc/systemd/system/

# Enable on login and start immediately
sudo systemctl daemon-reload
sudo systemctl enable --now go-mail-server

# Check logs to ensure it is running
sudo journalctl -u go-mail-server
```

---

To send a simple email:

```sh
curl -H "Authorization: some-auth-string" -d "email body" localhost:8080
```

To send a custom email:

```sh
curl -H "Authorization: some-auth-string" --json '{"to":["x@mail.com"],"subject":"Hello","message":"Hello again!"}' localhost:8080/custom
```
