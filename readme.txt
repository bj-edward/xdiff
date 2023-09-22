For grammar highlight and support xreq: 

cargo add clap --features derive
cargo add serde_yaml
cargo add tokio --features full
cargo add anyhow
cargo add reqwest
cargo add reqwest --features rustls --no-default-features
cargo add similar
cargo add console
cargo add similar --features inline
cargo add serde --features derive
cargo add http_serde
cargo add url --features serde
cargo add serde_json

verify similar features:
mkdir -p examples
touch examples/similar.rs

cargo run --example similar

mkdir -p fixtures

➜  xdiff git:(master) ✗ cargo run --example config

➜  xdiff2 git:(master) ✗ cargo run -- run -p rust -c fixtures/test.yml -e a=100
   Compiling xdiff v0.1.0 (/home/zhijli/Development/Rust/learning4tyrchen/xdiff/xdiff2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/xdiff run -p rust -c fixtures/test.yml -e a=100`
DiffProfile { req1: RequestProfile { method: GET, url: Url { scheme: "https", cannot_be_a_base: fals
e, username: "", password: None, host: Some(Domain("www.rust-lang.org")), port: None, path: "/", que
ry: None, fragment: None }, params: Some(Object {"hello": String("world")}), headers: {"user-agent":
 "Aloha"}, body: None }, req2: RequestProfile { method: GET, url: Url { scheme: "https", cannot_be_a
_base: false, username: "", password: None, host: Some(Domain("www.rust-lang.org")), port: None, pat
h: "/", query: None, fragment: None }, params: Some(Object {}), headers: {}, body: None }, res: Resp
onseProfile { skip_headers: ["set-cookie", "date"], skip_body: [] } }
ExtraArgs { headers: [], query: [("a", "100")], body: [] }

cargo run -- run -p rust -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10
-----------------------------------------------------------------------------------------------------
➜  ver_3 git:(master) ✗ cargo build
➜  ver_3 git:(master) ✗ target/debug/xdiff run -p todo -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10
➜  ver_3 git:(master) ✗ target/debug/xdiff run -p rust -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10

➜  ver_5 git:(master) ✗ cargo add dialoguer
➜  ver_5 git:(master) ✗ cargo build
➜  ver_5 git:(master) ✗ target/debug/xdiff parse
✔ Url1 · https://jsonplaceholder.typicode.com/todos/1
✔ Url2 · https://jsonplaceholder.typicode.com/todos/2
✔ Profile · todo
✔ Select headers to skip · date, content-type, x-powered-by, x-ratelimit-limit  -> use empty button to choose
---
todo:
  req1:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/1
  req2:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/2
  res:
    skip_headers:
    - date
    - content-type
    - x-powered-by
    - x-ratelimit-limit

So, we are able to add the generated contents to fixtures/test.yml.
Exec the following cmd after adding to fixtures/test.yml.

➜  ver_5 git:(master) ✗ target/debug/xdiff run -p todo1 -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10
1   1    | HTTP/2.0 200 OK
2        |-content-length: "83"
3        |-x-ratelimit-remaining: "999"
4        |-x-ratelimit-reset: "1695258658"
    2    |+content-length: "99"
    3    |+x-ratelimit-remaining: "998"
    4    |+x-ratelimit-reset: "1695289661"
5   5    | vary: "Origin, Accept-Encoding"
6   6    | access-control-allow-credentials: "true"
7   7    | cache-control: "max-age=43200"
8   8    | pragma: "no-cache"
9   9    | expires: "-1"
10  10   | x-content-type-options: "nosniff"
11       |-etag: "W/\"53-hfEnumeNh6YirfjyjaujcOPPT+s\""
    11   |+etag: "W/\"63-+s0zIP5ZEQN9hypVJUneLybJ+L0\""
12  12   | via: "1.1 vegur"
13       |-cf-cache-status: "REVALIDATED"
    13   |+cf-cache-status: "MISS"
14  14   | accept-ranges: "bytes"
15       |-report-to: "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report\/v3?s=TLzmn
6e6MNlXARTuARE0JAA7hRW%2FWJsEz863ewi9B7nq4xbUcKXowZVUjaCDv4ythBWNpqIj8WAUMEcCFA%2FOXySR1QJr6StSFgTqP
jxdrZS7Fa39pqOz1JPkmOEHK5dCKXdVrpzBekCpetgvGLE7Pg1AQbdSRM9%2FTRvR\"}],\"group\":\"cf-nel\",\"max_age
\":604800}"
    15   |+report-to: "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report\/v3?s=z2raj
q1tb597kTngRYc1XYTIkg8Iixvn%2BmqatbUvdZ%2FO1fsFCZhAqVNd11k96sZZDHjyePgBBZX4LvXwB8OJDttScTPcAEUaNeiTr
9bya86GdmsnS68n%2B0kreN2LxJXewsbGZhfhGLIw0rN3w9eZXwtTTKSoCs3j3Za8\"}],\"group\":\"cf-nel\",\"max_age
\":604800}"
16  16   | nel: "{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}"
17  17   | server: "cloudflare"
18       |-cf-ray: "80a1632e1cf033f6-NRT"
    18   |+cf-ray: "80a163351e22b012-NRT"
19  19   | alt-svc: "h3=\":443\"; ma=86400"
20  20   | 
21  21   | {
22  22   |   "completed": false,
23       |-  "id": 1,
24       |-  "title": "delectus aut autem",
    23   |+  "id": 2,
    24   |+  "title": "quis ut nam facilis et officia qui",
25  25   |   "userId": 1
26  26   | }
------------------------------------------------------------------------------------------

➜  ver_6 git:(master) ✗ cargo add syntect
➜  ver_6 git:(master) ✗ target/debug/xdiff parse
✔ Url1 · https://jsonplaceholder.typicode.com/todos/1
✔ Url2 · https://jsonplaceholder.typicode.com/todos/2
✔ Profile · todo
✔ Select headers to skip · content-type, content-length
---
todo:
  req1:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/1
  req2:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/2
  res:
    skip_headers:
    - content-type
    - content-length

we are able to see the highlighted output now.

➜  ver_6 git:(master) ✗ cargo check
➜  ver_6 git:(master) ✗ cargo clippy

➜  ver_6 git:(master) ✗ git config --global user.name zhjili
➜  ver_6 git:(master) ✗ git config --global user.email zjiel4cisco@gmail.com
➜  ver_6 git:(master) ✗ git init
Reinitialized existing Git repository in /home/zhijli/Development/Rust/learning4tyrchen/xdiff/ver_6/.git/

➜  ver_6 git:(master) ✗ git status
➜  ver_6 git:(master) ✗ git remote add origin https://github.com/bj-edward/xdiff.git

➜  .ssh ssh-keygen -t ed25519 -C zjiel4cisco@gmail.com
Generating public/private ed25519 key pair.
Enter file in which to save the key (/home/zhijli/.ssh/id_ed25519): github_auth
Enter passphrase (empty for no passphrase):
Enter same passphrase again:
Your identification has been saved in github_auth
Your public key has been saved in github_auth.pub
The key fingerprint is:
SHA256:uNrdNxv/kdRDyzYDu1t5B9a/PFo/WKoeu6389Pzo4b0 zjiel4cisco@gmail.com

➜  ~ ssh-agent
SSH_AUTH_SOCK=/tmp/ssh-XXXXXXJXUo83/agent.10936; export SSH_AUTH_SOCK;
SSH_AGENT_PID=10937; export SSH_AGENT_PID;
echo Agent pid 10937;
➜  ~ ps -ef | grep ssh-agent
zhijli   10937     1  0 10:27 ?        00:00:00 ssh-agent
➜  ~ ssh-add ~/.ssh/github_auth
Could not open a connection to your authentication agent.

➜  ~  ssh-agent bash
zhijli@CXGC-DALIAN ~ $ ssh-add ~/.ssh/github_auth
Identity added: /home/zhijli/.ssh/github_auth (zjiel4cisco@gmail.com)

➜  .ssh bash
zhijli@CXGC-DALIAN ~/.ssh $ ssh -T git@github.com
git@github.com: Permission denied (publickey).
zhijli@CXGC-DALIAN ~/.ssh $ ssh-add -l -E sha256
Could not open a connection to your authentication agent.
zhijli@CXGC-DALIAN ~/.ssh $ nvim config
Host github.com
    HostName github.com
    IdentityFile ~/.ssh/github_auth

zhijli@CXGC-DALIAN ~/.ssh $ ssh -T git@github.com
Hi bj-edward! You've successfully authenticated, but GitHub does not provide shell access.
