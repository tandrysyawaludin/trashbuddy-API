Majestic API

Trashbuddy is a prototype application which support go-green-living-style. It is can be called on-demand picking up system for trash. Ofcourse, for trash with benefit such as plastic, glass, can and then some.

Command:
export DATABASE_URL=postgres://tandrysyawaludin:Soedijant0@localhost/majesticapp_api
diesel migration redo
rustup update
cargo watch -x 'run'
ROCKET_ENV=stage cargo run

Server:
brew services start nginx
sudo nginx -s stop
sudo nginx

Supervisor:
easy_install supervisor
/usr/local/bin/echo_supervisord_conf > /usr/local/etc/supervisord.conf
brew services start supervisor
sudo unlink /tmp/supervisor.sock
supervisord -c /usr/local/etc/supervisord.conf
supervisorctl -c /usr/local/etc/supervisord.conf

cargo build --release
psql postgres

Code:
use regex::Regex;
let re = Regex::new(r"[^a-zA-Z0-9_.+-@]").unwrap();
let email_after_regex = re.replace_all(&email, "").to_string();
